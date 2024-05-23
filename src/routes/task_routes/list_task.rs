use actix_web::{HttpResponse, Responder};

pub async fn list_task() -> impl Responder{
    HttpResponse::Ok().body("list-task")
}