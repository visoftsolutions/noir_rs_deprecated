use crate::rust_srs_init_srs;

use super::{parse_c_str};

pub mod netsrs;

pub fn srs_init(points_buf: &[u8], num_points: u32, g2_point_buf: &[u8]) -> Result<(), String> {
    let error_msg_ptr = unsafe {
        rust_srs_init_srs(
            points_buf.as_ptr(),
            &num_points,
            g2_point_buf.as_ptr(),
        )
    };
    if !error_msg_ptr.is_null() {
        return Err(format!(
            "C++ error: {}",
            unsafe { parse_c_str(error_msg_ptr) }.unwrap_or("Parsing c_str failed".to_string())
        ));
    }
    Ok(())
}
