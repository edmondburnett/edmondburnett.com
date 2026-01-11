use crate::post::Post;
use askama::Template;
use askama_web::WebTemplate;
use axum::{extract::Path, http::StatusCode, response::IntoResponse};

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

#[derive(Template, WebTemplate)]
#[template(path = "post.html.j2")]
#[allow(dead_code)]
struct PostTemplate {
    id: String,
    title: String,
    description: String,
    tags: Vec<String>,
    html: String,
}

pub async fn post(Path(id): Path<String>) -> impl IntoResponse {
    let post = match Post::new(&id) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error loading post: {:?}", e);
            return StatusCode::NOT_FOUND.into_response();
        }
    };
    println!("Got post {:?}", post);

    PostTemplate {
        id: post.id,
        title: post.title,
        description: post.description,
        tags: post.tags,
        html: post.html,
    }
    .into_response()
}
