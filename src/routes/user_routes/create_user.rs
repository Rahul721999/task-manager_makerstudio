use crate::{
    schema::{save_data, User},
    AppState,
};
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use log::{error, info};
use serde::Deserialize;

#[derive(Deserialize)]
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
