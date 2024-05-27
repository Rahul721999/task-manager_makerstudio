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
    let mut state_data = match state_data.data.lock() {
        Ok(data) => data,
        Err(_) => {
            error!("Failed to acquire lock on the state data");
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };

    let user_id = user_id.into_inner();

    match state_data.users.get_mut(&user_id) {
        Some(user) => {
            let new_task = Task::new(&req.title, &req.description, req.due_date);
            let task_id = new_task.id;
            user.tasks.insert(task_id, new_task);

            save_data(&state_data);

            info!("Task created successfully with ID: {}", task_id);
            HttpResponse::Ok().json(task_id)
        }
        None => {
            HttpResponse::NotFound().body("User not found")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utility::test_utils::{create_test_user_and_task, init_app_state};
    use actix_web::http::StatusCode;
    use uuid::Uuid;

    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_create_task() {
        // Initialize the app state with an in-memory database
        let app_state = init_app_state();

        // Add a test user with test-task
        let (user_id, _test_task_id) = create_test_user_and_task(&app_state);


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
