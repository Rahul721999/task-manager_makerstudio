#[cfg(test)]
pub mod test_utils {
    use crate::schema::{load_data, save_data, Task, User};
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
