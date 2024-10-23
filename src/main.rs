use std::{net::SocketAddrV4, path::PathBuf, sync::Arc};
use axum::{routing::{delete, get, post, put}, Router};
use sqlx::postgres::{PgPoolOptions, PgPool};
use tower_http::services::ServeDir;
use crate::templates::*;
use crate::routes::*;

mod models;
mod handlers;
mod schemas;
mod templates;
mod routes;
mod repositories;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv::dotenv().unwrap();

    let static_dir = PathBuf::from(dotenv::var("STATIC_DIR").unwrap());
    println!("Serving static assets from: {:?}", std::fs::canonicalize(&static_dir).unwrap_or("Failed to locate static file directory".into()));

    let socket_addr = dotenv::var("SOCKET_ADDR").unwrap();
    let socket_addr = socket_addr.parse::<SocketAddrV4>().map_err(|e| format!("Failed to parse socket address {}", e))?;

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .map_err(|e| format!("Failed to bind to socket. Error: {}", e))?;

    let db_connection_string = dotenv::var("DATABASE_URL").unwrap();
    let db = match PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_connection_string)
        .await {
            Ok(pool) => pool,
            Err(e) => panic!("Failed to connect to the database: {}", e),
        };

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/api/meetups", get(handlers::get_meetup_list))
        .route("/api/meetups/:id", get(handlers::get_meetup))
        .route("/api/meetups", post(handlers::create_meetup))
        .route("/api/meetups/:id", put(handlers::put_meetup))
        .route("/api/meetups/:id", delete(handlers::delete_meetup))
        .nest_service("/static", ServeDir::new(static_dir))
        .with_state(Arc::new(AppState { db: db.clone() }));

    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .map_err(|e| format!("Server failed to start. Error: {}", e))?;

    Ok(())
}