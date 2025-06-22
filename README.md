# MCP Prompts Rust Implementation

Tento repozitář obsahuje Rust implementaci MCP Prompts serveru. Poskytuje nativní, vysoce výkonnou implementaci pro správu promptů a workflow.

## Účel

- **Nativní Implementace**: Vysoce výkonná Rust implementace MCP Prompts
- **Android Native Service**: Používá se jako nativní služba v Android aplikaci
- **Cross-platform**: Běží na Windows, Linux, macOS a Android
- **Memory Safe**: Bezpečná správa paměti díky Rust

## Funkce

- **Prompt Management**: Správa promptů a workflow
- **Template Processing**: Zpracování šablon s proměnnými
- **Storage Adapters**: Podpora pro souborový systém
- **High Performance**: Optimalizováno pro rychlost
- **Memory Efficiency**: Minimální využití paměti
- **Cross-compilation**: Podpora pro různé platformy

## Instalace

```bash
cargo install mcp-prompts-rs
```

## Použití

```bash
# Vývoj
cargo build
cargo test

# Spuštění
cargo run

# Release build
cargo build --release
```

## API

Rust implementace poskytuje stejné API jako TypeScript verze:

- Prompt CRUD operace
- Workflow execution
- Template processing
- Storage management

## Konfigurace

```toml
[storage]
type = "file"
path = "./data"

[templates]
delimiters = ["{{", "}}"]
```

## Závislosti

- `mcp-prompts-collection` - Prompt sbírka (Cargo dependency)

## Android Integrace

Tato implementace se používá jako nativní služba v Android aplikaci:

```rust
// Android AIDL interface
pub trait IMcpService {
    fn get_prompt(&self, id: String) -> Result<Prompt, Error>;
    fn execute_workflow(&self, workflow: Workflow) -> Result<WorkflowResult, Error>;
}
```

## Výkon

- **Startup time**: < 100ms
- **Memory usage**: < 10MB
- **Throughput**: > 1000 requests/second
