use crate::{schema::Task, AppState};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskList {
    tasks: Vec<Task>,
}

/// API endpoint to create new task
/// URL: "/users/{userid}/tasks/list"
pub async fn list_task(
    state_data: web::Data<AppState>,
    user_id: web::Path<Uuid>,
) -> impl Responder {
    match state_data.data.lock() {
        Ok(state_data) => {
            let user_id = &user_id.into_inner();
            if let Some(user) = state_data.users.get(user_id) {
                let task_list: Vec<Task> = user.tasks.values().cloned().collect();
                info!("Listing tasks for user ID: {}", user_id);
                HttpResponse::Ok().json(TaskList { tasks: task_list })
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
