use crate::action::ACTION_QUEUE;
use crate::core::Core;
use crate::crust::{Action, UserInput};
use prost::Message;
use std::cell::RefCell;
use std::ffi::CStr;
use std::os::raw::c_char;

thread_local!(static CORE: RefCell<Option<Core>> = RefCell::new(None));

#[no_mangle]
pub extern "C" fn init(resource: *const c_char) {
    let resource = unsafe { CStr::from_ptr(resource) };
    match resource.to_str() {
        Ok(resource) => {
            println!("🦀 resource path: {}", resource);
            CORE.with(|core| {
                *core.borrow_mut() = Some(Core::init(resource).expect("Failed to init Core"));
            });
        }
        Err(e) => println!("init() failed to convert c_str: {}", e),
    }
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

fn decode_message<T: Message + Default>(len: i64, encoded_action: *const u8) -> T {
    let buffer: &[u8];
    unsafe {
        buffer = std::slice::from_raw_parts(encoded_action, len as usize);
    }
    T::decode(buffer).expect("🦀 Failed to parse protobuf message")
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
pub extern "C" fn register_handler(handler: extern "C" fn(usize, *const u8)) {
    CORE.with(|core| {
        if let Some(core) = &mut *core.borrow_mut() {
            core.input_manager.register(wrap_handler(handler));
        }
    });
}

fn wrap_handler(handler: extern "C" fn(usize, *const u8)) -> Box<dyn Fn(&UserInput)> {
    Box::new(move |event: &UserInput| {
        let mut bytes = vec![];
        event
            .encode(&mut bytes)
            .expect("Failed to encode UserInput message");
        handler(bytes.len(), bytes.as_mut_ptr());
    })
}
