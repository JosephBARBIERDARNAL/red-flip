use actix::prelude::*;
use rand::Rng;
use std::time::Duration;

use crate::game::session::{GameSessionActor, PlayerChoice};
use crate::game::ws::{SendServerMessage, ServerMessage, SetSession};

/// AI player actor that responds to game messages and makes random choices
pub struct AiPlayerActor {
    user_id: String,
    session: Option<Addr<GameSessionActor>>,
    auto_play_enabled: bool,
}

impl AiPlayerActor {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            session: None,
            auto_play_enabled: false,
        }
    }

    fn make_random_choice(&self, _ctx: &mut Context<Self>) {
        if let Some(ref session) = self.session {
            let choices = ["rock", "paper", "scissors"];
            let choice = choices[rand::thread_rng().gen_range(0..3)];

            log::info!("AI player {} chose: {}", self.user_id, choice);

            session.do_send(PlayerChoice {
                user_id: self.user_id.clone(),
                choice: choice.to_string(),
            });
        }
    }
}

impl Actor for AiPlayerActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("AiPlayerActor started for {}", self.user_id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("AiPlayerActor stopped for {}", self.user_id);
    }
}

impl Handler<SendServerMessage> for AiPlayerActor {
    type Result = ();

    fn handle(&mut self, msg: SendServerMessage, ctx: &mut Self::Context) {
        match msg.0 {
            ServerMessage::RoundStart { .. } => {
                // Schedule random choice after 3 seconds
                if self.auto_play_enabled {
                    ctx.run_later(Duration::from_secs(3), |act, ctx| {
                        act.make_random_choice(ctx);
                    });
                }
            }
            ServerMessage::MatchFound { .. } => {
                // Enable auto-play when match is found
                self.auto_play_enabled = true;
            }
            _ => {
                // Ignore other messages (RoundResult, MatchComplete, etc.)
            }
        }
    }
}

impl Handler<SetSession> for AiPlayerActor {
    type Result = ();

    fn handle(&mut self, msg: SetSession, ctx: &mut Self::Context) {
        self.session = Some(msg.0);
        // Make first choice immediately when session is set
        self.make_random_choice(ctx);
    }
}
