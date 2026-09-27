#![allow(unused)]

use std::ffi::{CStr, CString};
use crate::si2;

pub unsafe fn open_c_file(path: &str, mode: &str) -> (*mut libc::FILE, *mut si2::FILE) {
    let c_path = CString::new(path).unwrap();
    let c_mode = CString::new(mode).unwrap();
    
    unsafe {
        let fp = libc::fopen(c_path.as_ptr(), c_mode.as_ptr());
        if fp.is_null() {
            panic!("failed to open file");
        }

        let fp_for_lefr = fp as *mut si2::FILE;
        return (fp, fp_for_lefr)
    }
}

pub unsafe fn const_c_char_ptr_to_string(raw: *const ::std::os::raw::c_char) -> String {
    unsafe { CStr::from_ptr(raw).to_string_lossy().into_owned() }
}

pub unsafe fn mut_c_char_ptr_to_string(raw: *mut ::std::os::raw::c_char) -> String {
    unsafe { CStr::from_ptr(raw).to_string_lossy().into_owned() }
}

pub unsafe fn const_c_char_ptr_to_str(raw: *const ::std::os::raw::c_char) -> &'static str {
    unsafe { CStr::from_ptr(raw).to_str().unwrap() }
}

pub unsafe fn mut_c_char_ptr_to_str(raw: *mut ::std::os::raw::c_char) -> &'static str {
    unsafe { CStr::from_ptr(raw).to_str().unwrap() }
}

pub unsafe fn const_c_char_ptr_to_cstr(raw: *const ::std::os::raw::c_char) -> &'static CStr {
    unsafe { CStr::from_ptr(raw) }
}

pub unsafe fn mut_c_char_ptr_to_cstr(raw: *mut ::std::os::raw::c_char) -> &'static CStr {
    unsafe { CStr::from_ptr(raw) }
}