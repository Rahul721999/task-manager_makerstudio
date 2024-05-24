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

#[cfg(test)]
mod test {
    use crate::schema::{load_data, save_data, Status, Task, User};
    use chrono::NaiveDate;
    use std::{str::FromStr, sync::Mutex};
    use uuid::Uuid;

    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_delete_task() {
        // Initialize the app state with an in-memory database
        let app_state = web::Data::new(AppState {
            data: Mutex::new(load_data()),
        });

        // Add a test user with test-task
        let mut test_user = User::new("test-user");
        let user_id = test_user.id;
        let test_task = Task::new(
            "sample-title",
            "sample-info",
            NaiveDate::from_ymd_opt(2000, 1, 1).expect("failed to parse NaiveDate"),
        );
        let test_task_id = test_task.id;

        // test task to the user
        test_user.tasks.insert(test_task.id, test_task);
        if let Ok(mut state_data) = app_state.data.lock() {
            state_data.users.insert(user_id, test_user);
            save_data(&state_data);
        };

        // Initialize the test app with the necessary route
        let mut app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/users/{user_id}/tasks/delete", web::post().to(delete_task)),
        )
        .await;

        // create JSON payload for delete task route
        let del_task = DeleteTask { id: test_task_id };

        // Create a request to create a task
        let req = test::TestRequest::post()
            .uri(&format!("/users/{}/tasks/delete", user_id))
            .set_json(del_task)
            .to_request();

        // Call the API and compare the response with the expected result
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let resp_body = test::read_body(resp).await;
        let resp_task_id: Uuid = serde_json::from_slice(&resp_body).unwrap();

        // Verify that the task was deleted from the user's tasks
        if let Ok(mut state_data) = app_state.data.lock() {
            if let Some(user) = state_data.users.get(&user_id) {
                assert!(!user.tasks.contains_key(&resp_task_id));
            };
            // forcefully reomving "new-test-user" to avoid duplicates
            state_data.users.remove(&user_id);
            save_data(&state_data);
        };

    }
}
