mod app_data;
mod errors;
mod routes;

use crate::app_data::AppData;
use axum::{routing::get, Router};
use routes::root;
use std::{env, sync::Arc};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let app_data = Arc::new(AppData::new());

    let app = Router::new().route("/", get(root)).with_state(app_data);

    let listener = TcpListener::bind(&format!("0.0.0.0:{}", env::var("SERVER_PORT").unwrap()))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
