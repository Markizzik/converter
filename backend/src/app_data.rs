use std::env;

pub struct AppData {
    pub temp_folder: String,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            temp_folder: env::var("TEMP_FOLDER").unwrap(),
        }
    }
}
