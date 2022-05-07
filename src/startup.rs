use crate::routes::{health_check, subscribe};

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;
// run must be marked as public
// it is not a binary entrypoint, and can be marked as async
// without using pro-macro incantation
pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    // wrap conn in smart pointer
    let connection = web::Data::new(connection);
    // grab connection from surrounding env
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // grab the pointer clone, attach to appication state
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
