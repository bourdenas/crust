use crate::core::Core;
use std::cell::RefCell;
use std::ffi::CStr;
use std::os::raw::c_char;

thread_local!(static CORE: RefCell<Option<Core>> = RefCell::new(None));

#[no_mangle]
pub extern "C" fn init(resource: *const c_char) {
    let resource = unsafe { CStr::from_ptr(resource) };
    match resource.to_str() {
        Ok(resource) => {
            println!("🦀 says: {}", resource);
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
