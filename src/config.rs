use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "hawkeye", about = "Local AI-powered markdown summarizer")]
pub struct AppConfig {
    /// Port for the Axum server
    #[arg(long, default_value = "7700")]
    pub port: u16,

    /// MLX inference server URL
    #[arg(long, default_value = "http://localhost:7701")]
    pub mlx_url: String,

    /// Number of concurrent workers
    #[arg(long, default_value = "4")]
    pub workers: usize,

    /// Path to Tantivy index directory
    #[arg(long, default_value = ".hawkeye_index")]
    pub index_path: String,
}
