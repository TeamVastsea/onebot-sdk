pub mod client;
pub mod error;
pub mod event;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod test {
    use crate::client::Client;
    use serde_json::Value::Bool;
    use tracing::{info, warn, Level};

    #[test]
    fn test_bot() {
        let mut client = Client::new(
            "ws://127.0.0.1:23333".to_string(),
            "HelloOneBotRS".to_string(),
        );
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .finish();
        tracing::subscriber::set_global_default(subscriber).unwrap();

        client
            .event_registry
            .register_heartbeat(Box::new(|meta, context| {
                if let Some(status) = &meta.status {
                    let good = status
                        .get("good")
                        .unwrap_or(&Bool(false))
                        .as_bool()
                        .unwrap_or(false);
                    let online = status
                        .get("online")
                        .unwrap_or(&Bool(false))
                        .as_bool()
                        .unwrap_or(false);

                    if !good || !online {
                        warn!(
                            "[meta.heartbeat]: Bot is not in good status: good={}, online={}",
                            good, online
                        );
                        return false;
                    }
                }

                info!(
                    "[meta.heartbeat]: Next beat in {}ms ({:?})",
                    meta.interval.unwrap_or(-1),
                    context.start_time.elapsed(),
                );
                true
            }));

        tokio::runtime::Runtime::new().unwrap().block_on(async {
            client.run().await.unwrap();
        });
    }
}
