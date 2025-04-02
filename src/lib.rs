pub mod storage;

use actix_web::{web, App, HttpResponse, Responder, http::StatusCode};
use serde::{Serialize, Deserialize};
use storage::PromptStorage;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Prompt {
    pub id: String,
    pub content: String,
    pub category: Option<String>,
    pub variables: Option<Vec<String>>,
}

/// Endpoint to list prompts
pub async fn list_prompts(storage: web::Data<Arc<dyn PromptStorage>>) -> impl Responder {
    match storage.list_prompts().await {
        Ok(prompts) => HttpResponse::Ok().json(prompts),
        Err(e) => {
            eprintln!("Error listing prompts: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to list prompts")
        }
    }
}

/// Endpoint to get a prompt by id
pub async fn get_prompt(
    storage: web::Data<Arc<dyn PromptStorage>>,
    id: web::Path<String>,
) -> impl Responder {
    match storage.get_prompt(&id).await {
        Ok(Some(prompt)) => HttpResponse::Ok().json(prompt),
        Ok(None) => HttpResponse::NotFound().body(format!("Prompt with id '{}' not found", id)),
        Err(e) => {
            eprintln!("Error getting prompt {}: {:?}", id, e);
            HttpResponse::InternalServerError().body(format!("Failed to get prompt {}", id))
        }
    }
}

/// Endpoint to create or update a prompt
pub async fn save_prompt(
    storage: web::Data<Arc<dyn PromptStorage>>,
    prompt: web::Json<Prompt>,
) -> impl Responder {
    let prompt_data = prompt.into_inner();
    match storage.save_prompt(&prompt_data).await {
        Ok(_) => HttpResponse::Ok().json(prompt_data),
        Err(e) => {
            eprintln!("Error saving prompt {}: {:?}", prompt_data.id, e);
            HttpResponse::InternalServerError().body(format!("Failed to save prompt {}", prompt_data.id))
        }
    }
}

/// Endpoint to delete a prompt
pub async fn delete_prompt(
    storage: web::Data<Arc<dyn PromptStorage>>,
    id: web::Path<String>,
) -> impl Responder {
    match storage.delete_prompt(&id).await {
        Ok(_) => HttpResponse::Ok().body(format!("Prompt '{}' deleted successfully", id)),
        Err(e) => {
            eprintln!("Error deleting prompt {}: {:?}", id, e);
            HttpResponse::InternalServerError().body(format!("Failed to delete prompt {}", id))
        }
    }
}

/// Factory function to construct the Actix Web application
pub fn create_app(storage: Arc<dyn PromptStorage>) -> App<actix_web::dev::AppEntry> {
    App::new()
        .app_data(web::Data::new(storage))
        .route("/prompts", web::get().to(list_prompts))
        .route("/prompts/{id}", web::get().to(get_prompt))
        .route("/prompts", web::post().to(save_prompt))
        .route("/prompts/{id}", web::put().to(save_prompt))
        .route("/prompts/{id}", web::delete().to(delete_prompt))
} 