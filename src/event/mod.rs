mod meta;
pub mod registry;

use crate::error::{Error, ErrorKind};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::time::Instant;
use tracing::{error, info};
use crate::client::Client;
use crate::event::meta::MetaEvent;

#[derive(Clone, Debug)]
pub enum EventType {
    Meta(MetaEvent),
    Message,
    Notice,
    Request,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub time: i64,
    pub post_type: String,
    pub self_id: i64,
    #[serde(flatten)]
    extra: Value,
}

#[derive(Debug)]
pub struct Context {
    pub start_time: Instant,
}

pub(crate) fn handle_event(client: &Client, incoming: &str) -> Result<(), Error> {
    let event: Event = serde_json::from_str(incoming)?;
    let instant = Instant::now();
    
    let event = match event.post_type.as_str() {
        "meta_event" => EventType::Meta(MetaEvent::from_event(event).ok_or(Error(ErrorKind::ParseError(None)))?),
        "message" => EventType::Message,
        "notice" => EventType::Notice,
        "request" => EventType::Request,
        _ => { return Err(Error(ErrorKind::EventNotRecognised)); }
    };
    
    client.event_registry.run_event(&event, Context { start_time: instant });

    Ok(())
}
