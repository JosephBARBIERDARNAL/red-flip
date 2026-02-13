use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use crate::game::matchmaking::{JoinQueue, LeaveQueue, MatchmakingActor};
use crate::game::session::{GameSessionActor, PlayerChoice, PlayerDisconnected};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Messages sent from client to server
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "join_queue")]
    JoinQueue { ranked: Option<bool> },
    #[serde(rename = "leave_queue")]
    LeaveQueue,
    #[serde(rename = "choice")]
    Choice { choice: String },
}

/// Messages sent from server to client
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "match_found")]
    MatchFound {
        session_id: String,
        opponent: OpponentInfo,
    },
    #[serde(rename = "round_start")]
    RoundStart { round: i32, timeout_secs: u64 },
    #[serde(rename = "opponent_chose")]
    OpponentChose,
    #[serde(rename = "round_result")]
    RoundResult {
        round: i32,
        your_choice: String,
        opponent_choice: String,
        winner: String, // "you", "opponent", "draw"
        your_score: i32,
        opponent_score: i32,
    },
    #[serde(rename = "match_complete")]
    MatchComplete {
        result: String, // "win", "loss", "draw"
        your_score: i32,
        opponent_score: i32,
        elo_change: Option<i32>,
        new_elo: Option<i32>,
    },
    #[serde(rename = "opponent_disconnected")]
    OpponentDisconnected,
    #[serde(rename = "error")]
    Error { message: String },
}

#[derive(Debug, Serialize, Clone)]
pub struct OpponentInfo {
    pub username: String,
    pub elo: i32,
}

/// Message to send ServerMessage to a PlayerWsActor
#[derive(Message)]
#[rtype(result = "()")]
pub struct SendServerMessage(pub ServerMessage);

/// Per-connection WebSocket actor
pub struct PlayerWsActor {
    pub user_id: String,
    pub username: String,
    pub elo: i32,
    pub is_guest: bool,
    pub hb: Instant,
    pub matchmaking: Addr<MatchmakingActor>,
    pub session: Option<Addr<GameSessionActor>>,
}

impl PlayerWsActor {
    pub fn new(
        user_id: String,
        username: String,
        elo: i32,
        is_guest: bool,
        matchmaking: Addr<MatchmakingActor>,
    ) -> Self {
        Self {
            user_id,
            username,
            elo,
            is_guest,
            hb: Instant::now(),
            matchmaking,
            session: None,
        }
    }

    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                log::warn!(
                    "WebSocket heartbeat failed for user {}, disconnecting",
                    act.user_id
                );
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

    fn send_message(&self, msg: &ServerMessage, ctx: &mut ws::WebsocketContext<Self>) {
        if let Ok(json) = serde_json::to_string(msg) {
            ctx.text(json);
        }
    }
}

impl Actor for PlayerWsActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
        log::info!("PlayerWsActor started for user {}", self.user_id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("PlayerWsActor stopped for user {}", self.user_id);
        // Leave queue if in queue
        self.matchmaking.do_send(LeaveQueue {
            user_id: self.user_id.clone(),
        });
        // Notify game session if in a game
        if let Some(ref session) = self.session {
            session.do_send(PlayerDisconnected {
                user_id: self.user_id.clone(),
            });
        }
    }
}

impl Handler<SendServerMessage> for PlayerWsActor {
    type Result = ();

    fn handle(&mut self, msg: SendServerMessage, ctx: &mut Self::Context) {
        self.send_message(&msg.0, ctx);
    }
}

/// Message to set the game session on the player actor
#[derive(Message)]
#[rtype(result = "()")]
pub struct SetSession(pub Addr<GameSessionActor>);

impl Handler<SetSession> for PlayerWsActor {
    type Result = ();

    fn handle(&mut self, msg: SetSession, _ctx: &mut Self::Context) {
        self.session = Some(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PlayerWsActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => {
                ctx.stop();
                return;
            }
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let parsed: Result<ClientMessage, _> = serde_json::from_str(&text);
                match parsed {
                    Ok(ClientMessage::JoinQueue { ranked }) => {
                        // Guest users can only play unranked
                        let ranked = if self.is_guest {
                            false
                        } else {
                            ranked.unwrap_or(true)
                        };
                        self.matchmaking.do_send(JoinQueue {
                            user_id: self.user_id.clone(),
                            username: self.username.clone(),
                            elo: self.elo,
                            ranked,
                            is_guest: self.is_guest,
                            addr: ctx.address(),
                        });
                    }
                    Ok(ClientMessage::LeaveQueue) => {
                        self.matchmaking.do_send(LeaveQueue {
                            user_id: self.user_id.clone(),
                        });
                    }
                    Ok(ClientMessage::Choice { choice }) => {
                        if let Some(ref session) = self.session {
                            session.do_send(PlayerChoice {
                                user_id: self.user_id.clone(),
                                choice,
                            });
                        } else {
                            self.send_message(
                                &ServerMessage::Error {
                                    message: "Not in a game".into(),
                                },
                                ctx,
                            );
                        }
                    }
                    Err(_) => {
                        self.send_message(
                            &ServerMessage::Error {
                                message: "Invalid message format".into(),
                            },
                            ctx,
                        );
                    }
                }
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_message_deserializes_supported_types() {
        let join: ClientMessage = serde_json::from_str(r#"{"type":"join_queue","ranked":true}"#)
            .expect("join_queue should deserialize");
        assert!(matches!(
            join,
            ClientMessage::JoinQueue { ranked: Some(true) }
        ));

        let leave: ClientMessage =
            serde_json::from_str(r#"{"type":"leave_queue"}"#).expect("leave_queue should parse");
        assert!(matches!(leave, ClientMessage::LeaveQueue));

        let choice: ClientMessage = serde_json::from_str(r#"{"type":"choice","choice":"rock"}"#)
            .expect("choice should deserialize");
        assert!(matches!(
            choice,
            ClientMessage::Choice { choice } if choice == "rock"
        ));
    }

    #[test]
    fn server_message_serializes_with_expected_tag() {
        let msg = ServerMessage::RoundResult {
            round: 2,
            your_choice: "paper".into(),
            opponent_choice: "rock".into(),
            winner: "you".into(),
            your_score: 2,
            opponent_score: 1,
        };

        let json = serde_json::to_value(msg).expect("server message should serialize");

        assert_eq!(json["type"], "round_result");
        assert_eq!(json["round"], 2);
        assert_eq!(json["winner"], "you");
    }
}
