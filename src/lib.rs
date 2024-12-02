use libc::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn rust_hello() -> *mut c_char {
    let message = "Hello from Rust!";
    let c_str = CString::new(message).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn rust_add(x: i32, y: i32) -> i32 {
    x + y
}

// Free memory allocated by Rust
#[no_mangle]
pub extern "C" fn rust_free_string(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr);
        }
    }
}
