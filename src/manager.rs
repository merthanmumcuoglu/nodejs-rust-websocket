use actix::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::models::ServerMessage;

pub struct RoomManager {
    pub sessions: HashMap<String, Recipient<ServerMessage>>,
    pub rooms: HashMap<String, HashSet<String>>,
}

impl RoomManager {
    pub fn new() -> Self {
        RoomManager {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    pub fn send_message(&self, room: &str, message: ServerMessage, skip_id: Option<&str>) {
        if let Some(users) = self.rooms.get(room) {
            for user_id in users {
                if let Some(skip) = skip_id {
                    if user_id == skip {
                        continue;
                    }
                }
                if let Some(addr) = self.sessions.get(user_id) {
                    let _ = addr.do_send(message.clone());
                }
            }
        }

    }

}

impl Actor for RoomManager {
    type Context = actix::Context<Self>;
}