use std::ffi::CStr;

extern crate vorbis_sys;

#[test]
fn test_version_string() {
    let version = unsafe { CStr::from_ptr(vorbis_sys::vorbis_version_string()) };
    assert!(version.to_str().unwrap().contains("Vorbis"));
}
