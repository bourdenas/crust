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
            Some(event) => self.build_event(event),
            None => InputEvent {
                event: Some(trust::input_event::Event::NoEvent(trust::NoEvent {})),
            },
        }
    }

    fn build_event(&self, event: Event) -> InputEvent {
        match event {
            Event::Quit { .. } => InputEvent {
                event: Some(trust::input_event::Event::QuitEvent(trust::QuitEvent {})),
            },
            Event::KeyDown {
                keycode: Some(key),
                repeat,
                ..
            } => InputEvent {
                event: Some(trust::input_event::Event::KeyEvent(trust::KeyEvent {
                    key: key.to_string(),
                    key_state: match repeat {
                        false => trust::KeyState::Pressed as i32,
                        true => trust::KeyState::None as i32,
                    },
                    ..Default::default()
                })),
            },
            Event::KeyUp {
                keycode: Some(key), ..
            } => InputEvent {
                event: Some(trust::input_event::Event::KeyEvent(trust::KeyEvent {
                    key: key.to_string(),
                    key_state: trust::KeyState::Released as i32,
                    ..Default::default()
                })),
            },
            _ => InputEvent {
                event: Some(trust::input_event::Event::NoEvent(trust::NoEvent {})),
            },
        }
    }
}
