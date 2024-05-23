use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

const DATA_FILE: &str = "data.json";

// User schema..
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: String,
    name: String,
    tasks: HashMap<String, Task>,
}

// Task Schema..
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    id: String,
    title: String,
    description: String,
    due_date: String,
    status: Status,
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
    users: HashMap<String, User>,
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