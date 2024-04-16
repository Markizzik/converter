use crate::{
    app_data::AppData,
    errors::AppErr,
    helpers::{convert, ConvertQuery},
};
use axum::{
    extract::{Multipart, Query, State},
    http::StatusCode,
};
use std::sync::Arc;

pub async fn ping() -> &'static str {
    "pong"
}

pub async fn convert_handler(
    State(app_data): State<Arc<AppData>>,
    Query(ConvertQuery { new_file_format }): Query<ConvertQuery>,
    mut multipart: Multipart,
) -> Result<String, AppErr> {
    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap();

        if field_name != "file" {
            continue;
        }

        let file_name = field.file_name().unwrap().to_string();

        let output_file_path = convert(field, &file_name, &new_file_format, &app_data).await?;

        return Ok(output_file_path);
    }

    Err(AppErr::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "file convert error",
    ))
}
