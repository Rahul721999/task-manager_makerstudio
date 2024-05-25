use crate::{schema::save_data, AppState};
use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct DeleteUser {
    id: Uuid,
}

/// API endpoint to delete a user
/// URL: "/users/delete"
pub async fn delete_user(
    req: web::Json<DeleteUser>,
    state_data: web::Data<AppState>,
) -> impl Responder {
    if let Ok(mut db) = state_data.data.lock() {
        let user_id = req.id;
        info!("Removing user: {} from db", user_id);
        if db.users.remove(&user_id).is_some() {
            save_data(&db);
            HttpResponse::Ok().body(format!("UserID: {} deleted", user_id))
        } else {
            HttpResponse::NotFound().body(format!("UserID:{} not found", user_id))
        }
    } else {
        HttpResponse::InternalServerError().body("failed to process your request")
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::schema::load_data;
    use actix_web::http::StatusCode;
    use actix_web::{test, App};
    use std::sync::Mutex;
    use crate::schema::User;

    #[actix_web::test]
    async fn test_delete_user() {
        let app_state = web::Data::new(AppState {
            data: Mutex::new(load_data()),
        });

        // adding 'test-user' before deleting it.
        let user = User::new("test-delete-user");
        let user_id = user.id;
        if let Ok(mut state_data) = app_state.data.lock() {
            if state_data.users.insert(user_id, user).is_some() {
                save_data(&state_data);
            }
        };

        // creating Test app
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/users/delete", web::post().to(delete_user)),
        )
        .await;

        // creating req for api
        let req = test::TestRequest::post()
            .uri("/users/delete")
            .set_json(DeleteUser { id: user_id })
            .to_request();

        // Call the API and compare response with expected result
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
