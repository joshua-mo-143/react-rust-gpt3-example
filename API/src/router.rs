use axum::{extract::State, routing::get, Json, Router};
use axum_extra::routing::SpaRouter;
use openai_api::{Client, api::{CompletionArgs, Engine}};
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

#[axum_macros::debug_handler]
pub async fn generate_prompt(State(state): State<AppState>) -> axum::Json<String> {
    let args = CompletionArgs::builder()
        .prompt(
            "Generate a pair of colors like below: 
    Example: 
    Color 1: #000000 (black) 
    Color 2: #FFFFFF (white) 
        
    Color 1: 
    Color 2:"
        )
        .max_tokens(200)
        .temperature(0.7)
        .top_p(0.9)
        .stop(vec!["\n".into()]);

    match state.client.complete_prompt(args).await {
        Ok(res) =>     Json(
        res.choices[0].text
    ),
        Err(_) => Json("There was an error!".to_string())
    }


}
