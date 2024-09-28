use super::*;

pub unsafe fn f32_le(mut a: float32_t, mut b: float32_t) -> bool {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: u64;
    let mut uB: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiB: u64;
    let mut signA: bool;
    let mut signB: bool;
    uA.f = a;
    uiA = uA.ui as u64;
    uB.f = b;
    uiB = uB.ui as u64;
    if !uiA & 0x7f800000_i32 as u64 == 0_i32 as u64 && uiA & 0x7fffff_i32 as u64 != 0
        || !uiB & 0x7f800000_i32 as u64 == 0_i32 as u64 && uiB & 0x7fffff_i32 as u64 != 0
    {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        return false;
    }
    signA = uiA as u32 >> 31_i32 != 0;
    signB = uiB as u32 >> 31_i32 != 0;
    (if signA as i32 != signB as i32 {
        (signA as i32 != 0 || ((uiA | uiB) << 1_i32) as u32 == 0) as i32
    } else {
        (uiA == uiB || signA as i32 ^ (uiA < uiB) as i32 != 0) as i32
    } != 0)
}
