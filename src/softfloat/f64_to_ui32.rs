use super::*;

pub unsafe fn f64_to_ui32(mut a: float64_t, mut roundingMode: u8, mut exact: bool) -> u64 {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut shiftDist: i64;
    uA.f = a;
    let mut uiA = uA.ui;
    let mut sign = uiA >> 63_i32 != 0;
    let mut exp = (uiA >> 52_i32) as i64 & 0x7ff_i32 as i64;
    let mut sig = uiA & 0xfffffffffffff_u64;
    if exp == 0x7ff_i32 as i64 && sig != 0 {
        sign = false;
    }
    if exp != 0 {
        sig |= 0x10000000000000_u64;
    }
    shiftDist = 0x427_i32 as i64 - exp;
    if (0_i32 as i64) < shiftDist {
        sig = softfloat_shiftRightJam64(sig, shiftDist as u64);
    }
    softfloat_roundToUI32(sign, sig, roundingMode, exact)
}
