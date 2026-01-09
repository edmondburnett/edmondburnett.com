use askama::Template;
use askama_web::WebTemplate;
use axum::{http::StatusCode, response::IntoResponse};

#[derive(Template, WebTemplate)]
#[template(path = "root.html.j2")]
#[allow(dead_code)]
pub struct RootTemplate<'a> {
    name: &'a str,
}

pub async fn root() -> RootTemplate<'static> {
    RootTemplate { name: "edmond" }
}

#[derive(Template, WebTemplate)]
#[template(path = "404.html.j2")]
struct NotFoundTemplate;

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, NotFoundTemplate)
}
