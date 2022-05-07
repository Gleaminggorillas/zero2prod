use crate::routes::{health_check, subscribe};

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
// run must be marked as public
// it is not a binary entrypoint, and can be marked as async
// without using pro-macro incantation
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // wrap conn in smart pointer
    let db_pool = Data::new(db_pool);
    // grab connection from surrounding env
    let server = HttpServer::new(move || {
        App::new()
            // add middleware logger
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // grab the pointer clone, attach to appication state
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
