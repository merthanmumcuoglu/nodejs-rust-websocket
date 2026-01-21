use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Instant, Duration};
use actix_web::web;
use uuid::Uuid;
use crate::manager::RoomManager;
use web::Data;


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);

pub struct WsSession {
    id: String,
    hb: Instant,
    room_manager: web::Data<actix::Addr<RoomManager>>,
}

impl WsSession {
    pub fn new(room_manager: web::Data<actix::Addr<RoomManager>>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            hb: Instant::now(),
            room_manager,
        }
    }

    pub fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("WebSocket timeout, disconnecting!");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        actix::Running::Stop
    }
}



impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(_)) => {
                println!("Unsupported binary message");
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}