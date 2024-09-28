use super::*;

pub unsafe fn f32_to_ui16(mut a: float32_t, mut roundingMode: u8, mut exact: bool) -> u64 {
    let mut old_flags: u8 = softfloat_exceptionFlags;
    let mut sig32: u64 = f32_to_ui32(a, roundingMode, exact);
    if sig32 > 65535_i32 as u64 {
        softfloat_exceptionFlags = (old_flags as i32 | softfloat_flag_invalid as i32) as u8;
        0xffff_i32 as u64
    } else {
        sig32
    }
}
