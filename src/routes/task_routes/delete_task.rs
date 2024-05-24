use crate::{
    schema::{save_data, Status, Task, User},
    AppState,
};
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use chrono::NaiveDate;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteTask {
    id: Uuid,
}

/// API endpoint to create new task
/// URL: "/users/{userid}/tasks/delete"
pub async fn delete_task(
    state_data: web::Data<AppState>,
    user_id: web::Path<Uuid>,
    req: web::Json<DeleteTask>,
) -> impl Responder {
    match state_data.data.lock() {
        Ok(mut state_data) => {
            if let Some(user) = state_data.users.get_mut(&user_id.into_inner()) {
                // parse task_id
                let task_id = &req.id;
                if !user.tasks.contains_key(task_id) {
                    warn!("Task-id: {} doesn't exists", task_id);
                    HttpResponse::NotFound().body("Task Doesn't exists")
                } else {
                    user.tasks.remove(task_id);

                    // update the new data to DB
                    save_data(&state_data);

                    info!("Task deleted successfully with ID: {}", task_id);
                    HttpResponse::Ok().json(task_id)
                }
            } else {
                HttpResponse::NotFound().body("User not found")
            }
        }
        Err(_) => {
            error!("Failed to acquire lock on the state data");
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}
