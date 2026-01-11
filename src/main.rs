use clap::{Parser, Subcommand};
use color_eyre::Result;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod handlers;
mod markdown;
mod post;
mod routes;

// usage:
// cargo run
// cargo run -- serve --port 8080
// cargo run -- validate (validates posts only)

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

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Validate) => {
            validate_posts()?;
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
    let app = routes::app_routes().layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

fn validate_posts() -> Result<()> {
    use post::Post;
    println!("Validating posts...");

    for entry in std::fs::read_dir("posts")? {
        let path = entry?.path();

        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        if let Some(id) = path.file_stem().and_then(|s| s.to_str()) {
            match Post::new(id) {
                Ok(_) => println!("✓ {}.md", id),
                Err(e) => eprintln!("✗ {}.md: {}", id, e),
            }
        } else {
            eprintln!("✗ {:?}: Invalid filename", path);
        }
    }

    Ok(())
}
