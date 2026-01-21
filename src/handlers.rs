use crate::AppState;
use crate::page::Page;
use crate::post::Post;
use askama::Template;
use askama_web::WebTemplate;
use axum::{extract::Path, extract::State, http::StatusCode, response::Html, response::IntoResponse};

#[derive(Template, WebTemplate)]
#[template(path = "404.html.j2")]
struct NotFoundTemplate;

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, NotFoundTemplate)
}

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
#[template(path = "post.html.j2")]
#[allow(dead_code)]
struct PostTemplate {
    post: Post,
}

pub async fn post(State(_state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let post = match Post::new(&id) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!(post_id = %id, error = %e, "Failed to load post");
            return (StatusCode::NOT_FOUND, NotFoundTemplate).into_response();
        }
    };

    if post.draft {
        tracing::warn!(post_id = %id, "Attempt to access draft post");
        return (StatusCode::NOT_FOUND, NotFoundTemplate).into_response();
    }

    PostTemplate { post }.into_response()
}

#[derive(Template, WebTemplate)]
#[template(path = "page.html.j2")]
#[allow(dead_code)]
struct PageTemplate {
    page: Page,
}

pub async fn page(State(_state): State<AppState>, Path(page_name): Path<String>) -> impl IntoResponse {
    let page = match Page::new(&page_name) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!(page_name = %page_name, error = %e, "Failed to load page");
            return (StatusCode::NOT_FOUND, NotFoundTemplate).into_response();
        }
    };

    PageTemplate { page }.into_response()
}
