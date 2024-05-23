use actix_web::{HttpResponse, Responder};

pub async fn update_task() -> impl Responder {
    HttpResponse::Ok().body("update-task")
}
