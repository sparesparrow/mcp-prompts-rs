# mcp-prompts-rs

A Rust-based server for managing AI prompts using the Model Context Protocol (MCP).

## Overview

mcp-prompts-rs is a Rust implementation of a prompt management server that adheres to the Model Context Protocol (MCP), an open standard for connecting AI applications to data sources and tools. This project is a Rust rewrite of the original [mcp-prompts](https://github.com/sparesparrow/mcp-prompts) TypeScript implementation.

The server provides functionality for storing, retrieving, and managing AI prompts with support for template variables, categorization, and multiple storage backends.

## Features

- **Prompt Management**: Create, retrieve, update, and delete prompts with categorization
- **Template Support**: Create prompts with variables for runtime customization
- **Storage Backends**: Support for file system and PostgreSQL storage options
- **API**: RESTful endpoints with Server-Sent Events (SSE) for real-time updates
- **MCP Integration**: Implements the Model Context Protocol for seamless integration with AI assistants like Claude
- **Project Orchestration**: Tools for automating software project creation using templates
- **Deployment**: Docker support and health check endpoints

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (Rust's package manager)
- Optional: PostgreSQL for database storage

### Setup

1. Clone the repository:

```bash
git clone https://github.com/your-username/mcp-prompts-rs.git
cd mcp-prompts-rs
```

2. Build the project:

```bash
cargo build
```

## Usage

### Running the Server

Start the server with default settings:

```bash
cargo run
```

With custom configuration:

```bash
cargo run -- --port 3000 --storage filesystem
```

### CLI Options

- `--port <PORT>`: Set the server port (default: 8080)
- `--storage <TYPE>`: Choose storage backend (filesystem, postgres)
- `--db-url <URL>`: PostgreSQL connection string (when using postgres storage)
- `--prompt-dir <DIR>`: Directory for prompt storage (when using filesystem storage)

## Integration with Claude

To integrate with Claude Desktop:

1. Open Claude Desktop
2. Go to Settings → Developer → Edit Config
3. Add the following to your configuration:

```json
{
  "mcp": {
    "servers": [
      {
        "name": "mcp-prompts-rs",
        "url": "http://localhost:8080"
      }
    ]
  }
}
```

## API Endpoints

### Prompts

- `GET /prompts`: List all prompts
- `GET /prompts/:id`: Get a specific prompt
- `POST /prompts`: Create a new prompt (requires at least `name` and `content` fields)
- `PUT /prompts/:id`: Update an existing prompt (requires at least `name` and `content` fields)
- `DELETE /prompts/:id`: Delete a prompt

#### Example Usage (with curl)

- **List all prompts:**
  ```bash
  curl -X GET http://localhost:3000/prompts
  ```
- **Create a prompt:**
  ```bash
  curl -X POST http://localhost:3000/prompts \
    -H "Content-Type: application/json" \
    -d '{"name": "test-prompt", "title": "Test Prompt", "content": "This is a test prompt."}'
  ```
- **Get a prompt by ID:**
  ```bash
  curl -X GET http://localhost:3000/prompts/<id>
  ```
- **Update a prompt:**
  ```bash
  curl -X PUT http://localhost:3000/prompts/<id> \
    -H "Content-Type: application/json" \
    -d '{"name": "test-prompt", "title": "Updated Test Prompt", "content": "This is an updated test prompt."}'
  ```
- **Delete a prompt:**
  ```bash
  curl -X DELETE http://localhost:3000/prompts/<id>
  ```

### SSE

- `GET /events`: Server-Sent Events endpoint for real-time updates

## Development

### Project Structure

- `src/main.rs`: Entry point and server setup
- `src/prompt/`: Prompt models and logic
- `src/storage/`: Storage backend implementations
- `src/api/`: API endpoint handlers
- `src/template/`: Template processing utilities

### Building from Source

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

## Docker Support

Build and run with Docker:

```bash
docker build -t mcp-prompts-rs .
docker run -p 8080:8080 mcp-prompts-rs
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Original [mcp-prompts](https://github.com/sparesparrow/mcp-prompts) TypeScript project
- [Model Context Protocol](https://github.com/modelcontextprotocol)
- [Rust SDK for MCP](https://github.com/modelcontextprotocol/rust-sdk)
