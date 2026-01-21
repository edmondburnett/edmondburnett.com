use axum::{Router, routing::any, routing::get};
use tower_http::services::{ServeDir, ServeFile};

use crate::AppState;
use crate::handlers;

pub fn app_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::root))
        .route("/archive", get(handlers::archive))
        .route("/p/{id}", get(handlers::post))
        .route_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        // .route_service("/favicon-16x16.png", ServeFile::new("static/favicon-16x16.png"))
        // .route_service("/favicon-32x32.png", ServeFile::new("static/favicon-32x32.png"))
        // .route_service("/favicon-48x48.png", ServeFile::new("static/favicon-48x48.png"))
        // .route_service("/apple-touch-icon.png", ServeFile::new("static/apple-touch-icon.png"))
        .route_service("/robots.txt", ServeFile::new("static/robots.txt"))
        .nest_service("/static", ServeDir::new("static").fallback(any(handlers::handler_404)))
        .route("/{page_name}", get(handlers::page))
        .fallback(handlers::handler_404)
}
