use axum::{Router, routing::any, routing::get};
use tower_http::services::ServeDir;

use crate::handlers;

pub fn app_routes() -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .nest_service("/static", ServeDir::new("static").fallback(any(handlers::handler_404)))
        .fallback(handlers::handler_404)
}
