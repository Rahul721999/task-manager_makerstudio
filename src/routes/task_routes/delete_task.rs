use crate::{schema::save_data, AppState};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteTask {
    id: Uuid,
}

/// API endpoint to delete new task
/// URL: "/users/{userid}/tasks/delete"
pub async fn delete_task(
    state_data: web::Data<AppState>,
    user_id: web::Path<Uuid>,
    req: web::Json<DeleteTask>,
) -> impl Responder {
    let mut state_data = match state_data.data.lock() {
        Ok(data) => data,
        Err(_) => {
            error!("Failed to acquire lock on the state data");
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };

    let user_id = user_id.into_inner();
    let task_id = &req.id;

    match state_data.users.get_mut(&user_id) {
        Some(user) => {
            if user.tasks.remove(task_id).is_some() {
                save_data(&state_data);
                info!("Task deleted successfully with ID: {}", task_id);
                HttpResponse::Ok().json(task_id)
            } else {
                warn!("Task-id: {} doesn't exist", task_id);
                HttpResponse::NotFound().body("Task doesn't exist")
            }
        }
        None => HttpResponse::NotFound().body("User not found"),
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
    async fn test_delete_task() {
        // Initialize the app state with an in-memory database
        let app_state = init_app_state();

        // Add a test user with test-task
        let (user_id, test_task_id) = create_test_user_and_task(&app_state);

        // Initialize the test app with the necessary route
        let app = test::init_service(
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

        // comparing the task-id from response with the task-id provided
        assert_eq!(resp_task_id, test_task_id);

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
