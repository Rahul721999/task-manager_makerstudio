use crate::{
    schema::{save_data, Status},
    AppState,
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTask {
    id: Uuid,
    status: Status,
}

/// API endpoint to create new task
/// URL: "/users/{userid}/tasks/update"
pub async fn update_task(
    state_data: web::Data<AppState>,
    user_id: web::Path<Uuid>,
    req: web::Json<UpdateTask>,
) -> impl Responder {
    match state_data.data.lock() {
        Ok(mut state_data) => {
            if let Some(user) = state_data.users.get_mut(&user_id.into_inner()) {
                // parse task_id
                let UpdateTask {
                    id: task_id,
                    status: task_status,
                } = req.into_inner();

                if let Some(task) = user.tasks.get_mut(&task_id) {
                    task.status = task_status.clone();

                    // update the new data to DB
                    save_data(&state_data);

                    info!(
                        "Staus of Task-Id: {}, updated to: {:?}",
                        task_id, task_status
                    );
                    HttpResponse::Ok().json(task_id)
                } else {
                    warn!("Task-id: {} doesn't exists", task_id);
                    HttpResponse::NotFound().body("Task Doesn't exists")
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

#[cfg(test)]
mod test {
    use crate::schema::{load_data, save_data, Status, Task, User};
    use chrono::NaiveDate;
    use std::sync::Mutex;
    use uuid::Uuid;
    use actix_web::http::StatusCode;

    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_update_task() {
        // Initialize the app state with an in-memory database
        let app_state = web::Data::new(AppState {
            data: Mutex::new(load_data()),
        });

        // Add a test user with a test task
        let mut test_user = User::new("test-user");
        let user_id = test_user.id;
        let test_task = Task::new(
            "sample-title",
            "sample-info",
            NaiveDate::from_ymd_opt(2000, 1, 1).expect("failed to parse NaiveDate"),
        );
        let test_task_id = test_task.id;

        // Add the test task to the user
        test_user.tasks.insert(test_task.id, test_task);
        if let Ok(mut state_data) = app_state.data.lock() {
            state_data.users.insert(user_id, test_user);
            save_data(&state_data);
        };

        // Initialize the test app with the necessary route
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/users/{user_id}/tasks/update", web::post().to(update_task)),
        )
        .await;

        // Create JSON payload for updating task status
        let update_task_payload = UpdateTask {
            id: test_task_id,
            status: Status::InProgress,
        };

        // Create a request to update the task status
        let req = test::TestRequest::post()
            .uri(&format!("/users/{}/tasks/update", user_id))
            .set_json(&update_task_payload)
            .to_request();

        // Call the API and compare the response with the expected result
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let resp_body = test::read_body(resp).await;
        let resp_task_id: Uuid = serde_json::from_slice(&resp_body).unwrap();

        // Compare the task ID from the response with the task ID provided
        assert_eq!(resp_task_id, test_task_id);

        // Verify that the task's status was updated
        if let Ok(mut state_data) = app_state.data.lock() {
            if let Some(user) = state_data.users.get(&user_id) {
                if let Some(task) = user.tasks.get(&resp_task_id) {
                    assert_eq!(task.status, Status::InProgress); // Ensure the status was updated
                }
                state_data.users.remove(&user_id);
                save_data(&state_data);
            }
        };

        // Clean up: Remove the test user to avoid duplicates
    }
}
