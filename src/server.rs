use crate::routes::{
    task_routes::{create_task, delete_task, list_task, update_task},
    user_routes::{create_user, delete_user},
};
use crate::AppState;

// extern crate imports
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use anyhow::Result;

pub async fn start_service(
    app_state: web::Data<AppState>,
) -> Result<actix_web::dev::Server, Box<dyn std::error::Error>> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/users")
                    .wrap(Logger::default())
                    .route("/create", web::post().to(create_user))
                    .route("/{userId}/delete", web::delete().to(delete_user)),
            )
            .service(
                web::scope("/users/{userId}/tasks")
                    .wrap(Logger::default())
                    .route("/create", web::post().to(create_task))
                    .route("/{taskId}", web::get().to(list_task))
                    .route("/{taskId}", web::put().to(update_task))
                    .route("/{taskId}", web::delete().to(delete_task)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run();
    Ok(server)
}
