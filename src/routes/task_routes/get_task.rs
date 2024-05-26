use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
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
    // Attempt to acquire the lock
    let state_data = match state_data.data.lock() {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error"),
    };

    // Extract user_id and task_id
    let user_id = user_id.into_inner();
    let task_id = &req.id;

    // Attempt to find the user and task
    match state_data.users.get(&user_id).and_then(|user| user.tasks.get(task_id)) {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().body(format!(
            "TaskId: {} or UserId: {} not found",
            task_id, user_id
        )),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::schema::{load_data, save_data, Task, User};
    use actix_web::{http::StatusCode, test, web, App};
    use chrono::NaiveDate;
    use std::sync::Mutex;
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
                .app_data(app_state.clone())
                .route("/users/{user_id}/tasks/get-task", web::get().to(get_task)),
        )
        .await;

        let get_task_req = GetTask { id: test_task_id };
        let req = test::TestRequest::get()
            .uri(&format!("/users/{}/tasks/get-task", user_id))
            .set_json(get_task_req)
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
                .route("/users/{user_id}/tasks/get-task", web::get().to(get_task)),
        )
        .await;

        let get_task_req = GetTask { id: task_id };
        let req = test::TestRequest::post()
            .uri(&format!("/users/{}/tasks/get-task", user_id))
            .set_json(&get_task_req)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
