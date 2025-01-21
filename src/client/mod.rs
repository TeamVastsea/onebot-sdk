use crate::error::{Error, ErrorKind};
use crate::event::handle_event;
use crate::event::registry::EventRegistry;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use tracing::{error, info};
use tungstenite::client::IntoClientRequest;

pub struct Client {
    uri: String,
    access_token: String,
    pub(crate) event_registry: EventRegistry,
}

impl Client {
    pub fn new(uri: String, access_token: String) -> Self {
        Client {
            uri,
            access_token,
            event_registry: EventRegistry::new(),
        }
    }

    pub async fn run(&self) -> Result<(), Error> {
        info!("Connecting to: {}", self.uri);

        let mut request = self.uri.as_str().into_client_request().unwrap();
        if !self.access_token.is_empty() {
            request.headers_mut().insert(
                "Authorization",
                format!("Bearer {}", self.access_token).parse().unwrap(),
            );
        }

        let (mut ws_stream, _) = connect_async(request)
            .await
            .map_err(|_| Error(ErrorKind::ConnectError))?;

        while let Some(Ok(msg)) = ws_stream.next().await {
            if let Ok(text) = msg.to_text() {
                if let Err(err) = handle_event(self, text) {
                    error!("{:?} in {}", err, text);
                }
            }
        }

        Ok(())
    }
}
