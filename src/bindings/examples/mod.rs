use crate::{bindings::parse_c_str, rust_examples_simple_create_and_verify_proof};

pub fn simple_create_and_verify_proof() -> Result<bool, String> {
    let mut result = false;
    let error_msg_ptr = unsafe { rust_examples_simple_create_and_verify_proof(&mut result) };
    if !error_msg_ptr.is_null() {
        return Err(format!(
            "C++ error: {}",
            unsafe { parse_c_str(error_msg_ptr) }.unwrap_or("Parsing c_str failed".to_string())
        ));
    }
    Ok(result)
}
