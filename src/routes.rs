use crate::engine::{CancelOrder, CreateOrder, GetDepth, GetOrder, MatchingEngine};
use crate::input::Side;
use crate::input::{CreateOrderInput, DeleteOrderInput, Depth};
use crate::orderbook::Orderbook;
use crate::output::{CreateOrderRequest, CreateOrderResponse, DeleteOrderResponse, OrderResponse};
use actix::Addr;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::{
    delete, get, post,
    web::{Data, Json},
    Responder,
};
use rust_decimal::Decimal;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// #[post("/order")]
// pub async fn create_order(
//     body: Json<CreateOrderInput>,
//     orderbook: Data<&Arc<Mutex<Orderbook>>>,
// ) -> impl Responder {
//     let price = body.0.price;
//     let quantity = body.0.quantity;
//     let user_id = body.0.user_id;
//     let side = body.0.side;

//     let mut orderbook = orderbook.lock().unwrap();
//     orderbook.create_order(price, quantity, user_id, side);

//     return HttpResponse::Ok().json(CreateOrderResponse {
//         order_id: String::from("Response"),
//     });

//     // println!("{:?}", body);
//     // "Order created"
// }
#[post("/order")]

pub async fn create_order_route(
    req: web::Json<CreateOrderRequest>,
    engine_addr: web::Data<Addr<MatchingEngine>>,
) -> impl Responder {
    let order_data = req.into_inner();

    let price = match Decimal::from_str(&order_data.price) {
        Ok(p) => p,
        Err(_) => return HttpResponse::BadRequest().body("Invalid price format"),
    };
    let quantity = match Decimal::from_str(&order_data.quantity) {
        Ok(q) => q,
        Err(_) => return HttpResponse::BadRequest().body("Invalid quantity format"),
    };

    let msg = CreateOrder {
        user_id: order_data.user_id,
        side: order_data.side,
        price,
        quantity,
    };

    match engine_addr.send(msg).await {
        Ok(Ok(order_id)) => HttpResponse::Ok().json(crate::output::CreateOrderResponse {
            status: "Order received".to_string(),
            order_id: order_id.to_string(),
        }),
        Ok(Err(e)) => HttpResponse::InternalServerError().body(e),
        Err(_) => HttpResponse::InternalServerError().body("Actor mailbox error"),
    }
}

#[get("/order/{order_id}")]
pub async fn get_order_route(
    path: web::Path<Uuid>,
    engine_addr: web::Data<Addr<MatchingEngine>>,
) -> impl Responder {
    let order_id = path.into_inner();
    let msg = GetOrder { order_id };

    match engine_addr.send(msg).await {
        Ok(Ok(order)) => HttpResponse::Ok().json(OrderResponse {
            order_id: order.order_id.to_string(),
            user_id: order.user_id,
            side: order.side,
            price: order.price.to_string(),
            quantity: order.quantity.to_string(),
            filled_quantity: order.filled_quantity.to_string(),
            timestamp: order.timestamp,
        }),
        Ok(Err(e)) => HttpResponse::NotFound().body(e),
        Err(_) => HttpResponse::InternalServerError().body("Actor mailbox error"),
    }
}

#[delete("/order/{order_id}/{user_id}")]
pub async fn cancel_order_route(
    path: web::Path<(Uuid, String)>,
    engine_addr: web::Data<Addr<MatchingEngine>>,
) -> impl Responder {
    let (order_id, user_id) = path.into_inner();
    let msg = CancelOrder { order_id, user_id };
    ///@note where is CancelOrder coming from?
    match engine_addr.send(msg).await {
        Ok(Ok(id)) => HttpResponse::Ok().json(crate::output::CreateOrderResponse {
            status: "Cancel request accepted".to_string(),
            order_id: id.to_string(),
        }),
        Ok(Err(e)) => HttpResponse::BadRequest().body(e),
        Err(_) => HttpResponse::InternalServerError().body("Actor mailbox error"),
    }
}

#[get("/depth")]
pub async fn get_depth_route(engine_addr: web::Data<Addr<MatchingEngine>>) -> impl Responder {
    match engine_addr.send(GetDepth).await {
        ///@note where is GetDepth coming from?
        Ok(Ok(depth)) => HttpResponse::Ok().json(depth),
        Ok(Err(e)) => HttpResponse::BadRequest().body(e),
        Err(_) => HttpResponse::InternalServerError().body("Actor mailbox error"),
    }
}
