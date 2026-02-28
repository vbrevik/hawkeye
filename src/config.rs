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

    /// MLX model name to use for inference
    #[arg(long, default_value = "mlx-community/Qwen3-Next-80B-A3B-Instruct-4bit")]
    pub mlx_model: String,

    /// Number of concurrent workers
    #[arg(long, default_value = "4")]
    pub workers: usize,

    /// Path to Tantivy index directory
    #[arg(long, default_value = ".hawkeye_index")]
    pub index_path: String,

    /// Redis connection URL
    #[arg(long, default_value = "redis://localhost:6379")]
    pub redis_url: String,

    /// Postgres connection URL
    #[arg(long, default_value = "postgresql://hawkeye:hawkeye@localhost:5433/hawkeye")]
    pub postgres_url: String,

    /// etcd base URL
    #[arg(long, default_value = "http://localhost:2379")]
    pub etcd_url: String,

    /// MinIO base URL
    #[arg(long, default_value = "http://localhost:9000")]
    pub minio_url: String,

    /// Milvus base URL
    #[arg(long, default_value = "http://localhost:19530")]
    pub milvus_url: String,

    /// Neo4j HTTP URL
    #[arg(long, default_value = "http://localhost:7475")]
    pub neo4j_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_default_service_urls() {
        let cfg = AppConfig::parse_from(["hawkeye"]);
        assert_eq!(cfg.redis_url, "redis://localhost:6379");
        assert_eq!(cfg.postgres_url, "postgresql://hawkeye:hawkeye@localhost:5433/hawkeye");
        assert_eq!(cfg.etcd_url, "http://localhost:2379");
        assert_eq!(cfg.minio_url, "http://localhost:9000");
        assert_eq!(cfg.milvus_url, "http://localhost:19530");
        assert_eq!(cfg.neo4j_url, "http://localhost:7475");
    }
}
