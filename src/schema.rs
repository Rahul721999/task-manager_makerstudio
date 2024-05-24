use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use uuid::Uuid;

const DATA_FILE: &str = "data.json";

// User schema..
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub tasks: HashMap<String, Task>,
}
impl User {
    pub fn new(name: &str) -> Self {
        let id = Uuid::new_v4();
        User {
            id,
            name : name.to_string(),
            tasks: HashMap::new(),
        }
    }
}
// Task Schema..
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub status: Status,
}

// Task-Status Schema..

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppStateData {
    pub users: HashMap<String, User>,
}

pub fn load_data() -> AppStateData {
    let mut file = match File::open(DATA_FILE) {
        Ok(file) => file,
        Err(_) => return AppStateData::default(),
    };

    let mut data = String::new();
    if file.read_to_string(&mut data).is_ok() {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        AppStateData::default()
    }
}

pub fn save_data(data: &AppStateData) {
    if let Ok(json) = serde_json::to_string(data) {
        if let Ok(mut file) = File::create(DATA_FILE) {
            let _ = file.write_all(json.as_bytes());
        }
    }
}
