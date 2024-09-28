use super::*;

pub unsafe fn f16_lt(mut a: float16_t, mut b: float16_t) -> bool {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: u64;
    let mut uB: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiB: u64;
    let mut signA: bool;
    let mut signB: bool;
    uA.f = a;
    uiA = uA.ui as u64;
    uB.f = b;
    uiB = uB.ui as u64;
    if !uiA & 0x7c00_i32 as u64 == 0_i32 as u64 && uiA & 0x3ff_i32 as u64 != 0
        || !uiB & 0x7c00_i32 as u64 == 0_i32 as u64 && uiB & 0x3ff_i32 as u64 != 0
    {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        return false;
    }
    signA = uiA as u16 as i32 >> 15_i32 != 0;
    signB = uiB as u16 as i32 >> 15_i32 != 0;
    (if signA as i32 != signB as i32 {
        (signA as i32 != 0 && ((uiA | uiB) << 1_i32) as u16 as i32 != 0_i32) as i32
    } else {
        (uiA != uiB && signA as i32 ^ (uiA < uiB) as i32 != 0) as i32
    } != 0)
}
