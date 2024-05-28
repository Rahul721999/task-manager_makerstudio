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
    pub id: Uuid,
    pub status: Status,
}

/// API endpoint to create new task
/// URL: "/users/{userid}/tasks/update"
pub async fn update_task(
    state_data: web::Data<AppState>,
    user_id: web::Path<Uuid>,
    req: web::Json<UpdateTask>,
) -> impl Responder {
    // try aquiring the lock on mutex
    let mut state_data = match state_data.data.lock() {
        Ok(state_data) => state_data,
        Err(_) => {
            error!("Failed to acquire lock on the state data");
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };

    // try finding the user in db
    let user = match state_data.users.get_mut(&user_id.into_inner()) {
        Some(user) => user,
        None => return HttpResponse::NotFound().body("User not found"),
    };

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
}

#[cfg(test)]
mod test {
    use crate::test_utils::{create_test_user_and_task, init_app_state};
    use actix_web::http::StatusCode;
    use uuid::Uuid;

    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_update_task() {
        // Initialize the app state with an in-memory database
        let app_state = init_app_state();

        // Add a test user with test-task
        let (user_id, test_task_id) = create_test_user_and_task(&app_state);

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
