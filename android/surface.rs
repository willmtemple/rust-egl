/* automatically generated by rust-bindgen */

use std::libc::*;
pub type Struct_ANativeWindow = c_void;
pub type ANativeWindow = Struct_ANativeWindow;
extern "C" {
    pub fn android_createDisplaySurface() -> *mut ANativeWindow;
}
