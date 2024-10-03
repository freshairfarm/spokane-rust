use std::{net::SocketAddrV4, path::PathBuf, sync::Arc};
use axum::{routing::{delete, get, post, put}, Router};
use maud::{html, Markup, DOCTYPE};
use sqlx::postgres::{PgPoolOptions, PgPool};
use tower_http::services::ServeDir;

mod models;
mod handlers;
mod schemas;

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

fn head(title: String) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            title { (title) }
            link href="https://cdn.jsdelivr.net/npm/beercss@3.7.10/dist/cdn/beer.min.css" rel="stylesheet";
            link rel="preconnect" href="https://fonts.googleapis.com";
            link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
            link href="https://fonts.googleapis.com/css2?family=Alfa+Slab+One&display=swap" rel="stylesheet";
            link href="/static/style.css" rel="stylesheet";
            script type="module" src="https://cdn.jsdelivr.net/npm/beercss@3.7.10/dist/cdn/beer.min.js"{}
            script type="module" src="https://cdn.jsdelivr.net/npm/material-dynamic-colors@1.1.2/dist/cdn/material-dynamic-colors.min.js"{}
        }
    }
}

async fn hello_world() -> Markup {
    html! {
        (DOCTYPE)
        html {
            (head("Hello, world!".into()))
            body {
                nav."drawer" {
                    header {
                        nav {
                            img."circle" src="https://s3-dev-usw2-spokanerust-static-objects.s3.us-west-2.amazonaws.com/cuddlyferris__1_-removebg-preview.png" alt="Ferris the Crab";
                            h6."alfa-slab-one-regular" { "Spokane Rust" }
                        }
                    }
                    a {
                        i { "home" }
                        span."max" { "Home" }
                    }
                    a { 
                        i { "event" }
                        span { "All Events" }
                    }
                    a {
                        i { "info" }
                        span { "About" }
                    }
                    a {
                        i { "menu_book" }
                        span { "Code of Conduct" }
                    }
                    a {
                        i { "help" }
                        span { "Learning Resources" }
                    }
                }
            }
        }
    }
}