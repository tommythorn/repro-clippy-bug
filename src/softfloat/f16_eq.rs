use super::*;

pub unsafe fn f16_eq(mut a: float16_t, mut b: float16_t) -> bool {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: u64;
    let mut uB: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiB: u64;
    uA.f = a;
    uiA = uA.ui as u64;
    uB.f = b;
    uiB = uB.ui as u64;
    if !uiA & 0x7c00_i32 as u64 == 0_i32 as u64 && uiA & 0x3ff_i32 as u64 != 0
        || !uiB & 0x7c00_i32 as u64 == 0_i32 as u64 && uiB & 0x3ff_i32 as u64 != 0
    {
        if uiA & 0x7e00_i32 as u64 == 0x7c00_i32 as u64 && uiA & 0x1ff_i32 as u64 != 0
            || uiB & 0x7e00_i32 as u64 == 0x7c00_i32 as u64 && uiB & 0x1ff_i32 as u64 != 0
        {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        }
        return false;
    }
    uiA == uiB || ((uiA | uiB) << 1_i32) as u16 == 0
}
