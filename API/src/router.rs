use axum::{extract::State, routing::get, Router, response::IntoResponse, http::StatusCode, Json};
use axum_extra::routing::SpaRouter;
use openai_api::{Client};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct PromptReq {
    message: String,
}

#[derive(Clone)]
pub struct AppState {
    client: Client,
}

pub fn handle_router(api_key: String, static_folder: PathBuf) -> Router {
    let prompt_client = AppState {
        client: Client::new(&api_key),
    };

    let spa = SpaRouter::new("/", static_folder);

    Router::new()
        .merge(spa)
        .route("/api/prompt", get(generate_prompt))
        .with_state(prompt_client)
}
pub async fn generate_prompt(State(state): State<AppState>) -> impl IntoResponse {

    let prompt = "Generate a random name.
    
    Example:
    Name: John Doe
    
    Name:";

    match state.client.complete_prompt_sync(prompt) {
        Ok(result) => (StatusCode::OK, Json(result.choices[0].text.clone())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!(":( There was an error: {err}")).into_response()
    }

    
}
