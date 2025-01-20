use crate::event::Event;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum MetaEventType {
    Lifecycle,
    Heartbeat,
}

#[derive(Debug, Clone)]
pub struct MetaEvent {
    pub time: i64,
    pub post_type: String,
    pub self_id: i64,
    pub meta_event_type: MetaEventType,
    // pub sub_type: Option<String>, In websocket connection, this will always be `connect`
    pub status: Option<Value>,
    pub interval: Option<i64>,
}

impl MetaEvent {
    pub fn from_event(event: Event) -> Option<MetaEvent> {
        Some(MetaEvent {
            time: event.time,
            post_type: event.post_type,
            self_id: event.self_id,
            interval: event.extra.get("interval").and_then(Value::as_i64),
            status: event.extra.get("status").cloned(),
            meta_event_type: match event.extra.get("meta_event_type")?.as_str()? {
                "lifecycle" => MetaEventType::Lifecycle,
                "heartbeat" => MetaEventType::Heartbeat,
                _ => return None,
            },
        })
    }

    pub fn get_event_name(&self) -> &str {
        match self.meta_event_type {
            MetaEventType::Lifecycle => "meta.lifecycle",
            MetaEventType::Heartbeat => "meta.heartbeat",
        }
    }
}
