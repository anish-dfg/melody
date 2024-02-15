mod app;
mod errors;
mod init;
mod launch;
mod state;

use errors::LaunchError;

#[tokio::main]
async fn main() -> Result<(), LaunchError> {
    let app = init::build_app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888")
        .await
        .map_err(|e| LaunchError::Internal(e.to_string()))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| LaunchError::Internal(e.to_string()))?;

    Ok(())
}
