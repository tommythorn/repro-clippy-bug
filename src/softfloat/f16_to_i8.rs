use super::*;

pub unsafe fn f16_to_i8(mut a: float16_t, mut roundingMode: u8, mut exact: bool) -> i8 {
    let mut old_flags: u8 = softfloat_exceptionFlags;
    let mut sig32: i64 = f16_to_i32(a, roundingMode, exact);
    if sig32 > 127_i32 as i64 {
        softfloat_exceptionFlags = (old_flags as i32 | softfloat_flag_invalid as i32) as u8;
        0x7f_i32 as i8
    } else if sig32 < -128_i32 as i64 {
        softfloat_exceptionFlags = (old_flags as i32 | softfloat_flag_invalid as i32) as u8;
        return (-0x7f_i32 - 1_i32) as i8;
    } else {
        return sig32 as i8;
    }
}
