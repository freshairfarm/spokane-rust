use maud::{html, Markup, DOCTYPE};

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
            script src="https://unpkg.com/htmx.org@2.0.3" integrity="sha384-0895/pl2MU10Hqc6jd4RvrthNlDiE9U1tWmX7WRESftEDRosgxNsQG/Ze9YMRzHq" crossorigin="anonymous"{}
        }
    }
}

pub async fn hello_world() -> Markup {
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
                main."responsive" {
                    h3 { "Hello, world!" }
                    h5 { "Made using Rust, Maud, Axum, HTMX, and BeerCSS" }

                    button { }
                }
            }
        }
    }
}