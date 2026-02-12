use actix::prelude::*;
use std::time::{Duration, Instant};

use crate::db::Database;
use crate::game::ai::AiPlayerActor;
use crate::game::session::GameSessionActor;
use crate::game::ws::{OpponentInfo, PlayerWsActor, SendServerMessage, ServerMessage, SetSession};
use crate::models::user::User;

/// Queued player info
struct QueuedPlayer {
    user_id: String,
    username: String,
    elo: i32,
    ranked: bool,
    is_guest: bool,
    addr: Addr<PlayerWsActor>,
    queued_at: Instant,
}

/// Singleton matchmaking actor
pub struct MatchmakingActor {
    queue: Vec<QueuedPlayer>,
    db: Database,
}

impl MatchmakingActor {
    pub fn new(db: Database) -> Self {
        Self {
            queue: Vec::new(),
            db,
        }
    }
}

impl Actor for MatchmakingActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("MatchmakingActor started");
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinQueue {
    pub user_id: String,
    pub username: String,
    pub elo: i32,
    pub ranked: bool,
    pub is_guest: bool,
    pub addr: Addr<PlayerWsActor>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveQueue {
    pub user_id: String,
}

impl Handler<JoinQueue> for MatchmakingActor {
    type Result = ();

    fn handle(&mut self, msg: JoinQueue, ctx: &mut Self::Context) {
        // Don't allow duplicate queue entries
        if self.queue.iter().any(|p| p.user_id == msg.user_id) {
            msg.addr.do_send(SendServerMessage(ServerMessage::Error {
                message: "Already in queue".into(),
            }));
            return;
        }

        // Notify player they're queued
        msg.addr.do_send(SendServerMessage(ServerMessage::Queued));

        let user_id = msg.user_id.clone();

        self.queue.push(QueuedPlayer {
            user_id: msg.user_id,
            username: msg.username,
            elo: msg.elo,
            ranked: msg.ranked,
            is_guest: msg.is_guest,
            addr: msg.addr,
            queued_at: Instant::now(),
        });

        self.try_match();

        // Schedule AI matchmaking check after 3 seconds
        ctx.run_later(Duration::from_secs(3), move |act, ctx| {
            act.check_ai_matchmaking(ctx, &user_id);
        });
    }
}

impl Handler<LeaveQueue> for MatchmakingActor {
    type Result = ();

    fn handle(&mut self, msg: LeaveQueue, _ctx: &mut Self::Context) {
        self.queue.retain(|p| p.user_id != msg.user_id);
    }
}

impl MatchmakingActor {
    fn try_match(&mut self) {
        if self.queue.len() < 2 {
            return;
        }

        // Simple FIFO matching for now
        let p2 = self.queue.remove(1);
        let p1 = self.queue.remove(0);

        let is_ranked = p1.ranked && p2.ranked;

        // Notify both players
        p1.addr
            .do_send(SendServerMessage(ServerMessage::MatchFound {
                session_id: String::new(), // Will be set by session
                opponent: OpponentInfo {
                    username: p2.username.clone(),
                    elo: p2.elo,
                },
            }));

        p2.addr
            .do_send(SendServerMessage(ServerMessage::MatchFound {
                session_id: String::new(),
                opponent: OpponentInfo {
                    username: p1.username.clone(),
                    elo: p1.elo,
                },
            }));

        // Create game session
        let session = GameSessionActor::new(
            p1.user_id.clone(),
            p1.username.clone(),
            p1.elo,
            p1.is_guest,
            false, // p1 is not AI
            p1.addr.clone().recipient(),
            p2.user_id.clone(),
            p2.username.clone(),
            p2.elo,
            p2.is_guest,
            false, // p2 is not AI
            p2.addr.clone().recipient(),
            is_ranked,
            self.db.clone(),
        );

        let session_addr = session.start();

        // Set session on both player actors
        p1.addr.do_send(SetSession(session_addr.clone()));
        p2.addr.do_send(SetSession(session_addr));
    }

    fn check_ai_matchmaking(&mut self, ctx: &mut Context<Self>, target_user_id: &str) {
        // Find player in queue (may have already matched or left)
        let player_idx = self.queue.iter().position(|p| p.user_id == target_user_id);

        if let Some(idx) = player_idx {
            let player = self.queue.remove(idx);
            let now = Instant::now();

            // Verify player waited >= 3 seconds
            if now.duration_since(player.queued_at) >= Duration::from_secs(3) {
                log::info!("Timeout: matching {} with AI", player.username);
                self.match_with_ai(player, ctx);
            }
            // If player matched with human already, they're no longer in queue
            // so this branch won't execute (player not found)
        }
    }

    fn match_with_ai(&mut self, player: QueuedPlayer, ctx: &mut Context<Self>) {
        let db = self.db.clone();
        let player_user_id = player.user_id.clone();
        let player_username = player.username.clone();
        let player_elo = player.elo;
        let player_is_guest = player.is_guest;
        let player_ranked = player.ranked;
        let player_addr = player.addr.clone();

        // Async fetch random AI user
        let fut = async move {
            User::get_random_ai(&db).await.ok().map(|ai| (ai, db))
        };

        ctx.spawn(
            fut.into_actor(self).map(move |result, _act, _ctx| {
                if let Some((ai_user, db)) = result {
                    // Notify player
                    player_addr.do_send(SendServerMessage(ServerMessage::MatchFound {
                        session_id: String::new(),
                        opponent: OpponentInfo {
                            username: ai_user.username.clone(),
                            elo: ai_user.elo,
                        },
                    }));

                    // Create AI actor
                    let ai_actor = AiPlayerActor::new(ai_user.id.clone()).start();

                    // Notify AI (for consistency)
                    ai_actor.do_send(SendServerMessage(ServerMessage::MatchFound {
                        session_id: String::new(),
                        opponent: OpponentInfo {
                            username: player_username.clone(),
                            elo: player_elo,
                        },
                    }));

                    // Create game session
                    let session = GameSessionActor::new(
                        player_user_id,
                        player_username,
                        player_elo,
                        player_is_guest,
                        false, // player is not AI
                        player_addr.clone().recipient(),
                        ai_user.id.clone(),
                        ai_user.username.clone(),
                        ai_user.elo,
                        false, // AI is not guest
                        true,  // AI is AI
                        ai_actor.clone().recipient(),
                        player_ranked,
                        db,
                    );

                    let session_addr = session.start();
                    player_addr.do_send(SetSession(session_addr.clone()));
                    ai_actor.do_send(SetSession(session_addr));
                } else {
                    player_addr.do_send(SendServerMessage(ServerMessage::Error {
                        message: "Failed to find opponent".into(),
                    }));
                }
            })
        );
    }
}
