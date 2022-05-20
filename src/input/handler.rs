use crate::trust::UserInput;

#[derive(Default)]
pub struct InputManager {
    handlers: Vec<Box<dyn Fn(&UserInput)>>,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager::default()
    }

    pub fn register(&mut self, handler: Box<dyn Fn(&UserInput)>) -> usize {
        self.handlers.push(handler);
        self.handlers.len() - 1
    }

    pub fn unregister(&mut self, id: usize) {
        drop(self.handlers.remove(id));
    }

    pub fn handle(&self, event: UserInput) {
        for handler in &self.handlers {
            handler(&event);
        }
    }
}
