use crate::{crust::EmitAction, event::EventManager};

pub struct Events;

impl Events {
    pub fn emit(emit_action: EmitAction, event_manager: &mut EventManager) {
        if let Some(event) = emit_action.event {
            event_manager.handle(event);
        }
    }
}
