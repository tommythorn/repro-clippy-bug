use super::*;

pub unsafe fn f64_lt_quiet(mut a: float64_t, mut b: float64_t) -> bool {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut uB: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiB: u64;
    let mut signA: bool;
    let mut signB: bool;
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
    signA = uiA >> 63_i32 != 0;
    signB = uiB >> 63_i32 != 0;
    (if signA as i32 != signB as i32 {
        (signA as i32 != 0 && (uiA | uiB) & 0x7fffffffffffffff_u64 != 0) as i32
    } else {
        (uiA != uiB && signA as i32 ^ (uiA < uiB) as i32 != 0) as i32
    } != 0)
}
