use crate::crust::Action;
use std::{cell::RefCell, sync::mpsc::Sender};

// A global Action queue that receives/dispatches actions during the frame.
thread_local!(pub static ACTION_QUEUE: RefCell<Option<ActionQueue>> = RefCell::new(None));

pub struct ActionQueue {
    tx: Sender<Action>,
}

impl ActionQueue {
    pub fn new(tx: Sender<Action>) -> Self {
        ActionQueue { tx }
    }

    pub fn push(&self, action: Action) {
        if let Err(e) = self.tx.send(action) {
            eprintln!("Pushing into action queue failed: {}", e);
        }
    }
}
