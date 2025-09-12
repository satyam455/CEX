use crate::input::{CreateOrderInput, DeleteOrderInput, Depth};
use crate::output::{CreateOrderResponse, DeleteOrderResponse};
use actix_web::HttpResponse;
use actix_web::{delete, get, post, web::Json, Responder};

#[post("/order")]

pub async fn create_order(body: Json<CreateOrderInput>) -> impl Responder {
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;

    return HttpResponse::Ok().json(CreateOrderResponse {
        order_id: String::from("Response"),
    });

    // println!("{:?}", body);
    // "Order created"
}

#[delete("/order")]

pub async fn delete_order(body: Json<DeleteOrderInput>) -> impl Responder {
    let order_id = body.0.order_id;

    return HttpResponse::Ok().json(DeleteOrderResponse {
        price: 0,
        quantity: 0,
    });
}

#[get("/depth")]

pub async fn get_depth(body: json<Depth>) -> impl Responder {
    HttpResponse::Ok().json(Depth {
        bids: vec![],
        asks: vec![],
        lastUpdatedId: String::from("Depth"),
    })
}
