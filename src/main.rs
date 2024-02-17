mod app;
mod init;
mod launch;
mod state;

#[tokio::main]
async fn main() {
    let app = init::build_app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888")
        .await
        .expect("could not bind to tcp listener");

    axum::serve(listener, app)
        .await
        .expect("failed to start app");
}
