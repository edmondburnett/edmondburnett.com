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
mod project;
use project::Project;
mod project_category;
mod routes;
mod tag;
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
        #[arg(short, long, default_value = "7000")]
        port: u16,
    },
    Validate,
}

#[derive(Clone)]
pub struct AppState {
    posts: Arc<Vec<Post>>,
    #[allow(dead_code)]
    projects: Arc<Vec<Project>>,
    current_env: String,
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
            start_server(7000).await?;
        }
    }

    Ok(())
}

fn envvar_or_default(key: &str, default: Option<&str>) -> String {
    std::env::var(key)
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| default.unwrap_or("").to_string())
}

async fn start_server(port: u16) -> Result<()> {
    let current_env = envvar_or_default("CURRENT_ENV", Some("local"));
    let include_drafts = current_env != "prod";

    let posts = Post::list(include_drafts)?;
    let projects = Project::list()?;
    tracing::info!("Loaded {} posts, {} projects.", posts.len(), projects.len());

    let state = AppState {
        posts: Arc::new(posts),
        projects: Arc::new(projects),
        current_env,
    };

    let app = routes::app_routes().with_state(state).layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
