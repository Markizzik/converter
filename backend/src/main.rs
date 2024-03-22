mod app_data;
mod errors;
mod routes;

use crate::app_data::AppData;
use axum::{routing::get, Router};
use clap::{command, Arg};
use routes::root;
use std::{env, sync::Arc};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let match_result = command!()
        .arg(Arg::new("server_port").short('p'))
        .get_matches();

    dotenvy::dotenv().unwrap();

    let app_data = Arc::new(AppData::new());

    let app = Router::new().route("/", get(root)).with_state(app_data);

    let server_port: &u16 = match_result.get_one("server_port").unwrap();
    let listener = TcpListener::bind(&format!("0.0.0.0:{server_port}"))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
