use axum::Router;
use clap::{command, value_parser, Arg};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

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

    let server_port: &u16 = match_result.get_one("server_port").unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], *server_port));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, using_serve_dir()).await.unwrap();
}

fn using_serve_dir() -> Router {
    Router::new().nest_service("/", ServeDir::new("assets"))
}
