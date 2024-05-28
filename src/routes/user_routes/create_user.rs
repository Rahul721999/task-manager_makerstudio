// src/routes/user_routes/create_user.rs
use crate::{
    schema::{save_data, User},
    AppState,
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}

/// API endpoint to create new task
/// URL: "/users/delete/"
pub async fn create_user(
    state_data: web::Data<AppState>,
    req: web::Json<NewUser>,
) -> impl Responder {
    // Try acquiring the lock on the mutex
    let mut state_data = match state_data.data.lock() {
        Ok(state_data) => state_data,
        Err(_) => {
            error!("Failed to acquire lock on the state data");
            return HttpResponse::InternalServerError().body("DB error");
        }
    };

    let new_user = User::new(&req.name);
    let user_id = new_user.id;

    // Add new user to the DB
    state_data.users.insert(user_id, new_user);

    // Update the new data to DB
    save_data(&state_data);

    info!("User created successfully with ID: {}", user_id);
    HttpResponse::Ok().json(user_id)
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::load_data;
    use actix_web::{test, App};
    use std::sync::Mutex;
    use uuid::Uuid;

    #[actix_web::test]
    async fn test_create_user() {
        let app_state = web::Data::new(AppState {
            data: Mutex::new(load_data()),
        });

        // creating Test app
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/users/create", web::post().to(create_user)),
        )
        .await;

        // creating req for api
        let req = test::TestRequest::post()
            .uri("/users/create")
            .set_json(NewUser {
                name: "Test-User".to_string(),
            })
            .to_request();

        // comparing response with expected result
        let resp: Uuid = test::call_and_read_body_json(&app, req).await;
        assert!(!resp.is_nil());

        // forcefully reomving "new-test-user" to avoid duplicates
        if let Ok(mut state_data) = app_state.data.lock() {
            if state_data.users.remove(&resp).is_some() {
                save_data(&state_data);
            }
        };
    }
}
