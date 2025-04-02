pub mod storage;
pub mod models;

// Comment out rmcp server/model imports until we figure out the correct structure
// use rmcp::model::{ServerCapabilities, Prompt as McpPrompt, Resource};
// use rmcp::{
//     Server, ServerHandler, ServerRequest, ServerResponse, ServerError,
//     CreateParams, UpdateParams, DeleteParams,
// };
use std::sync::Arc;
use crate::storage::PromptStorage;

// Keep conversion functions commented out for now as they depend on MCP types
/*
/// Converts our internal Prompt struct to rmcp::model::prompt::Prompt.
fn to_mcp_prompt(prompt: InternalPrompt) -> McpPrompt {
    McpPrompt {
        // ... conversion logic ...
    }
}

/// Converts rmcp::model::prompt::Prompt to our internal Prompt struct.
fn from_mcp_prompt(mcp_prompt: McpPrompt) -> Result<InternalPrompt> {
    // ... conversion logic ...
}
*/

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

// Temporarily comment out the entire ServerHandler implementation
/*
#[async_trait]
impl ServerHandler for McpPromptServerHandler {
    #[instrument(skip(self, _req), name = "get_capabilities")]
    async fn get_capabilities(&self, _req: ServerRequest<()>) -> Result<ServerCapabilities> {
        info!("Reporting capabilities");
        Ok(ServerCapabilities {
            server_name: "mcp-prompts-rs".to_string(),
            ..Default::default()
        })
    }

    #[instrument(skip(self, _req), name = "list_prompts")]
    async fn list_prompts(&self, _req: ServerRequest<()>) -> Result<Vec<McpPrompt>> {
        debug!("Listing all prompts");
        let internal_prompts = self.storage.list_prompts().await?;
        info!(count = internal_prompts.len(), "Found prompts");
        let mcp_prompts: Vec<McpPrompt> = internal_prompts.into_iter().map(to_mcp_prompt).collect();
        Ok(mcp_prompts)
    }

    #[instrument(skip(self, req), name = "get_prompt")]
    async fn get_prompt(&self, req: ServerRequest<String>) -> Result<McpPrompt> {
        let id_str = req.params;
        debug!("Getting prompt");

        let id_uuid = Uuid::parse_str(&id_str)
            .map_err(|_| ServerError::invalid_request(format!("Invalid prompt ID format: {}", id_str)))?;

        match self.storage.get_prompt(&id_uuid).await? {
            Some(stored_prompt) => {
                let mcp_prompt = to_mcp_prompt(stored_prompt.clone());
                info!("Prompt retrieved successfully");
                Ok(mcp_prompt)
            }
            None => {
                warn!("Prompt not found");
                Err(ServerError::resource_not_found(format!("Prompt '{}' not found", id_str)).into())
            }
        }
    }

    #[instrument(skip(self, _req), name = "list_resources")]
    async fn list_resources(&self, _req: ServerRequest<Option<String>>) -> Result<Vec<Resource>> {
        warn!("list_resources not implemented");
        Ok(vec![])
    }

    #[instrument(skip(self, req), name = "get_resource", fields(resource_id = %req.params))]
    async fn get_resource(&self, req: ServerRequest<String>) -> Result<Resource> {
        warn!("get_resource not implemented");
        let id = req.params;
        Err(ServerError::resource_not_found(format!(
            "Resource '{}' not found (or not supported)",
            id
        ))
        .into())
    }

    // --- Prompt Modification Methods ---

    #[instrument(skip(self, req), name = "create_prompt")]
    async fn create_prompt(
        &self,
        req: ServerRequest<CreateParams<McpPrompt>>,
    ) -> Result<McpPrompt> {
        let mcp_prompt = req.params.item;
        let prompt_id_str = mcp_prompt.id.clone();
        debug!(prompt = ?mcp_prompt, "Attempting to create prompt");
        let internal_prompt = from_mcp_prompt(mcp_prompt)?;
        let prompt_uuid = internal_prompt.id;
        if self.storage.get_prompt(&prompt_uuid).await?.is_some() {
            warn!("Attempted to create existing prompt");
            return Err(ServerError::invalid_request(format!(
                "Prompt '{}' already exists. Use update_prompt instead.",
                prompt_id_str
            ))
            .into());
        }
        self.storage
            .save_prompt(&internal_prompt)
            .await
            .with_context(|| format!("Failed to create prompt '{}'", prompt_id_str))?;
        info!("Prompt created successfully");
        Ok(to_mcp_prompt(internal_prompt))
    }

    #[instrument(skip(self, req), name = "update_prompt")]
    async fn update_prompt(
        &self,
        req: ServerRequest<UpdateParams<McpPrompt>>,
    ) -> Result<McpPrompt> {
        let mcp_prompt = req.params.item;
        let prompt_id_str = mcp_prompt.id.clone();
        debug!(prompt = ?mcp_prompt, "Attempting to update prompt");
        let internal_prompt = from_mcp_prompt(mcp_prompt)?;
        let prompt_uuid = internal_prompt.id;
        if self.storage.get_prompt(&prompt_uuid).await?.is_none() {
            warn!("Attempted to update non-existent prompt");
            return Err(ServerError::resource_not_found(format!(
                "Prompt '{}' not found. Cannot update.",
                prompt_id_str
            ))
            .into());
        }
        self.storage
            .save_prompt(&internal_prompt)
            .await
            .with_context(|| format!("Failed to update prompt '{}'", prompt_id_str))?;
        info!("Prompt updated successfully");
        Ok(to_mcp_prompt(internal_prompt))
    }

    #[instrument(skip(self, req), name = "delete_prompt")]
    async fn delete_prompt(&self, req: ServerRequest<DeleteParams>) -> Result<ServerResponse<()>> {
        let id_str = req.params.id;
        debug!("Attempting to delete prompt");
        let id_uuid = Uuid::parse_str(&id_str)
            .map_err(|_| ServerError::invalid_request(format!("Invalid prompt ID format: {}", id_str)))?;
        let deleted = self.storage
            .delete_prompt(&id_uuid)
            .await
            .with_context(|| format!("Failed to delete prompt '{}'", id_str))?;
        if deleted {
            info!("Prompt deleted successfully");
            Ok(ServerResponse { result: () })
        } else {
            warn!("Attempted to delete non-existent prompt");
            Err(ServerError::resource_not_found(format!(
                "Prompt '{}' not found. Cannot delete.",
                id_str
            ))
            .into())
        }
    }

    // TODO: Implement other ServerHandler methods as needed (e.g., for tools)
}
*/
