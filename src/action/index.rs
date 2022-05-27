use std::{cell::RefCell, collections::HashMap};

// A global Action queue that receives/dispatches actions during the frame.
thread_local!(pub static INDEX: RefCell<Option<Index>> = RefCell::new(None));

pub struct Index {
    entity_index: HashMap<String, u32>,
}

impl Index {
    pub fn new() -> Self {
        Index {
            entity_index: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, node_id: &str, entity_id: u32) {
        if let Some(existing_id) = self.entity_index.insert(node_id.to_owned(), entity_id) {
            panic!("Entity {node_id} already exists with {existing_id}");
        }
    }

    pub fn remove_entity(&mut self, node_id: &str) -> Option<u32> {
        self.entity_index.remove(node_id)
    }

    pub fn find_entity(&self, node_id: &str) -> Option<u32> {
        match self.entity_index.get(node_id) {
            Some(entity_id) => Some(*entity_id),
            None => None,
        }
    }
}
