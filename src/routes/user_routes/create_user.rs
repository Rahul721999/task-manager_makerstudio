use crate::{
    schema::{save_data, User},
    AppState,
};
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use log::{error, info};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    name: String,
}
pub async fn create_user(
    state_data: web::Data<AppState>,
    req: web::Json<NewUser>,
) -> impl Responder {
    if let Ok(mut state_data) = state_data.data.lock() {
        let new_user = User::new(&req.name);
        let user_id = new_user.id;
        
        // add new user to the DB
        state_data.users.insert(user_id, new_user);

        // update the new data to DB
        save_data(&state_data);

        info!("User created successfully with ID: {}", user_id);
        return HttpResponse::Ok().json(user_id);
    };
    error!("failed to get DB");
    HttpResponse::InternalServerError().body("DB error")
}


#[cfg(test)]
mod test{

    use uuid::Uuid;
    use std::sync::Mutex;
    use crate::schema::load_data;
    use std::collections::HashMap;

    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_create_user(){
        let app_state = web::Data::new(AppState{
            data: Mutex::new(load_data()),
        });

        // creating Test app
        let mut app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/users/create", web::post().to(create_user)),
        ).await;

        // creating req for api
        let req = test::TestRequest::post()
            .uri("/users/create")
            .set_json(NewUser{
                name : "Test-User".to_string(),
            })
            .to_request();

        // comparing response with expected result
        let resp: Uuid = test::call_and_read_body_json(&app, req).await;
        assert!(!resp.is_nil());

        // forcefully reomving "new-test-user" to avoid duplicates
        if let Ok(mut state_data) = app_state.data.lock() {
            if state_data.users.remove(&resp).is_some(){
                save_data(&state_data);
            }
        };

    }
}