use actix::prelude::*;

use crate::db::Database;
use crate::game::session::GameSessionActor;
use crate::game::ws::{OpponentInfo, PlayerWsActor, SendServerMessage, ServerMessage, SetSession};

/// Queued player info
struct QueuedPlayer {
    user_id: String,
    username: String,
    elo: i32,
    ranked: bool,
    is_guest: bool,
    addr: Addr<PlayerWsActor>,
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

    fn handle(&mut self, msg: JoinQueue, _ctx: &mut Self::Context) {
        // Don't allow duplicate queue entries
        if self.queue.iter().any(|p| p.user_id == msg.user_id) {
            msg.addr.do_send(SendServerMessage(ServerMessage::Error {
                message: "Already in queue".into(),
            }));
            return;
        }

        // Notify player they're queued
        msg.addr.do_send(SendServerMessage(ServerMessage::Queued));

        self.queue.push(QueuedPlayer {
            user_id: msg.user_id,
            username: msg.username,
            elo: msg.elo,
            ranked: msg.ranked,
            is_guest: msg.is_guest,
            addr: msg.addr,
        });

        self.try_match();
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
            p1.addr.clone(),
            p2.user_id.clone(),
            p2.username.clone(),
            p2.elo,
            p2.is_guest,
            p2.addr.clone(),
            is_ranked,
            self.db.clone(),
        );

        let session_addr = session.start();

        // Set session on both player actors
        p1.addr.do_send(SetSession(session_addr.clone()));
        p2.addr.do_send(SetSession(session_addr));
    }
}
