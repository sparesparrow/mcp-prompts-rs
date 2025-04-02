pub mod postgres;

use crate::Prompt;
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Trait defining the storage operations for prompts.
#[async_trait]
pub trait PromptStorage: Send + Sync + 'static { // Require Send + Sync + 'static
    async fn list_prompts(&self) -> Result<Vec<Prompt>>;
    async fn get_prompt(&self, id: &str) -> Result<Option<Prompt>>; // Return Option<Prompt>
    async fn save_prompt(&self, prompt: &Prompt) -> Result<()>;
    async fn delete_prompt(&self, id: &str) -> Result<()>;
}

/// Filesystem storage implementation.
#[derive(Clone)] // Add Clone
pub struct FileSystemStorage {
    base_path: PathBuf,
}

impl FileSystemStorage {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        FileSystemStorage {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    fn get_prompt_path(&self, id: &str) -> PathBuf {
        self.base_path.join(format!("{}.json", id))
    }
}

#[async_trait]
impl PromptStorage for FileSystemStorage {
    async fn list_prompts(&self) -> Result<Vec<Prompt>> {
        let mut prompts = Vec::new();
        let mut dir = fs::read_dir(&self.base_path).await
            .with_context(|| format!("Failed to read prompt directory: {:?}", self.base_path))?;

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Ok(Some(prompt)) = self.get_prompt(stem).await {
                        prompts.push(prompt);
                    }
                }
            }
        }
        Ok(prompts)
    }

    async fn get_prompt(&self, id: &str) -> Result<Option<Prompt>> {
        let path = self.get_prompt_path(id);
        if !path.exists() {
            return Ok(None);
        }
        let mut file = fs::File::open(&path).await
            .with_context(|| format!("Failed to open prompt file: {:?}", path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await
            .with_context(|| format!("Failed to read prompt file: {:?}", path))?;
        let prompt: Prompt = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to deserialize prompt from file: {:?}", path))?;
        Ok(Some(prompt))
    }

    async fn save_prompt(&self, prompt: &Prompt) -> Result<()> {
        let path = self.get_prompt_path(&prompt.id);
        fs::create_dir_all(&self.base_path).await
            .with_context(|| format!("Failed to create prompt directory: {:?}", self.base_path))?;
        let contents = serde_json::to_string_pretty(prompt)
            .with_context(|| format!("Failed to serialize prompt: {}", prompt.id))?;
        let mut file = fs::File::create(&path).await
            .with_context(|| format!("Failed to create prompt file: {:?}", path))?;
        file.write_all(contents.as_bytes()).await
            .with_context(|| format!("Failed to write to prompt file: {:?}", path))?;
        Ok(())
    }

    async fn delete_prompt(&self, id: &str) -> Result<()> {
        let path = self.get_prompt_path(id);
        if path.exists() {
            fs::remove_file(&path).await
                .with_context(|| format!("Failed to delete prompt file: {:?}", path))?;
        }
        Ok(())
    }
} 