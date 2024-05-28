use crate::{schema::Task, AppState};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

/// API endpoint to create new task
/// URL: "/users/{userid}/tasks/list"
pub async fn list_task(
    state_data: web::Data<AppState>,
    user_id: web::Path<Uuid>,
) -> impl Responder {
    // Try acquiring the lock on the mutex
    let state_data = match state_data.data.lock() {
        Ok(state_data) => state_data,
        Err(_) => {
            error!("Failed to acquire lock on the state data");
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };

    let user_id = user_id.into_inner();

    // Try finding the user in the database
    match state_data.users.get(&user_id) {
        Some(user) => {
            let task_list: Vec<Task> = user.tasks.values().cloned().collect();
            info!("Listing tasks for user ID: {}", user_id);
            HttpResponse::Ok().json(TaskList { tasks: task_list })
        }
        None => HttpResponse::NotFound().body("User not found"),
    }
}