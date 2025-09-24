use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use askama::Template;

mod controllers;
/// A simple Spin HTTP component.
#[http_component]
fn handle_ai_bookmark_app(req: Request) -> anyhow::Result<impl IntoResponse, anyhow::Error> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let template = HelloTemplate {
        name: "World".to_string(),
    };

    let body = template.render()?;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(body)
        .build())
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}
