use crate::validation::Validator;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

#[unsafe(no_mangle)]
pub extern "C" fn wgsl_validator_create() -> *mut Validator {
    Box::into_raw(Box::new(Validator::new()))
}

#[unsafe(no_mangle)]
pub extern "C" fn wgsl_validator_destroy(ptr: *mut Validator) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wgsl_validator_validate(
    validator: *mut Validator,
    shader_src: *const c_char,
    out_error: *mut *mut c_char,
) -> c_int {
    if validator.is_null() || shader_src.is_null() || out_error.is_null() {
        return -1;
    }

    let shader_str = unsafe {
        match CStr::from_ptr(shader_src).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        }
    };

    let validator = unsafe { &mut *validator };

    match validator.validate_wgsl(shader_str) {
        Ok(_) => {
            unsafe {
                *out_error = ptr::null_mut();
            }
            0
        }
        Err(err) => {
            let c_string = CString::new(err.to_string()).unwrap();
            unsafe {
                *out_error = c_string.into_raw();
            }
            1
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wgsl_validator_free_error(err: *mut c_char) {
    if !err.is_null() {
        unsafe {
            drop(CString::from_raw(err));
        }
    }
}
