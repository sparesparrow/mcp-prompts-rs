pub mod storage;

use serde::{Serialize, Deserialize};
use storage::PromptStorage;
use std::sync::Arc;
use rmcp::model::resource::{Resource, ResourceList};
use rmcp::model::prompt::{Prompt as McpPrompt, PromptList, PromptCapabilities, GetPromptParams};
use rmcp::model::capabilities::{ServerCapabilities, TransportType};
use rmcp::server::{ServerHandler, ServerRequest, ServerResponse, ServerError, CreateParams, DeleteParams, UpdateParams};
use async_trait::async_trait;
use anyhow::{Result, Context};
use tera::{Tera, Context as TeraContext};
use std::collections::HashMap;
use tracing::{debug, error, info, instrument, warn};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Prompt {
    pub id: String,
    pub content: String,
    pub category: Option<String>,
    pub variables: Option<Vec<String>>,
    pub description: Option<String>,
}

/// Converts our internal Prompt struct to rmcp::model::prompt::Prompt.
fn to_mcp_prompt(prompt: Prompt) -> McpPrompt {
    McpPrompt {
        id: prompt.id,
        description: prompt.description,
        content: prompt.content,
        // Note: McpPrompt might have an `arguments` field. We are not mapping it here.
    }
}

/// Converts rmcp::model::prompt::Prompt to our internal Prompt struct.
/// Assumes the input McpPrompt has necessary fields (id, content).
fn from_mcp_prompt(mcp_prompt: McpPrompt) -> Prompt {
    Prompt {
        id: mcp_prompt.id, // ID must be present for saving
        description: mcp_prompt.description,
        content: mcp_prompt.content,
        category: None, // Not part of standard McpPrompt, set default or handle differently
        variables: None, // Not part of standard McpPrompt, set default or handle differently
    }
}

/// Handler struct for the MCP server
#[derive(Clone)]
pub struct McpPromptServerHandler {
    storage: Arc<dyn PromptStorage>,
}

impl McpPromptServerHandler {
    pub fn new(storage: Arc<dyn PromptStorage>) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl ServerHandler for McpPromptServerHandler {
    #[instrument(skip(self, _req), name = "get_capabilities")]
    async fn get_capabilities(&self, _req: ServerRequest<()>) -> Result<ServerCapabilities> {
        info!("Reporting capabilities");
        Ok(ServerCapabilities {
            server_name: "mcp-prompts-rs".to_string(),
            prompt_capabilities: Some(PromptCapabilities {}),
            transports: vec![TransportType::SseServer]
        })
    }

    #[instrument(skip(self, _req), name = "list_prompts")]
    async fn list_prompts(&self, _req: ServerRequest<()>) -> Result<PromptList> {
        debug!("Listing all prompts");
        let prompts = self.storage.list_prompts().await?;
        info!(count = prompts.len(), "Found prompts");
        let mcp_prompts: Vec<McpPrompt> = prompts.into_iter().map(to_mcp_prompt).collect();
        Ok(PromptList { prompts: mcp_prompts })
    }

    #[instrument(skip(self, req), name = "get_prompt", fields(prompt_id = %req.params.id))]
    async fn get_prompt(&self, req: ServerRequest<GetPromptParams>) -> Result<McpPrompt> {
        let params = req.params;
        let id = params.id;
        let arguments: HashMap<String, serde_json::Value> = params.arguments.unwrap_or_default();
        debug!(arguments = ?arguments, "Getting prompt");

        match self.storage.get_prompt(&id).await? {
            Some(stored_prompt) => {
                let mut mcp_prompt = to_mcp_prompt(stored_prompt.clone());

                if !arguments.is_empty() && !stored_prompt.content.is_empty() {
                    let tera_ctx = TeraContext::from_serialize(&arguments)
                        .context("Failed to create Tera context from arguments")?;

                    match Tera::one_off(&stored_prompt.content, &tera_ctx, false) {
                        Ok(rendered_content) => {
                            mcp_prompt.content = rendered_content;
                        }
                        Err(e) => {
                            warn!(error = %e, "Tera template rendering error. Returning raw content.");
                        }
                    }
                }
                info!("Prompt retrieved successfully");
                Ok(mcp_prompt)
            },
            None => {
                warn!("Prompt not found");
                Err(ServerError::resource_not_found(format!("Prompt '{}' not found", id)).into())
            }
        }
    }

    #[instrument(skip(self, _req), name = "list_resources")]
    async fn list_resources(&self, _req: ServerRequest<Option<String>>) -> Result<ResourceList> {
        warn!("list_resources not implemented");
        Ok(ResourceList { resources: vec![] })
    }

    #[instrument(skip(self, req), name = "get_resource", fields(resource_id = %req.params))]
    async fn get_resource(&self, req: ServerRequest<String>) -> Result<Resource> {
        warn!("get_resource not implemented");
        let id = req.params;
        Err(ServerError::resource_not_found(format!("Resource '{}' not found (or not supported)", id)).into())
    }

    // --- Prompt Modification Methods ---

    #[instrument(skip(self, req), name = "create_prompt", fields(prompt_id = %req.params.item.id))]
    async fn create_prompt(&self, req: ServerRequest<CreateParams<McpPrompt>>) -> Result<McpPrompt> {
        let mcp_prompt = req.params.item;
        let prompt_id = mcp_prompt.id.clone();
        debug!(prompt = ?mcp_prompt, "Attempting to create prompt");

        // Convert to internal format for saving
        let internal_prompt = from_mcp_prompt(mcp_prompt);

        // Check if prompt already exists (optional, depends on desired create behavior)
        if self.storage.get_prompt(&prompt_id).await?.is_some() {
            warn!("Attempted to create existing prompt");
            return Err(ServerError::invalid_request(format!("Prompt '{}' already exists. Use update_prompt instead.", prompt_id)).into());
        }

        self.storage.save_prompt(&internal_prompt).await
            .with_context(|| format!("Failed to create prompt '{}'", prompt_id))?;

        // Return the saved prompt (converted back to McpPrompt)
        info!("Prompt created successfully");
        Ok(to_mcp_prompt(internal_prompt))
    }

    #[instrument(skip(self, req), name = "update_prompt", fields(prompt_id = %req.params.item.id))]
    async fn update_prompt(&self, req: ServerRequest<UpdateParams<McpPrompt>>) -> Result<McpPrompt> {
        let mcp_prompt = req.params.item;
        let prompt_id = mcp_prompt.id.clone();
        debug!(prompt = ?mcp_prompt, "Attempting to update prompt");

        // Convert to internal format for saving
        let internal_prompt = from_mcp_prompt(mcp_prompt);

        // Check if prompt exists before updating
        if self.storage.get_prompt(&prompt_id).await?.is_none() {
            warn!("Attempted to update non-existent prompt");
            return Err(ServerError::resource_not_found(format!("Prompt '{}' not found. Cannot update.", prompt_id)).into());
        }

        self.storage.save_prompt(&internal_prompt).await
            .with_context(|| format!("Failed to update prompt '{}'", prompt_id))?;

        // Return the updated prompt
        info!("Prompt updated successfully");
        Ok(to_mcp_prompt(internal_prompt))
    }

    #[instrument(skip(self, req), name = "delete_prompt", fields(prompt_id = %req.params.id))]
    async fn delete_prompt(&self, req: ServerRequest<DeleteParams>) -> Result<ServerResponse<()>> {
        let id = req.params.id;
        debug!("Attempting to delete prompt");

        // Check if prompt exists before deleting
        if self.storage.get_prompt(&id).await?.is_none() {
            warn!("Attempted to delete non-existent prompt");
            return Err(ServerError::resource_not_found(format!("Prompt '{}' not found. Cannot delete.", id)).into());
        }

        self.storage.delete_prompt(&id).await
            .with_context(|| format!("Failed to delete prompt '{}'", id))?;

        info!("Prompt deleted successfully");
        Ok(ServerResponse { result: () }) // Return empty success response
    }

    // TODO: Implement other ServerHandler methods as needed (e.g., for tools)
} 