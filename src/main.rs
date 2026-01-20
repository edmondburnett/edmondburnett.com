use clap::{Parser, Subcommand};
use color_eyre::Result;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod handlers;
mod markdown;
mod post;
use post::Post;
mod page;
mod routes;
mod validation;

#[derive(Parser)]
#[command(name = "myapp")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Serve {
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
    Validate,
}

#[derive(Clone)]
pub struct AppState {
    posts: Arc<Vec<Post>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Validate) => {
            validation::validate_posts()?;
        }
        Some(Commands::Serve { port }) => {
            start_server(port).await?;
        }
        None => {
            // start w/default port
            start_server(3000).await?;
        }
    }

    Ok(())
}

async fn start_server(port: u16) -> Result<()> {
    let posts = Post::list()?;
    tracing::info!("Loaded {} posts.", posts.len());

    let state = AppState { posts: Arc::new(posts) };

    let app = routes::app_routes().with_state(state).layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
