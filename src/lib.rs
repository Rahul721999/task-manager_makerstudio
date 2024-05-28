// src/lib.rs
pub mod routes;
pub mod schema;
pub mod server;

pub use routes::task_routes::*;
pub use routes::user_routes::*;
pub use schema::*;
pub use server::*;


// AppState
use std::sync::Mutex;
pub struct AppState {
    pub data: Mutex<AppStateData>,
}

#[cfg(test)]
pub mod test_utils {
    use super::{load_data, save_data, User, Task};
    use crate::AppState;
    use actix_web::web;
    use chrono::NaiveDate;
    use std::sync::Mutex;
    use uuid::Uuid;

    pub fn init_app_state() -> web::Data<AppState> {
        web::Data::new(AppState {
            data: Mutex::new(load_data()),
        })
    }

    pub fn create_test_user_and_task(app_state: &web::Data<AppState>) -> (Uuid, Uuid) {
        // Create a test user and a sample task
        let mut test_user = User::new("test-user");
        let user_id = test_user.id;
        let test_task = Task::new(
            "sample-title",
            "sample-info",
            NaiveDate::from_ymd_opt(2000, 1, 1).expect("failed to parse NaiveDate"),
        );
        let test_task_id = test_task.id;

        // Add the test task to the user
        test_user.tasks.insert(test_task.id, test_task);
        if let Ok(mut state_data) = app_state.data.lock() {
            state_data.users.insert(user_id, test_user);
            save_data(&state_data);
        };

        (user_id, test_task_id)
    }
}