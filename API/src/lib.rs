use shuttle_secrets::SecretStore;
use std::path::PathBuf;
use sync_wrapper::SyncWrapper;

mod router;
use router::handle_router;

#[shuttle_service::main]
async fn axum(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> shuttle_service::ShuttleAxum {
    let gpt3_token = secrets
        .get("GPT3_API_KEY")
        .expect("You need to set GPT3_API_KEY in your Secrets.toml file!");

    let router = handle_router(gpt3_token, static_folder);
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
