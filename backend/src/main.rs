mod app_data;
mod errors;
mod helpers;
mod routes;

use crate::app_data::AppData;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use clap::{command, value_parser, Arg};
use routes::{file_handler, ping};
use std::{env, sync::Arc};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let match_result = command!()
        .arg(
            Arg::new("server_port")
                .short('p')
                .long("server_port")
                .value_parser(value_parser!(u16)),
        )
        .get_matches();

    dotenvy::dotenv().unwrap();

    let app_data = Arc::new(AppData::new());

    tokio::fs::remove_dir(&app_data.temp_folder).await.ok();
    tokio::fs::create_dir(&app_data.temp_folder).await.unwrap();
    tokio::fs::remove_dir(&app_data.converted_files_folder)
        .await
        .ok();
    tokio::fs::create_dir(&app_data.converted_files_folder)
        .await
        .unwrap();

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/convert", post(file_handler))
        // .layer(DefaultBodyLimit::max(1024))
        .with_state(app_data);

    let server_port: &u16 = match_result.get_one("server_port").unwrap();
    let listener = TcpListener::bind(&format!("0.0.0.0:{server_port}"))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
