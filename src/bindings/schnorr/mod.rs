use crate::{schnorr_compute_public_key, schnorr_construct_signature, schnorr_verify_signature};

use super::parse_c_str;

pub fn compute_public_key(bytes: &[u8]) -> Result<[u8; 64], String> {
    let mut result = [0u8; 64];
    let error_msg_ptr =
        unsafe { schnorr_compute_public_key(bytes.as_ptr(), result.as_mut_slice().as_mut_ptr()) };
    if !error_msg_ptr.is_null() {
        return Err(format!(
            "C++ error: {}",
            unsafe { parse_c_str(error_msg_ptr) }.unwrap_or("Parsing c_str failed".to_string())
        ));
    }
    Ok(result)
}

pub unsafe fn construct_signature(
    message: &[u8], // needs to be c string ending on null char
    private_key: &[u8; 32],
) -> Result<([u8; 32], [u8; 32]), String> {
    let mut s = [0u8; 32];
    let mut e = [0u8; 32];
    let error_msg_ptr = unsafe {
        schnorr_construct_signature(
            message.as_ptr(),
            private_key.as_slice().as_ptr(),
            s.as_mut_slice().as_mut_ptr(),
            e.as_mut_slice().as_mut_ptr(),
        )
    };
    if !error_msg_ptr.is_null() {
        return Err(format!(
            "C++ error: {}",
            unsafe { parse_c_str(error_msg_ptr) }.unwrap_or("Parsing c_str failed".to_string())
        ));
    }
    Ok((s, e))
}

pub unsafe fn verify_signature(
    message: &[u8], // needs to be c string ending on null char
    pub_key: [u8; 64],
    sig_s: [u8; 32],
    sig_e: [u8; 32],
) -> Result<bool, String> {
    let mut result = false;
    let error_msg_ptr = unsafe {
        schnorr_verify_signature(
            message.as_ptr(),
            pub_key.as_slice().as_ptr(),
            sig_s.as_slice().as_ptr(),
            sig_e.as_slice().as_ptr(),
            &mut result,
        )
    };
    if !error_msg_ptr.is_null() {
        return Err(format!(
            "C++ error: {}",
            unsafe { parse_c_str(error_msg_ptr) }.unwrap_or("Parsing c_str failed".to_string())
        ));
    }
    Ok(result)
}
