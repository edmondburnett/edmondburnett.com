use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(root))
        .fallback(handler_404)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Template, WebTemplate)]
#[template(path = "root.html")]
#[allow(dead_code)]
struct RootTemplate<'a> {
    name: &'a str,
}

async fn root() -> RootTemplate<'static> {
    RootTemplate { name: "edmond" }
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found")
}
