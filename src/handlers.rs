use crate::AppState;
use crate::page::Page;
use crate::post::Post;
use askama::Template;
use askama_web::WebTemplate;
use axum::{extract::Path, extract::State, http::StatusCode, http::header, response::Html, response::IntoResponse};
use rss::{ChannelBuilder, ItemBuilder};

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
    posts: &'a Vec<Post>,
}

pub async fn root(State(state): State<AppState>) -> impl IntoResponse {
    let template = RootTemplate { posts: &state.posts };

    match template.render() {
        Ok(html) => Html(html),
        Err(e) => {
            tracing::error!("Template render error: {}", e);
            Html("Error rendering page".to_string())
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "archive.html.j2")]
#[allow(dead_code)]
pub struct ArchiveTemplate<'a> {
    posts: &'a Vec<Post>,
}

pub async fn archive(State(state): State<AppState>) -> impl IntoResponse {
    let template = ArchiveTemplate { posts: &state.posts };

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

pub async fn rss(State(state): State<AppState>) -> impl IntoResponse {
    let mut items: Vec<rss::Item> = Vec::new();
    let mut count = 0;

    for post in state.posts.iter() {
        count += 1;
        if count > 10 {
            break;
        }

        let item = ItemBuilder::default()
            .title(Some(post.title.clone()))
            .link(Some(format!("https://edmondburnett.com/p/{}", post.id)))
            .description(Some(post.description.clone()))
            .pub_date(Some(post.date.to_rfc2822()))
            .build();
        items.push(item);
    }

    let channel = ChannelBuilder::default()
        .title("edmondburnett.com")
        .link("https://edmondburnett.com")
        .description("A personal blog on code, infrastructure, and engineering.")
        .items(items)
        .build();

    let rss_string = channel.to_string();

    ([(header::CONTENT_TYPE, "text/xml; charset=utf-8")], rss_string)
}
