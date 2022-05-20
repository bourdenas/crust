use crate::core::Core;
use crate::trust::{Action, UserInput};
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

#[no_mangle]
pub extern "C" fn execute(len: i64, encoded_action: *const u8) -> u32 {
    let buffer: &[u8];
    unsafe {
        buffer = std::slice::from_raw_parts(encoded_action, len as usize);
    }

    let mut retval = 0;

    let action = Action::decode(buffer);
    match action {
        Ok(action) => {
            println!("🦀 execute: {:?}", action);
            CORE.with(|core| {
                if let Some(core) = &mut *core.borrow_mut() {
                    if let Some(id) = core.executor.execute(action, &mut core.world) {
                        retval = id;
                    }
                }
            });
        }
        Err(e) => println!("🦀 execute error: {}", e),
    }

    retval
}

#[no_mangle]
pub extern "C" fn register_handler(handler: extern "C" fn(usize, *const u8)) -> u64 {
    let mut handler_id = 0;
    CORE.with(|core| {
        if let Some(core) = &mut *core.borrow_mut() {
            handler_id = core.input_manager.register(wrap_handler(handler));
        }
    });

    handler_id as u64
}

#[no_mangle]
pub extern "C" fn unregister_handler(handler_id: u64) {
    CORE.with(|core| {
        if let Some(core) = &mut *core.borrow_mut() {
            core.input_manager.unregister(handler_id as usize);
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
