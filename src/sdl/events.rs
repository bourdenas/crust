use crate::trust::{self, InputEvent};
use crate::Status;
use sdl2::event::Event;

pub struct EventPump {
    event_pump: sdl2::EventPump,
}

impl EventPump {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, Status> {
        Ok(EventPump {
            event_pump: sdl_context.event_pump()?,
        })
    }

    pub fn poll(&mut self) -> InputEvent {
        match self.event_pump.poll_event() {
            Some(event) => match event {
                Event::Quit { .. } => InputEvent {
                    event: Some(trust::input_event::Event::QuitEvent(trust::QuitEvent {})),
                },
                _ => InputEvent {
                    event: Some(trust::input_event::Event::NoEvent(trust::NoEvent {})),
                },
            },
            None => InputEvent {
                event: Some(trust::input_event::Event::NoEvent(trust::NoEvent {})),
            },
        }
    }
}
