use spin_sdk::http::{IntoResponse, Request, Response, Router};
use spin_sdk::http_component;
use askama::Template;

mod controllers;
mod models;

/// A simple Spin HTTP component.
#[http_component]
fn handle_ai_bookmark_app(req: Request) -> anyhow::Result<impl IntoResponse, anyhow::Error> {
    let mut router = Router::default();

    router.get("/", controllers::home::handle_home);
    router.get("/about", controllers::home::handle_about);
    router.post("/add-name", controllers::home::handle_add_name);
    router.post("/bookmark", controllers::bookmarks::handle_add_bookmark);
    router.get("/bookmark", controllers::bookmarks::handle_get_all_bookmarks);
    
    router.any("/*", controllers::home::handle_not_found);

    Ok(router.handle(req))
}
