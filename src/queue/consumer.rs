use crate::embedding::client::EmbedClient;
use crate::inference::client::InferenceClient;
use crate::milvus::client::MilvusClient;
use crate::queue::stream::RedisQueue;
use crate::queue::worker;
use crate::search::indexer::SearchIndexer;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::{watch, Mutex};
use tokio::task::JoinHandle;

#[allow(clippy::too_many_arguments)] // each arg is a distinct service dependency
pub fn spawn_consumers(
    queue: RedisQueue,
    client: Arc<InferenceClient>,
    indexer: Arc<Mutex<SearchIndexer>>,
    pool: PgPool,
    embed: Arc<EmbedClient>,
    milvus: Arc<MilvusClient>,
    count: usize,
    shutdown_rx: watch::Receiver<bool>,
) -> Vec<JoinHandle<()>> {
    (0..count)
        .map(|i| {
            let q = queue.clone();
            let c = client.clone();
            let idx = indexer.clone();
            let p = pool.clone();
            let e = embed.clone();
            let m = milvus.clone();
            let rx = shutdown_rx.clone();
            let name = format!("worker-{}", i);

            tokio::spawn(async move {
                consumer_loop(&q, &c, &idx, &p, &e, &m, &name, rx).await;
            })
        })
        .collect()
}

#[allow(clippy::too_many_arguments)] // each arg is a distinct service dependency
async fn consumer_loop(
    queue: &RedisQueue,
    client: &InferenceClient,
    indexer: &Arc<Mutex<SearchIndexer>>,
    pool: &PgPool,
    embed: &EmbedClient,
    milvus: &MilvusClient,
    consumer_name: &str,
    mut shutdown_rx: watch::Receiver<bool>,
) {
    loop {
        tokio::select! {
            _ = shutdown_rx.changed() => {
                tracing::info!(consumer = consumer_name, "shutting down");
                break;
            }
            result = queue.read_next(consumer_name) => {
                match result {
                    Ok(Some((msg_id, file))) => {
                        let file_path = file.path.display().to_string();
                        match worker::process_file(&file, client, indexer, pool, embed, milvus).await {
                            Ok(()) => {
                                if let Err(e) = queue.ack_completed(&msg_id).await {
                                    tracing::error!(error = %e, "failed to ack completed");
                                }
                            }
                            Err(e) => {
                                tracing::error!(file = %file_path, error = %e, "processing failed");
                                if let Err(ae) =
                                    queue.ack_failed(&msg_id, &file_path, &e.to_string()).await
                                {
                                    tracing::error!(error = %ae, "failed to ack failed");
                                }
                            }
                        }
                    }
                    Ok(None) => {}
                    Err(e) => {
                        tracing::error!(error = %e, consumer = consumer_name, "stream read error");
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    }
                }
            }
        }
    }
}
