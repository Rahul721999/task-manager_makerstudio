use crate::{
    schema::{save_data, Status, Task},
    AppState,
};
use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use log::{error, info};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewTask {
    title: String,
    description: String,
    due_date: NaiveDate,
    status: Status,
}

/// API endpoint to create new task
/// URL: "/users/{userid}/tasks/create"
pub async fn create_task(
    state_data: web::Data<AppState>,
    user_id: web::Path<Uuid>,
    req: web::Json<NewTask>,
) -> impl Responder {
    match state_data.data.lock() {
        Ok(mut state_data) => {
            if let Some(user) = state_data.users.get_mut(&user_id.into_inner()) {
                // creating new task
                let new_task = Task::new(&req.title, &req.description, req.due_date);
                let task_id = new_task.id;
                user.tasks.insert(task_id, new_task);

                // update the new data to DB
                save_data(&state_data);

                info!("Task created successfully with ID: {}", task_id);
                HttpResponse::Ok().json(task_id)
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
    use crate::schema::{load_data, save_data, Status,User};
    use chrono::NaiveDate;
    use std::sync::Mutex;
    use uuid::Uuid;
    use actix_web::http::StatusCode;

    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_create_task() {
        // Initialize the app state with an in-memory database
        let app_state = web::Data::new(AppState {
            data: Mutex::new(load_data()),
        });

        // Add a test user
        let test_user = User::new("test-user");
        let user_id = test_user.id;
        if let Ok(mut state_data) = app_state.data.lock() {
            state_data.users.insert(user_id, test_user);
            save_data(&state_data);
        };

        // Initialize the test app with the necessary route
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/users/{user_id}/tasks/create", web::post().to(create_task)),
        )
        .await;

        // Define a new task
        let new_task = NewTask {
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            due_date: NaiveDate::from_ymd_opt(2024, 5, 24).expect("failed to create Due-Date"),
            status: Status::ToDo,
        };

        // Create a request to create a task
        let req = test::TestRequest::post()
            .uri(&format!("/users/{}/tasks/create", user_id))
            .set_json(&new_task)
            .to_request();

        // Call the API and compare the response with the expected result
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let resp_body = test::read_body(resp).await;
        let task_id: Uuid = serde_json::from_slice(&resp_body).unwrap();

        // Verify that the task was added to the user's tasks
        if let Ok(mut state_data) = app_state.data.lock() {
            if let Some(user) = state_data.users.get(&user_id) {
                assert!(user.tasks.contains_key(&task_id));
            };
            // forcefully reomving "new-test-user" to avoid duplicates
            state_data.users.remove(&user_id);
            save_data(&state_data);
        };
    }
}
