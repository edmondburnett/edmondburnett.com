use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/", get(root)).layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
