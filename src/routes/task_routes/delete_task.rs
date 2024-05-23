use actix_web::{HttpResponse, Responder};

pub async fn delete_task() -> impl Responder{
    HttpResponse::Ok().body("delete-task")
}