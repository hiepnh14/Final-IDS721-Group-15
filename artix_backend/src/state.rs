use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub username: String,
    pub message: String,
}

pub struct AppState {
    pub messages: Mutex<Vec<Message>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            messages: Mutex::new(Vec::new()),
        }
    }
}
