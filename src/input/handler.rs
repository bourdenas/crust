use crate::crust::UserInput;

#[derive(Default)]
pub struct InputManager {
    handlers: Vec<Box<dyn Fn(&UserInput)>>,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager::default()
    }

    pub fn register(&mut self, handler: Box<dyn Fn(&UserInput)>) {
        self.handlers.push(handler);
    }

    pub fn handle(&self, event: UserInput) {
        for handler in &self.handlers {
            handler(&event);
        }
    }
}
