use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Event {
    Load,
    Save,
}

pub type Subscriber = fn(event: Event);

#[derive(Default)]
pub struct Publisher {
    events: HashMap<Event, Vec<Subscriber>>,
}

impl Publisher {
    pub fn subscribe(&mut self, event: Event, listener: Subscriber) {
        self.events.entry(event.clone()).or_default();
        self.events.get_mut(&event).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event: Event, listener: Subscriber) {
        self.events
            .get_mut(&event)
            .unwrap()
            .retain(|&x| x != listener);
    }

    pub fn notify(&self, event: Event) {
        let listeners = self.events.get(&event).unwrap();
        for listener in listeners {
            listener(event.clone());
        }
    }
}
