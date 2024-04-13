use std::env;

pub struct AppData {
    pub temp_folder: String,
    pub converted_files_folder: String,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            temp_folder: env::var("TEMP_FOLDER").unwrap(),
            converted_files_folder: env::var("CONVERTED_FILES_FOLDER").unwrap(),
        }
    }
}
