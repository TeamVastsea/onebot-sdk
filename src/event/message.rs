use crate::event::Event;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum MessageEventType {
    Group,
    Private,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sender {
    pub user_id: i64,
    pub nickname: String,
    pub sex: Option<String>, // male, female, unknown
    pub age: Option<i32>,
    pub card: Option<String>, // 群名片／备注
    #[serde(flatten)]
    pub extra: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Anonymous {
    pub id: i64,
    pub name: String,
    pub flag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub message_type: MessageEventType,
    pub sub_type: String, // friend, group, other
    pub message_id: i32,
    pub user_id: i64,
    pub message: Value, // 消息内容
    pub raw_message: String,
    pub font: Option<i32>,
    pub sender: Sender,
    pub reply: Option<Value>,         // 要回复的内容
    pub auto_escape: Option<bool>,    // 是否转义
    pub anonymous: Option<Anonymous>, // 匿名信息
    pub group_id: Option<i64>,
    #[serde(flatten)]
    pub extra: Option<Value>,
    // message seq, real id, message format, target id, raw
}

impl MessageEvent {
    pub fn from_event(event: Event) -> Option<MessageEvent> {
        let mut extra = event.extra;
        let extra = extra.as_object_mut()?;
        Some(MessageEvent {
            time: event.time,
            self_id: event.self_id,
            post_type: event.post_type,
            message_type: match extra.remove("message_type")?.as_str()? {
                "private" => MessageEventType::Private,
                "group" => MessageEventType::Group,
                _ => return None,
            },
            sub_type: extra.remove("sub_type")?.as_str()?.to_string(),
            message_id: extra.remove("message_id")?.as_i64()? as i32,
            user_id: extra.remove("user_id")?.as_i64()?,
            message: extra.remove("message")?.clone(),
            raw_message: extra.remove("raw_message")?.as_str()?.to_string(),
            font: extra
                .remove("font")
                .and_then(|v: Value| Value::as_i64(&v))
                .map(|x| x as i32),
            sender: serde_json::from_value(extra.remove("sender")?).ok()?,
            reply: extra.remove("reply"),
            auto_escape: extra
                .remove("auto_escape")
                .and_then(|v: Value| Value::as_bool(&v)),
            anonymous: extra
                .remove("anonymous")
                .and_then(|v: Value| serde_json::from_value(v).ok()),
            group_id: extra
                .remove("group_id")
                .and_then(|v: Value| Value::as_i64(&v)),
            extra: Some(Value::Object(extra.clone())),
        })
    }
}
