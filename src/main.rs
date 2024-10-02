use std::{net::SocketAddrV4, sync::Arc};
use axum::{routing::{delete, get, post, put}, Router};
use maud::{html, Markup, DOCTYPE};
use sqlx::postgres::{PgPoolOptions, PgPool};

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
        .with_state(Arc::new(AppState { db: db.clone() }));

    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .map_err(|e| format!("Server failed to start. Error: {}", e))?;

    Ok(())
}

async fn hello_world() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "Hello, world!" }
                link href="https://cdn.jsdelivr.net/npm/beercss@3.7.10/dist/cdn/beer.min.css" rel="stylesheet";
                script type="module" src="https://cdn.jsdelivr.net/npm/beercss@3.7.10/dist/cdn/beer.min.js"{}
                script type="module" src="https://cdn.jsdelivr.net/npm/material-dynamic-colors@1.1.2/dist/cdn/material-dynamic-colors.min.js"{}
            }
            body."dark" {
                nav."left drawer l" {
                    header {
                        nav {
                            img."circle" src="https://rustacean.net/assets/cuddlyferris.png";
                            h6 { "Rust, Powered by Beer CSS!" }
                        }
                    }
                    a {
                        i { "home" }
                        div { "Home" }
                    }
                    a {
                        i { "search" }
                        div { "Search" }
                    }
                    div."divider";
                    label { "Label" }
                    a {
                        i { "widgets" }
                        div { "Widgets" }
                    }
                    a {
                        i { "help" }
                        div { "Help" }
                    }
                }

                nav."left m" {
                    header {
                        img."circle" src="https://www.beercss.com/favicon.png";
                    }
                    a {
                        i { "home" }
                        div { "Home" }
                    }
                    a {
                        i { "search" }
                        div { "Search" }
                    }
                    a {
                        i { "widgets" }
                        div { "Widgets" }
                    }
                }

                nav."bottom s" {
                    a {
                        i { "home" }
                        div { "Home" }
                    }
                    a {
                        i { "search" }
                        div { "Search" }
                    }
                    a {
                        i { "widgets" }
                        div { "Widgets" }
                    }
                }

                main."responsive" {
                    img."responsive round" src="https://www.beercss.com/beer-and-woman.svg";
                    h3 { "Welcome!" }
                    h5 { "Rust serves beer here!" }
                }
            }
        }
    }
}