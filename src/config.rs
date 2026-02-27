use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "eagle3", about = "Local AI-powered markdown summarizer")]
pub struct AppConfig {
    /// Port for the Axum server
    #[arg(long, default_value = "3000")]
    pub port: u16,

    /// MLX inference server URL
    #[arg(long, default_value = "http://localhost:8100")]
    pub mlx_url: String,

    /// Number of concurrent workers
    #[arg(long, default_value = "4")]
    pub workers: usize,

    /// Path to Tantivy index directory
    #[arg(long, default_value = ".eagle3_index")]
    pub index_path: String,
}
