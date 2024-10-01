use std::net::SocketAddrV4;
use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};
use sqlx::postgres::{PgPoolOptions, PgPool};
use dotenv;

mod models;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv::dotenv().unwrap();

    let socket_addr = dotenv::var("SOCKET_ADDR").unwrap();
    let socket_addr = socket_addr.parse::<SocketAddrV4>().map_err(|e| format!("Failed to parse socket address {}", e.to_string()))?;

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .map_err(|e| format!("Failed to bind to socket. Error: {}", e.to_string()))?;

    let db_connection_string = dotenv::var("DATABASE_URL").unwrap();
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_connection_string)
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(hello_world))
        .with_state(AppState { db });

    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .map_err(|e| format!("Server failed to start. Error: {}", e.to_string()))?;

    Ok(())
}

async fn hello_world() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "Hello, world!" }
            }
            body {
                h1 { "Hello, world!" }
                p { "Welcome to the Rust website." }
            }
        }
    }
}