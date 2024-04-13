use crate::{
    app_data::AppData,
    errors::AppErr,
    helpers::{ffmpeg_convert, gen_random_string, rm_file, FileFormat},
};
use axum::{
    extract::{Multipart, State},
    http::{HeaderMap, StatusCode},
};
use std::sync::Arc;
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn root() -> &'static str {
    "Hello!"
}

pub async fn file_handler(
    State(app_data): State<Arc<AppData>>,
    mut multipart: Multipart,
) -> Result<String, AppErr> {
    let temp_folder = &app_data.temp_folder;

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap();

        if field_name != "file" {
            continue;
        }

        let file_name = field.file_name().unwrap().to_string();

        let headers = field.headers();

        let Some(new_file_format) = headers.get("new_file_format") else {
            return Err(AppErr::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "no new file format found",
            ));
        };

        let Ok(new_file_format) = new_file_format.to_str() else {
            return Err(AppErr::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "parse new file format error",
            ));
        };

        let rnd_string = gen_random_string();

        let output_file_path =
            format!("./{temp_folder}/{rnd_string}/{file_name}.{new_file_format}");

        let new_file_download_link = format!("{rnd_string}/{file_name}.{new_file_format}");

        let Ok(file_bytes) = field.bytes().await else {
            return Err(AppErr::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "no file bytes found",
            ));
        };

        let input_file_path = format!("./{}/{file_name}", app_data.temp_folder);

        let mut file = File::create(&input_file_path).await.unwrap();

        file.write_all(&file_bytes).await.unwrap();

        ffmpeg_convert(&input_file_path, &output_file_path).await?;

        rm_file(&input_file_path).await?;

        return Ok(new_file_download_link);
    }

    Err(AppErr::new(StatusCode::INTERNAL_SERVER_ERROR, ""))
}
