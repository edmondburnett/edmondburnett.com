use crate::AppState;
use crate::post::Post;
use askama::Template;
use askama_web::WebTemplate;
use axum::{extract::Path, extract::State, http::StatusCode, response::Html, response::IntoResponse};

#[derive(Template, WebTemplate)]
#[template(path = "root.html.j2")]
#[allow(dead_code)]
pub struct RootTemplate<'a> {
    name: &'a str,
    posts: &'a Vec<Post>,
}

pub async fn root(State(state): State<AppState>) -> impl IntoResponse {
    let template = RootTemplate {
        name: "edmond",
        posts: &state.posts,
    };

    match template.render() {
        Ok(html) => Html(html),
        Err(e) => {
            tracing::error!("Template render error: {}", e);
            Html("Error rendering page".to_string())
        }
    }
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

pub async fn post(State(_state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let post = match Post::new(&id) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!(post_id = %id, error = %e, "Failed to load post");
            return (StatusCode::NOT_FOUND, NotFoundTemplate).into_response();
        }
    };

    PostTemplate {
        id: post.id,
        title: post.title,
        description: post.description,
        tags: post.tags,
        html: post.html,
    }
    .into_response()
}
