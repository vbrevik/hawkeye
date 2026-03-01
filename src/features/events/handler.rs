use crate::shared::config::DEFAULT_WORKSPACE_ID;
use crate::shared::state::AppState;
use axum::extract::State;
use axum::response::sse::{Event, KeepAlive, Sse};
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use tokio_stream::StreamExt;

pub async fn handle_events(
    State(state): State<Arc<AppState>>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let redis_url = state.config.redis_url.clone();
    let channel = super::publisher::events_channel(DEFAULT_WORKSPACE_ID);

    let (tx, rx) = tokio::sync::mpsc::channel::<String>(256);

    tokio::spawn(async move {
        let client = match redis::Client::open(redis_url.as_str()) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!(error = %e, "failed to open Redis client for SSE");
                return;
            }
        };

        let mut pubsub = match client.get_async_pubsub().await {
            Ok(ps) => ps,
            Err(e) => {
                tracing::error!(error = %e, "failed to get pub/sub connection for SSE");
                return;
            }
        };

        if let Err(e) = pubsub.subscribe(&channel).await {
            tracing::error!(error = %e, "failed to subscribe to events channel");
            return;
        }

        tracing::info!(channel = %channel, "SSE client connected");

        let mut msg_stream = Box::pin(pubsub.on_message());
        while let Some(msg) = msg_stream.next().await {
            match msg.get_payload::<String>() {
                Ok(payload) => {
                    if tx.send(payload).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    tracing::warn!(error = %e, "invalid pub/sub message payload");
                }
            }
        }

        tracing::info!("SSE client disconnected");
    });

    let stream = async_stream::stream! {
        let mut rx = rx;
        while let Some(payload) = rx.recv().await {
            if let Ok(event_data) = serde_json::from_str::<serde_json::Value>(&payload) {
                let event_type = event_data
                    .get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("message");

                yield Ok::<_, Infallible>(
                    Event::default()
                        .event(event_type)
                        .data(payload),
                );
            }
        }
    };

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keepalive"),
    )
}
