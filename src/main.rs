mod models;
mod manager;
mod session;

use actix::{Actor};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use manager::RoomManager;
use session::WsSession;



async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    room_manager: web::Data<actix::Addr<RoomManager>>,
) -> Result<HttpResponse, Error> {
    ws::start(WsSession::new(room_manager), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let room_manager = RoomManager::new().start();
    let room_manager_data = web::Data::new(room_manager);

    println!("WebSocket starting on ws://");


    HttpServer::new(move || {
        App::new()
            .app_data(room_manager_data.clone())
            .route("/ws", web::get().to(ws_index))
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}