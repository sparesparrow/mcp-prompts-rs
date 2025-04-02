use mcp_prompts_rs::storage::{FileSystemStorage, PromptStorage};
use mcp_prompts_rs::create_app;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use clap::Parser;
use std::sync::Arc;

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
    let args = Cli::parse();
    println!("Starting MCP Prompts Server: {:#?}", args);

    // Initialize storage based on args
    let storage: Arc<dyn PromptStorage> = match args.storage.as_str() {
        "filesystem" => {
            println!("Using filesystem storage at: {}", args.prompt_dir);
            Arc::new(FileSystemStorage::new(args.prompt_dir))
        }
        "postgres" => {
            // Placeholder for PostgreSQL storage initialization
            panic!("PostgreSQL storage not yet implemented!");
            // let db_url = args.db_url.expect("db_url is required for postgres storage");
            // Initialize and return Arc<PostgresStorage>
        }
        _ => {
            panic!("Unsupported storage type: {}", args.storage);
        }
    };

    // Placeholder for MCP server initialization using rmcp library
    // let mcp_config = McpServerConfig { /* configuration parameters */ };
    // let mcp_server = McpServer::new(mcp_config).await.unwrap();

    println!("Listening on http://127.0.0.1:{}", args.port);

    HttpServer::new(move || {
        // Clone storage Arc for each worker thread
        let storage_clone = storage.clone();
        create_app(storage_clone) // Pass storage to the app factory
    })
    .bind(("127.0.0.1", args.port))?
    .run()
    .await
}
