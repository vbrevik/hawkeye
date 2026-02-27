mod api;
mod config;
mod inference;
mod queue;
mod scanner;
mod search;
mod summary;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("eagle3=info")
        .init();
    tracing::info!("eagle3 starting");
}
