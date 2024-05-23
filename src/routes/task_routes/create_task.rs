use actix_web::{HttpResponse, Responder};

pub async fn create_task() -> impl Responder{
    HttpResponse::Ok().body("create-task")
}