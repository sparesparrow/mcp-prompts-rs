use mcp_prompts_rs::storage::{FileSystemStorage, PromptStorage};
use mcp_prompts_rs::storage::postgres::PostgresStorage;
use mcp_prompts_rs::McpPromptServerHandler;
use clap::Parser;
use std::sync::Arc;
use rmcp::transport::sse_server::SseServerTransport;
use rmcp::server::Server;
use tracing_subscriber::{fmt, EnvFilter};

// If available, import the rmcp crate for MCP server functionality
// use rmcp::server::{McpServer, McpServerConfig};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Port to run the server on
    #[arg(long, default_value_t = 8080)]
    port: u16,

    /// Storage backend type (filesystem, postgres)
    #[arg(long, default_value = "filesystem")]
    storage: String,

    /// PostgreSQL connection URL
    #[arg(long)]
    db_url: Option<String>,

    /// Directory for prompt storage (when using filesystem storage)
    #[arg(long, default_value = "./prompts")]
    prompt_dir: String,
}

async fn list_prompts() -> impl Responder {
    HttpResponse::Ok().body("Listing prompts...")
}

async fn get_prompt(web::Path(id): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Getting prompt with id: {}", id))
}

async fn create_prompt(body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("Created prompt: {}", body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing subscriber
    // Use `RUST_LOG=info` (or debug, trace, etc.) to control log level
    // Example: RUST_LOG=mcp_prompts_rs=debug,rmcp=info cargo run
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Cli::parse();
    tracing::info!(args = ?args, "Starting MCP Prompts Server");

    // Initialize storage based on args
    let storage: Arc<dyn PromptStorage> = match args.storage.as_str() {
        "filesystem" => {
            tracing::info!(path = %args.prompt_dir, "Using filesystem storage");
            Arc::new(FileSystemStorage::new(args.prompt_dir))
        }
        "postgres" => {
            let db_url = args.db_url.clone().expect("--db-url is required for postgres storage");
            tracing::info!(url = %db_url, "Using PostgreSQL storage");
            let pg_storage = PostgresStorage::new(&db_url).await
                .expect("Failed to connect to PostgreSQL");
            // Initialize schema (consider making this optional via CLI arg)
            pg_storage.init_schema().await.expect("Failed to initialize DB schema");
            tracing::info!("Database schema initialized (if not exists)");
            Arc::new(pg_storage)
        }
        _ => {
            tracing::error!(storage_type = %args.storage, "Unsupported storage type specified");
            panic!("Unsupported storage type: {}", args.storage);
        }
    };

    // Placeholder for MCP server initialization using rmcp library
    // let mcp_config = McpServerConfig { /* configuration parameters */ };
    // let mcp_server = McpServer::new(mcp_config).await.unwrap();

    // --- Initialize MCP Server ---
    let handler = McpPromptServerHandler::new(storage);
    let server = Server::new(handler);

    // --- Start SSE Transport ---
    let addr = format!("127.0.0.1:{}", args.port);
    tracing::info!(address = %addr, "Starting MCP SSE server");
    SseServerTransport::bind(&addr)
        .await?
        .serve(server)
        .await
}
