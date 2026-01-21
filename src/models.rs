use serde::{Deserialize, Serialize};
use actix::Message;


#[derive(Debug, Clone, Serialize, Message)]
#[rtype(result = "()")]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "joined")]
    Joined { room: String, user_id: String },
    #[serde(rename = "left")]
    Left { room: String, user_id: String },
    #[serde(rename = "message")]
    Message { room: String, user_id: String, message: String },
    #[serde(rename = "user_joined")]
    UserJoined { room: String, user_id: String },
    #[serde(rename = "user_left")]
    UserLeft { room: String, user_id: String },
    #[serde(rename = "rooms")]
    Rooms { rooms: Vec<String> },
    #[serde(rename = "pong")]
    Pong,
    #[serde(rename = "error")]
    Error { message: String },
    #[serde(rename = "users")]
    Users { room: String, users: Vec<String> },
}