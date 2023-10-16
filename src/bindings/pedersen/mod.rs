use super::parse_c_str;
use crate::pedersen___plookup_commit_with_hash_index;

pub fn plookup_commit_with_hash_index(
    input_buf: &[u8],
    hash_index: u32,
) -> Result<[u8; 64], String> {
    let mut result = [0u8; 64];
    let error_msg_ptr = unsafe {
        pedersen___plookup_commit_with_hash_index(
            input_buf.as_ptr(),
            &hash_index,
            result.as_mut_slice().as_mut_ptr(),
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
