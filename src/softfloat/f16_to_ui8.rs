use super::*;

pub unsafe fn f16_to_ui8(mut a: float16_t, mut roundingMode: u8, mut exact: bool) -> u8 {
    let mut old_flags: u8 = softfloat_exceptionFlags;
    let mut sig32: u64 = f16_to_ui32(a, roundingMode, exact);
    if sig32 > 255_i32 as u64 {
        softfloat_exceptionFlags = (old_flags as i32 | softfloat_flag_invalid as i32) as u8;
        0xff_i32 as u8
    } else {
        sig32 as u8
    }
}
