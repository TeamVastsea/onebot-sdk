use crate::event::meta::{MetaEvent, MetaEventType};
use crate::event::{Context, EventType};

pub struct EventRegistry {
    meta_event: MetaEvents,
}

type MetaHandler = Box<dyn Fn(&MetaEvent, &Context) -> bool + 'static>;

struct MetaEvents {
    life_cycle: Vec<MetaHandler>,
    heartbeat: Vec<MetaHandler>,
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
        }
    }

    pub fn register_life_cycle(&mut self, f: MetaHandler) {
        self.meta_event.life_cycle.push(f);
    }

    pub fn register_heartbeat(&mut self, f: MetaHandler) {
        self.meta_event.heartbeat.push(f);
    }

    pub fn run_event(&self, event: &EventType, context: Context) {
        match event {
            EventType::Meta(meta) => match meta.meta_event_type {
                MetaEventType::Lifecycle => {
                    for f in &self.meta_event.life_cycle {
                        f(meta, &context);
                    }
                }
                MetaEventType::Heartbeat => {
                    for f in &self.meta_event.heartbeat {
                        f(meta, &context);
                    }
                }
            },
            EventType::Message => {}
            EventType::Notice => {}
            EventType::Request => {}
        }
    }
}
