use crate::orderbook::Orderbook;
use actix_web::web::{Data, Json};
use actix_web::{App, HttpServer};
use actix::Actor;
use engine::MatchingEngine;
use routes::{cancel_order_route, create_order_route, get_depth_route, get_order_route};

pub mod engine;
pub mod input;
pub mod orderbook;
pub mod output;
pub mod routes;

#[actix_web::main]

async fn main() -> Result<(), std::io::Error> {
    let engine = MatchingEngine::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(engine.clone()))
            .app_data(Data::new(Orderbook::new()))
            // .service(create_order)
            // .service(delete_order)
            // .service(get_depth)
            .service(create_order_route)
            .service(get_order_route)
            .service(cancel_order_route)
            .service(get_depth_route)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
