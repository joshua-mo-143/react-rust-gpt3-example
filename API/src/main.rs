use shuttle_secrets::SecretStore;
use std::path::PathBuf;

mod router;
use router::handle_router;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let gpt3_token = secrets
        .get("GPT3_API_KEY")
        .expect("You need to set GPT3_API_KEY in your Secrets.toml file!");

    let router = handle_router(gpt3_token, static_folder);

    Ok(router.into())
}
