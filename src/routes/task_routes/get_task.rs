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
pub struct GetTask {
    id: Uuid,
}

/// API endpoint to delete new task
/// URL: "/users/{userid}/tasks/get-task"
pub async fn get_task(
    state_data: web::Data<AppState>,
    user_id: web::Path<Uuid>,
    req: web::Json<GetTask>,
) -> impl Responder {
    match state_data.data.lock() {
        Ok(mut state_data) => {
            let user_id = &user_id.into_inner();
            if let Some(user) = state_data.users.get(user_id) {
                let task_id = &req.id;
                if let Some(task) = user.tasks.get(task_id) {
                    info!("TaskId: {} found for userId: {}", task_id, user_id);
                    HttpResponse::Ok().json(task)
                } else {
                    warn!("TaskId: {} not exist", task_id);
                    HttpResponse::NotFound().body(format!("TaskId: {} doesn't exist", task_id))
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
    use super::*;
    use crate::schema::{load_data, save_data, Task, User};
    use actix_web::{test, web, App, http::StatusCode};
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTask {
        id: Uuid,
    }

    #[actix_web::test]
    async fn test_get_task() {
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
        test_user.tasks.insert(test_task.id, test_task.clone());
        if let Ok(mut state_data) = app_state.data.lock() {
            state_data.users.insert(user_id, test_user);
            save_data(&state_data);
        };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state.clone()))
                .route("/users/{user_id}/tasks/get-task", web::post().to(get_task)),
        )
        .await;

        let get_task_req = GetTask { id: test_task_id };
        let req = test::TestRequest::post()
            .uri(&format!("/users/{}/tasks/get-task", user_id))
            .set_json(&get_task_req)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let response_task: Task = test::read_body_json(resp).await;
        assert_eq!(response_task.id, test_task_id);
        assert_eq!(response_task.title, test_task.title);
    }

    #[actix_web::test]
    async fn test_get_task_not_found() {
        let user_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();

        // Initialize the app state with an in-memory database
        let app_state = web::Data::new(AppState {
            data: Mutex::new(load_data()),
        });

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state.clone()))
                .route("/users/{user_id}/tasks/get-task", web::post().to(get_task)),
        )
        .await;

        let get_task_req = GetTask { id: task_id };
        let req = test::TestRequest::post()
            .uri(&format!("/users/{}/tasks/get-task", user_id))
            .set_json(&get_task_req)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let body = test::read_body(resp).await;
        assert_eq!(body, "User not found");
    }
}

