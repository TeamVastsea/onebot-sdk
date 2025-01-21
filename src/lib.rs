pub mod client;
pub mod error;
pub mod event;

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
            .on_heartbeat(
                Box::new(|meta, context| {
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
                }),
                i32::MAX,
            )
            .on_private_message(
                Box::new(|message, context| {
                    info!(
                        "[message.private]: From {}({}), {}: {} ({:?})",
                        message.sender.nickname,
                        message.sender.user_id,
                        message.sub_type,
                        message.raw_message,
                        context.start_time.elapsed(),
                    );
                    true
                }),
                i32::MAX,
            )
            .on_group_message(
                Box::new(|message, context| {
                    info!(
                        "[message.group]: From {}({}) in group {}, {}: {} ({:?})",
                        message.sender.nickname,
                        message.sender.user_id,
                        message.group_id.unwrap_or(-1),
                        message.sub_type,
                        message.raw_message,
                        context.start_time.elapsed(),
                    );
                    true
                }),
                i32::MAX,
            );

        tokio::runtime::Runtime::new().unwrap().block_on(async {
            client.run().await.unwrap();
        });
    }
}
