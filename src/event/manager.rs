use crate::crust::Event;

#[derive(Default)]
pub struct EventManager {
    handlers: Vec<Box<dyn Fn(&Event)>>,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager::default()
    }

    pub fn register(&mut self, handler: Box<dyn Fn(&Event)>) {
        self.handlers.push(handler);
    }

    pub fn handle(&self, event: Event) {
        for handler in &self.handlers {
            handler(&event);
        }
    }
}
