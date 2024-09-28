use super::*;

pub unsafe fn f32_to_i16(mut a: float32_t, mut roundingMode: u8, mut exact: bool) -> i64 {
    let mut old_flags: u8 = softfloat_exceptionFlags;
    let mut sig32: i64 = f32_to_i32(a, roundingMode, exact);
    if sig32 > 32767_i32 as i64 {
        softfloat_exceptionFlags = (old_flags as i32 | softfloat_flag_invalid as i32) as u8;
        0x7fff_i32 as i64
    } else if sig32 < (-32767_i32 - 1_i32) as i64 {
        softfloat_exceptionFlags = (old_flags as i32 | softfloat_flag_invalid as i32) as u8;
        return (-0x7fff_i32 - 1_i32) as i64;
    } else {
        return sig32;
    }
}
