use crate::action::ACTION_QUEUE;
use crate::core::Core;
use crate::crust::{Action, CrustConfig, Event, UserInput};
use prost::Message;
use std::cell::RefCell;

thread_local!(static CORE: RefCell<Option<Core>> = RefCell::new(None));

#[no_mangle]
pub extern "C" fn init(len: i64, encoded_action: *const u8) {
    let config: CrustConfig = decode_message(len, encoded_action);

    println!("🦀 config: {:?}", config);
    CORE.with(|core| {
        *core.borrow_mut() = Some(Core::init(config).expect("Failed to init Core"));
    });
}

#[no_mangle]
pub extern "C" fn run() {
    CORE.with(|core| {
        if let Some(core) = &mut *core.borrow_mut() {
            core.run();
        }
    });
}

#[no_mangle]
pub extern "C" fn halt() {
    CORE.with(|core| {
        *core.borrow_mut() = None;
    });
}

#[no_mangle]
pub extern "C" fn execute(len: i64, encoded_action: *const u8) {
    let action = decode_message::<Action>(len, encoded_action);

    ACTION_QUEUE.with(|queue| {
        if let Some(queue) = &*queue.borrow() {
            queue.push(action);
        }
    });
}

#[no_mangle]
pub extern "C" fn register_input_handler(handler: extern "C" fn(usize, *const u8)) {
    CORE.with(|core| {
        if let Some(core) = &mut *core.borrow_mut() {
            core.input_manager.register(wrap_input_handler(handler));
        }
    });
}

#[no_mangle]
pub extern "C" fn register_event_handler(handler: extern "C" fn(usize, *const u8)) {
    CORE.with(|core| {
        if let Some(core) = &mut *core.borrow_mut() {
            core.event_manager.register(wrap_event_handler(handler));
        }
    });
}

fn decode_message<T: Message + Default>(len: i64, encoded_action: *const u8) -> T {
    let buffer: &[u8];
    unsafe {
        buffer = std::slice::from_raw_parts(encoded_action, len as usize);
    }
    T::decode(buffer).expect("🦀 Failed to parse protobuf message")
}

fn wrap_input_handler(handler: extern "C" fn(usize, *const u8)) -> Box<dyn Fn(&UserInput)> {
    Box::new(move |event: &UserInput| {
        let mut bytes = vec![];
        event
            .encode(&mut bytes)
            .expect("Failed to encode UserInput message");
        handler(bytes.len(), bytes.as_mut_ptr());
    })
}

fn wrap_event_handler(handler: extern "C" fn(usize, *const u8)) -> Box<dyn Fn(&Event)> {
    Box::new(move |event: &Event| {
        let mut bytes = vec![];
        event
            .encode(&mut bytes)
            .expect("Failed to encode Event message");
        handler(bytes.len(), bytes.as_mut_ptr());
    })
}
