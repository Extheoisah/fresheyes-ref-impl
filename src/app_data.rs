// app_data.rs
use std::sync::{Arc, Mutex};
#[derive(Debug, Clone)]
pub struct AppData {
    pub token: Arc<Mutex<String>>,
}

impl AppData {
    pub fn new(token: String) -> Self {
        Self {
            token: Arc::new(Mutex::new(token.parse().unwrap())),
        }
    }
}
