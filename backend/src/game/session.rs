use actix::prelude::*;
use std::time::Duration;

use crate::db::Database;
use crate::game::elo::calculate_elo;
use crate::game::ws::{PlayerWsActor, SendServerMessage, ServerMessage};
use crate::models::elo_history::EloHistory;
use crate::models::match_record::{MatchRecord, Round};
use crate::models::user::User;

const ROUND_TIMEOUT_SECS: u64 = 15;

/// Per-match game session actor
pub struct GameSessionActor {
    p1_id: String,
    p1_username: String,
    p1_elo: i32,
    p1_addr: Addr<PlayerWsActor>,
    p1_choice: Option<String>,
    p2_id: String,
    p2_username: String,
    p2_elo: i32,
    p2_addr: Addr<PlayerWsActor>,
    p2_choice: Option<String>,
    p1_score: i32,
    p2_score: i32,
    current_round: i32,
    rounds: Vec<Round>,
    is_ranked: bool,
    db: Database,
    finished: bool,
}

impl GameSessionActor {
    pub fn new(
        p1_id: String,
        p1_username: String,
        p1_elo: i32,
        p1_addr: Addr<PlayerWsActor>,
        p2_id: String,
        p2_username: String,
        p2_elo: i32,
        p2_addr: Addr<PlayerWsActor>,
        is_ranked: bool,
        db: Database,
    ) -> Self {
        Self {
            p1_id,
            p1_username,
            p1_elo,
            p1_addr,
            p1_choice: None,
            p2_id,
            p2_username,
            p2_elo,
            p2_addr,
            p2_choice: None,
            p1_score: 0,
            p2_score: 0,
            current_round: 1,
            rounds: Vec::new(),
            is_ranked,
            db,
            finished: false,
        }
    }

    fn start_round(&mut self, ctx: &mut Context<Self>) {
        self.p1_choice = None;
        self.p2_choice = None;

        let msg = ServerMessage::RoundStart {
            round: self.current_round,
            timeout_secs: ROUND_TIMEOUT_SECS,
        };

        self.p1_addr.do_send(SendServerMessage(msg.clone()));
        self.p2_addr.do_send(SendServerMessage(msg));

        // Round timeout
        ctx.run_later(Duration::from_secs(ROUND_TIMEOUT_SECS), |act, ctx| {
            act.resolve_round(ctx);
        });
    }

    fn resolve_round(&mut self, ctx: &mut Context<Self>) {
        if self.finished {
            return;
        }

        let p1_choice = self.p1_choice.take();
        let p2_choice = self.p2_choice.take();

        // If both already resolved (both chose), skip
        if self.rounds.len() >= self.current_round as usize {
            return;
        }

        let winner = determine_winner(p1_choice.as_deref(), p2_choice.as_deref());

        match winner {
            RoundWinner::Player1 => self.p1_score += 1,
            RoundWinner::Player2 => self.p2_score += 1,
            RoundWinner::Draw => {}
        }

        let round = Round {
            round_number: self.current_round,
            player1_choice: p1_choice.clone(),
            player2_choice: p2_choice.clone(),
            winner: match winner {
                RoundWinner::Player1 => Some(self.p1_id.clone()),
                RoundWinner::Player2 => Some(self.p2_id.clone()),
                RoundWinner::Draw => Some("draw".into()),
            },
        };
        self.rounds.push(round);

        let p1_choice_str = p1_choice.unwrap_or_else(|| "none".into());
        let p2_choice_str = p2_choice.unwrap_or_else(|| "none".into());

        // Send round results
        self.p1_addr
            .do_send(SendServerMessage(ServerMessage::RoundResult {
                round: self.current_round,
                your_choice: p1_choice_str.clone(),
                opponent_choice: p2_choice_str.clone(),
                winner: match winner {
                    RoundWinner::Player1 => "you".into(),
                    RoundWinner::Player2 => "opponent".into(),
                    RoundWinner::Draw => "draw".into(),
                },
                your_score: self.p1_score,
                opponent_score: self.p2_score,
            }));

        self.p2_addr
            .do_send(SendServerMessage(ServerMessage::RoundResult {
                round: self.current_round,
                your_choice: p2_choice_str,
                opponent_choice: p1_choice_str,
                winner: match winner {
                    RoundWinner::Player1 => "opponent".into(),
                    RoundWinner::Player2 => "you".into(),
                    RoundWinner::Draw => "draw".into(),
                },
                your_score: self.p2_score,
                opponent_score: self.p1_score,
            }));

        // Check if match is over (Bo3: first to 2 wins, max 5 rounds)
        if self.p1_score >= 2 || self.p2_score >= 2 || self.current_round >= 5 {
            self.finish_match(ctx);
        } else {
            self.current_round += 1;
            self.start_round(ctx);
        }
    }

    fn finish_match(&mut self, ctx: &mut Context<Self>) {
        self.finished = true;

        let (winner_id, p1_outcome, p2_outcome) = if self.p1_score > self.p2_score {
            (Some(self.p1_id.clone()), "win", "loss")
        } else if self.p2_score > self.p1_score {
            (Some(self.p2_id.clone()), "loss", "win")
        } else {
            (None, "draw", "draw")
        };

        let outcome = if self.p1_score > self.p2_score {
            1.0
        } else if self.p2_score > self.p1_score {
            0.0
        } else {
            0.5
        };

        // Calculate and persist Elo + match
        let db = self.db.clone();
        let p1_id = self.p1_id.clone();
        let p2_id = self.p2_id.clone();
        let p1_elo = self.p1_elo;
        let p2_elo = self.p2_elo;
        let p1_score = self.p1_score;
        let p2_score = self.p2_score;
        let is_ranked = self.is_ranked;
        let rounds_json = serde_json::to_string(&self.rounds).unwrap_or_else(|_| "[]".into());
        let p1_addr = self.p1_addr.clone();
        let p2_addr = self.p2_addr.clone();
        let winner_clone = winner_id.clone();

        let p1_outcome = p1_outcome.to_string();
        let p2_outcome = p2_outcome.to_string();

        actix::spawn(async move {
            let (new_p1_elo, new_p2_elo) = if is_ranked {
                // Fetch current game counts
                let p1_games = User::find_by_id(&db, &p1_id)
                    .await
                    .ok()
                    .flatten()
                    .map(|u| u.total_games)
                    .unwrap_or(0);
                let p2_games = User::find_by_id(&db, &p2_id)
                    .await
                    .ok()
                    .flatten()
                    .map(|u| u.total_games)
                    .unwrap_or(0);

                calculate_elo(p1_elo, p1_games, p2_elo, p2_games, outcome)
            } else {
                (p1_elo, p2_elo)
            };

            // Create match record
            if let Ok(m) = MatchRecord::create(&db, &p1_id, &p2_id, is_ranked, p1_elo, p2_elo).await
            {
                let _ = MatchRecord::finish(
                    &db,
                    &m.id,
                    winner_clone.as_deref(),
                    p1_score,
                    p2_score,
                    &rounds_json,
                    new_p1_elo,
                    new_p2_elo,
                    "completed",
                )
                .await;

                if is_ranked {
                    let _ = User::update_elo(&db, &p1_id, new_p1_elo).await;
                    let _ = User::update_elo(&db, &p2_id, new_p2_elo).await;
                    let _ = EloHistory::create(&db, &p1_id, &m.id, p1_elo, new_p1_elo).await;
                    let _ = EloHistory::create(&db, &p2_id, &m.id, p2_elo, new_p2_elo).await;
                }

                // Update win/loss/draw stats
                let p1_won = if p1_score > p2_score {
                    Some(true)
                } else if p1_score < p2_score {
                    Some(false)
                } else {
                    None
                };
                let _ = User::increment_stats(&db, &p1_id, p1_won).await;
                let _ = User::increment_stats(&db, &p2_id, p1_won.map(|w| !w)).await;
            }

            let p1_elo_change = if is_ranked {
                Some(new_p1_elo - p1_elo)
            } else {
                None
            };
            let p2_elo_change = if is_ranked {
                Some(new_p2_elo - p2_elo)
            } else {
                None
            };

            p1_addr.do_send(SendServerMessage(ServerMessage::MatchComplete {
                result: p1_outcome,
                your_score: p1_score,
                opponent_score: p2_score,
                elo_change: p1_elo_change,
                new_elo: if is_ranked { Some(new_p1_elo) } else { None },
            }));

            p2_addr.do_send(SendServerMessage(ServerMessage::MatchComplete {
                result: p2_outcome,
                your_score: p2_score,
                opponent_score: p1_score,
                elo_change: p2_elo_change,
                new_elo: if is_ranked { Some(new_p2_elo) } else { None },
            }));
        });

        ctx.stop();
    }

    fn forfeit(&mut self, disconnected_user_id: &str, ctx: &mut Context<Self>) {
        if self.finished {
            return;
        }
        self.finished = true;

        let (winner_addr, loser_id) = if disconnected_user_id == self.p1_id {
            (&self.p2_addr, &self.p1_id)
        } else {
            (&self.p1_addr, &self.p2_id)
        };

        winner_addr.do_send(SendServerMessage(ServerMessage::OpponentDisconnected));

        // Record as forfeit (loser gets full loss Elo penalty)
        let db = self.db.clone();
        let p1_id = self.p1_id.clone();
        let p2_id = self.p2_id.clone();
        let p1_elo = self.p1_elo;
        let p2_elo = self.p2_elo;
        let is_ranked = self.is_ranked;
        let loser_is_p1 = disconnected_user_id == self.p1_id;
        let rounds_json = serde_json::to_string(&self.rounds).unwrap_or_else(|_| "[]".into());
        let p1_addr = self.p1_addr.clone();
        let p2_addr = self.p2_addr.clone();

        let _loser_id = loser_id.clone();

        actix::spawn(async move {
            let outcome = if loser_is_p1 { 0.0 } else { 1.0 };

            let (new_p1_elo, new_p2_elo) = if is_ranked {
                let p1_games = User::find_by_id(&db, &p1_id)
                    .await
                    .ok()
                    .flatten()
                    .map(|u| u.total_games)
                    .unwrap_or(0);
                let p2_games = User::find_by_id(&db, &p2_id)
                    .await
                    .ok()
                    .flatten()
                    .map(|u| u.total_games)
                    .unwrap_or(0);
                calculate_elo(p1_elo, p1_games, p2_elo, p2_games, outcome)
            } else {
                (p1_elo, p2_elo)
            };

            let winner_id = if loser_is_p1 { &p2_id } else { &p1_id };
            let (p1_score, p2_score) = if loser_is_p1 { (0, 2) } else { (2, 0) };

            if let Ok(m) = MatchRecord::create(&db, &p1_id, &p2_id, is_ranked, p1_elo, p2_elo).await
            {
                let _ = MatchRecord::finish(
                    &db,
                    &m.id,
                    Some(winner_id),
                    p1_score,
                    p2_score,
                    &rounds_json,
                    new_p1_elo,
                    new_p2_elo,
                    "forfeit",
                )
                .await;

                if is_ranked {
                    let _ = User::update_elo(&db, &p1_id, new_p1_elo).await;
                    let _ = User::update_elo(&db, &p2_id, new_p2_elo).await;
                    let _ = EloHistory::create(&db, &p1_id, &m.id, p1_elo, new_p1_elo).await;
                    let _ = EloHistory::create(&db, &p2_id, &m.id, p2_elo, new_p2_elo).await;
                }

                let p1_won = !loser_is_p1;
                let _ = User::increment_stats(&db, &p1_id, Some(p1_won)).await;
                let _ = User::increment_stats(&db, &p2_id, Some(!p1_won)).await;
            }

            // Notify winner
            let winner_addr = if loser_is_p1 { &p2_addr } else { &p1_addr };
            let (winner_score, loser_score) = if loser_is_p1 {
                (p2_score, p1_score)
            } else {
                (p1_score, p2_score)
            };
            let winner_new_elo = if loser_is_p1 { new_p2_elo } else { new_p1_elo };
            let winner_old_elo = if loser_is_p1 { p2_elo } else { p1_elo };

            winner_addr.do_send(SendServerMessage(ServerMessage::MatchComplete {
                result: "win".into(),
                your_score: winner_score,
                opponent_score: loser_score,
                elo_change: if is_ranked {
                    Some(winner_new_elo - winner_old_elo)
                } else {
                    None
                },
                new_elo: if is_ranked {
                    Some(winner_new_elo)
                } else {
                    None
                },
            }));
        });

        ctx.stop();
    }
}

impl Actor for GameSessionActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!(
            "GameSession started: {} vs {}",
            self.p1_username,
            self.p2_username
        );
        self.start_round(ctx);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerChoice {
    pub user_id: String,
    pub choice: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerDisconnected {
    pub user_id: String,
}

impl Handler<PlayerChoice> for GameSessionActor {
    type Result = ();

    fn handle(&mut self, msg: PlayerChoice, ctx: &mut Self::Context) {
        if self.finished {
            return;
        }

        let valid_choices = ["rock", "paper", "scissors"];
        if !valid_choices.contains(&msg.choice.as_str()) {
            return;
        }

        if msg.user_id == self.p1_id && self.p1_choice.is_none() {
            self.p1_choice = Some(msg.choice);
            // Notify opponent that this player has chosen
            self.p2_addr
                .do_send(SendServerMessage(ServerMessage::OpponentChose));
        } else if msg.user_id == self.p2_id && self.p2_choice.is_none() {
            self.p2_choice = Some(msg.choice);
            self.p1_addr
                .do_send(SendServerMessage(ServerMessage::OpponentChose));
        }

        // Both chose -> resolve immediately
        if self.p1_choice.is_some() && self.p2_choice.is_some() {
            self.resolve_round(ctx);
        }
    }
}

impl Handler<PlayerDisconnected> for GameSessionActor {
    type Result = ();

    fn handle(&mut self, msg: PlayerDisconnected, ctx: &mut Self::Context) {
        self.forfeit(&msg.user_id, ctx);
    }
}

enum RoundWinner {
    Player1,
    Player2,
    Draw,
}

fn determine_winner(p1: Option<&str>, p2: Option<&str>) -> RoundWinner {
    match (p1, p2) {
        (None, None) => RoundWinner::Draw,
        (Some(_), None) => RoundWinner::Player1,
        (None, Some(_)) => RoundWinner::Player2,
        (Some(a), Some(b)) => {
            if a == b {
                RoundWinner::Draw
            } else if (a == "rock" && b == "scissors")
                || (a == "paper" && b == "rock")
                || (a == "scissors" && b == "paper")
            {
                RoundWinner::Player1
            } else {
                RoundWinner::Player2
            }
        }
    }
}
