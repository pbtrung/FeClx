extern crate libc;

use std::ffi::CStr;

use self::libc::{c_char, c_int, c_void, size_t};

include!("core.rs");
include!("version.rs");
include!("utils.rs");