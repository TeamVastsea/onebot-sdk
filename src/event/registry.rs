use crate::event::message::{MessageEvent, MessageEventType};
use crate::event::meta::{MetaEvent, MetaEventType};
use crate::event::{Context, EventType};

pub struct EventRegistry {
    meta_event: MetaEvents,
    message_event: MessageEvents,
}

type MetaHandler = Box<dyn Fn(&MetaEvent, &Context) -> bool + 'static>;
type MessageHandler = Box<dyn Fn(&MessageEvent, &Context) -> bool + 'static>;

struct MetaEvents {
    life_cycle: Vec<(MetaHandler, i32)>,
    heartbeat: Vec<(MetaHandler, i32)>,
}

struct MessageEvents {
    group: Vec<(MessageHandler, i32)>,
    private: Vec<(MessageHandler, i32)>,
}

impl Default for EventRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl EventRegistry {
    pub fn new() -> Self {
        EventRegistry {
            meta_event: MetaEvents {
                life_cycle: Vec::new(),
                heartbeat: Vec::new(),
            },
            message_event: MessageEvents {
                group: Vec::new(),
                private: Vec::new(),
            },
        }
    }

    pub fn on_life_cycle(&mut self, f: MetaHandler, order: i32) -> &mut Self {
        self.meta_event.life_cycle.push((f, order));
        self.meta_event.life_cycle.sort_by_key(|x| x.1);
        self
    }

    pub fn on_heartbeat(&mut self, f: MetaHandler, order: i32) -> &mut Self {
        self.meta_event.heartbeat.push((f, order));
        self.meta_event.heartbeat.sort_by_key(|x| x.1);
        self
    }

    pub fn on_group_message(&mut self, f: MessageHandler, order: i32) -> &mut Self {
        self.message_event.group.push((f, order));
        self.message_event.group.sort_by_key(|x| x.1);
        self
    }

    pub fn on_private_message(&mut self, f: MessageHandler, order: i32) -> &mut Self {
        self.message_event.private.push((f, order));
        self.message_event.private.sort_by_key(|x| x.1);
        self
    }

    pub fn run_event(&self, event: &EventType, context: Context) {
        match event {
            EventType::Meta(meta) => match meta.meta_event_type {
                MetaEventType::Lifecycle => {
                    for (f, _) in &self.meta_event.life_cycle {
                        f(meta, &context);
                    }
                }
                MetaEventType::Heartbeat => {
                    for (f, _) in &self.meta_event.heartbeat {
                        f(meta, &context);
                    }
                }
            },
            EventType::Message(message) => match message.message_type {
                MessageEventType::Group => {
                    for (f, _) in &self.message_event.group {
                        f(message, &context);
                    }
                }
                MessageEventType::Private => {
                    for (f, _) in &self.message_event.private {
                        f(message, &context);
                    }
                }
            },
            EventType::Notice => {}
            EventType::Request => {}
        }
    }
}
