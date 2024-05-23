use actix_web::{HttpResponse, Responder};

pub async fn delete_user() -> impl Responder{
    HttpResponse::Ok().body("delete_user")
}