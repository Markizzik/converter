use crate::{
    app_data::AppData,
    errors::AppErr,
    helpers::{ffmpeg_convert, gen_random_string, rm_dir, ConvertQuery},
};
use axum::{
    extract::{Multipart, Query, State},
    http::StatusCode,
};
use std::{path::Path, sync::Arc};
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn ping() -> &'static str {
    "pong"
}

pub async fn file_handler(
    State(app_data): State<Arc<AppData>>,
    Query(ConvertQuery { new_file_format }): Query<ConvertQuery>,
    mut multipart: Multipart,
) -> Result<String, AppErr> {
    let temp_folder = &app_data.temp_folder;
    let converted_files_folder = &app_data.converted_files_folder;

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap();

        if field_name != "file" {
            continue;
        }

        let file_name = field.file_name().unwrap().to_string();

        let rnd_string = gen_random_string();

        let Ok(file_bytes) = field.bytes().await else {
            return Err(AppErr::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "no file bytes found",
            ));
        };

        let file_name_without_extension =
            Path::new(&file_name).file_stem().unwrap().to_str().unwrap();

        let download_folder = format!("{temp_folder}/{rnd_string}");
        let input_file_path = format!("{download_folder}/{file_name}");

        let output_folder = format!("{converted_files_folder}/{rnd_string}");
        let output_file_path =
            format!("{output_folder}/{file_name_without_extension}.{new_file_format}");

        tokio::fs::create_dir(&download_folder).await.unwrap();
        let mut file = File::create(&input_file_path).await.unwrap();

        file.write_all(&file_bytes).await.unwrap();

        ffmpeg_convert(&input_file_path, &output_file_path)
            .await
            .unwrap();

        // rm_dir(&download_folder).await.unwrap();

        return Ok(output_file_path);
    }

    Err(AppErr::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "file convert error",
    ))
}
