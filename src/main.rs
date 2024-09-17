use std::net::{Ipv4Addr, SocketAddrV4};
use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world));
    
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8000);

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .unwrap();

    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .unwrap();
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