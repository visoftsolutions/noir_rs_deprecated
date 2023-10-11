use std::ffi::CStr;

pub mod acir;
pub mod examples;
pub mod srs;

pub unsafe fn parse_c_str(ptr: *const ::std::os::raw::c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    CStr::from_ptr(ptr)
        .to_str()
        .map_or(None, |s| Some(s.to_string()))
}

pub fn serialize_slice(data: &[u8]) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&(data.len() as u32).to_be_bytes());
    buffer.extend_from_slice(data);
    buffer
}
