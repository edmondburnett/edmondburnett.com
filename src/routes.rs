use axum::{Router, routing::any, routing::get};
use tower_http::services::{ServeDir, ServeFile};

use crate::AppState;
use crate::handlers;

pub fn app_routes() -> Router<AppState> {
    Router::new()
        // Routes
        .route("/", get(handlers::root))
        .route("/p/{id}", get(handlers::post))
        .route("/about", get(handlers::about))
        // Root-level static files
        .route_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        .route_service("/main_old.css", ServeFile::new("static/main_old.css"))
        // .route_service("/favicon-16x16.png", ServeFile::new("static/favicon-16x16.png"))
        // .route_service("/favicon-32x32.png", ServeFile::new("static/favicon-32x32.png"))
        // .route_service("/favicon-48x48.png", ServeFile::new("static/favicon-48x48.png"))
        // .route_service("/apple-touch-icon.png", ServeFile::new("static/apple-touch-icon.png"))
        .route_service("/robots.txt", ServeFile::new("static/robots.txt"))
        .route_service("/pgp.html", ServeFile::new("static/pgp.html"))
        .route_service("/pgp.asc", ServeFile::new("static/pgp.asc"))
        // Regular /static files
        .nest_service("/static", ServeDir::new("static").fallback(any(handlers::handler_404)))
        // 404
        .fallback(handlers::handler_404)
}
