use crate::Prompt;
use super::PromptStorage;
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use std::sync::Arc;

// Define a struct that maps to the database table row
// We derive FromRow to automatically map PgRow to this struct
#[derive(FromRow, Debug)]
struct PromptRow {
    id: String, // Assuming TEXT or VARCHAR in DB
    content: String,
    category: Option<String>,
    variables: Option<serde_json::Value>, // Assuming JSON or JSONB in DB
    description: Option<String>,
    // Add timestamp fields if they exist in the DB
    // created_at: chrono::DateTime<chrono::Utc>,
    // updated_at: chrono::DateTime<chrono::Utc>,
}

// Helper to convert from DB row struct to our application Prompt struct
impl From<PromptRow> for Prompt {
    fn from(row: PromptRow) -> Self {
        Prompt {
            id: row.id,
            content: row.content,
            category: row.category,
            variables: row.variables.and_then(|v| serde_json::from_value(v).ok()),
            description: row.description,
        }
    }
}

/// PostgreSQL storage implementation.
#[derive(Clone)]
pub struct PostgresStorage {
    pool: Arc<PgPool>,
}

impl PostgresStorage {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5) // Configure pool size
            .connect(database_url)
            .await
            .with_context(|| format!("Failed to create PostgreSQL connection pool for URL: {}", database_url))?;
        Ok(PostgresStorage { pool: Arc::new(pool) })
    }

    /// Initializes the database schema if it doesn't exist.
    pub async fn init_schema(&self) -> Result<()> {
        // Use SQLx's query! macro for compile-time checked SQL (optional but recommended)
        // Or use query() for runtime SQL strings.
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS prompts (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                category TEXT,
                variables JSONB,
                description TEXT,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );
            "#
        )
        .execute(&*self.pool)
        .await
        .context("Failed to initialize database schema")?;
        Ok(())
    }
}

#[async_trait]
impl PromptStorage for PostgresStorage {
    async fn list_prompts(&self) -> Result<Vec<Prompt>> {
        let rows: Vec<PromptRow> = sqlx::query_as("SELECT * FROM prompts")
            .fetch_all(&*self.pool)
            .await
            .context("Failed to fetch prompts from database")?;
        Ok(rows.into_iter().map(Prompt::from).collect())
    }

    async fn get_prompt(&self, id: &str) -> Result<Option<Prompt>> {
        let row: Option<PromptRow> = sqlx::query_as("SELECT * FROM prompts WHERE id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
            .with_context(|| format!("Failed to fetch prompt with id '{}' from database", id))?;
        Ok(row.map(Prompt::from))
    }

    async fn save_prompt(&self, prompt: &Prompt) -> Result<()> {
        // Convert variables Vec<String> to JSON for storage
        let variables_json = prompt.variables.as_ref()
            .map(|v| serde_json::to_value(v))
            .transpose()
            .context("Failed to serialize prompt variables to JSON")?;

        sqlx::query(
            r#"
            INSERT INTO prompts (id, content, category, variables, description)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                content = EXCLUDED.content,
                category = EXCLUDED.category,
                variables = EXCLUDED.variables,
                description = EXCLUDED.description,
                updated_at = NOW();
            "#
        )
        .bind(&prompt.id)
        .bind(&prompt.content)
        .bind(&prompt.category)
        .bind(&variables_json)
        .bind(&prompt.description)
        .execute(&*self.pool)
        .await
        .with_context(|| format!("Failed to save prompt with id '{}' to database", prompt.id))?;
        Ok(())
    }

    async fn delete_prompt(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM prompts WHERE id = $1")
            .bind(id)
            .execute(&*self.pool)
            .await
            .with_context(|| format!("Failed to delete prompt with id '{}' from database", id))?;
        // Consider checking rows affected if necessary
        Ok(())
    }
} 