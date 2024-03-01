use crate::{app_data::AppData, errors::AppErr};
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
};
use std::sync::Arc;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn file_handler(
    State(app_data): State<Arc<AppData>>,
    mut multipart: Multipart,
) -> Result<String, AppErr> {
    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name().unwrap() == "file" {
            let file_name = field.file_name().unwrap().to_string();

            let Ok(file_bytes) = field.bytes().await else {
                continue;
            };

            // let input_file_path = format!("./{}/{file_name}", app_data.temp_folder);

            // let mut file = OpenOptions::new()
            //     .read(true)
            //     .write(true)
            //     .create(true)
            //     .open(&input_file_path)
            //     .await
            //     .unwrap();

            // file.write_all(&file_bytes).await.unwrap();

            // let output_file_path = format!("./{}/{file_name}.wav", app_data.temp_folder);

            // ffmpeg_convert(&input_file_path, &output_file_path).await?;

            // let file = File::open(&output_file_path).await.unwrap();

            // return app_data.file_tt(file, &format!("{file_name}.wav")).await;
        }
    }

    Err(AppErr::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "no file bytes found",
    ))
}
