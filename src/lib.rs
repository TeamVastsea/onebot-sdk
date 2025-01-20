pub mod client;
pub mod event;
pub mod error;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod test {
    use tracing::{info, Level};
    use crate::client::Client;

    #[test]
    fn test_bot() {
        let mut client = Client::new("ws://127.0.0.1:23333".to_string(), "HelloOneBotRS".to_string());
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .finish();
        tracing::subscriber::set_global_default(subscriber).unwrap();
        
        client.event_registry.register_heartbeat(Box::new(|meta, context| {
            info!("[meta.heartbeat]: Next beat in {}ms ({:?})", meta.interval.unwrap_or(-1), context.start_time.elapsed());
            true
        }));

        tokio::runtime::Runtime::new().unwrap().block_on(async {
            client.run().await.unwrap();
        });
    }
}
