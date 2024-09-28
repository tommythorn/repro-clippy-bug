use super::*;

pub unsafe fn f64_eq(mut a: float64_t, mut b: float64_t) -> bool {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut uB: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiB: u64;
    uA.f = a;
    uiA = uA.ui;
    uB.f = b;
    uiB = uB.ui;
    if !uiA & 0x7ff0000000000000_u64 == 0_i32 as u64 && uiA & 0xfffffffffffff != 0
        || !uiB & 0x7ff0000000000000_u64 == 0_i32 as u64 && uiB & 0xfffffffffffff != 0
    {
        if uiA & 0x7ff8000000000000_u64 == 0x7ff0000000000000_u64 && uiA & 0x7ffffffffffff_u64 != 0
            || uiB & 0x7ff8000000000000_u64 == 0x7ff0000000000000_u64
                && uiB & 0x7ffffffffffff_u64 != 0
        {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        }
        return false;
    }
    uiA == uiB || (uiA | uiB) & 0x7fffffffffffffff_u64 == 0
}
