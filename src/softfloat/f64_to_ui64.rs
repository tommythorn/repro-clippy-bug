use super::*;

pub unsafe fn f64_to_ui64(mut a: float64_t, mut roundingMode: u8, mut exact: bool) -> u64 {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut sig: u64;
    let mut shiftDist: i64;
    let mut sigExtra: uint64_extra = uint64_extra { extra: 0, v: 0 };
    uA.f = a;
    uiA = uA.ui;
    sign = uiA >> 63_i32 != 0;
    exp = (uiA >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sig = uiA & 0xfffffffffffff;
    if exp != 0 {
        sig |= 0x10000000000000_u64;
    }
    shiftDist = 0x433_i32 as i64 - exp;
    if shiftDist <= 0_i32 as i64 {
        if shiftDist < -11_i32 as i64 {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            return if exp == 0x7ff_i32 as i64 && uiA & 0xfffffffffffff != 0 {
                0xffffffffffffffff_u64
            } else if sign as i32 != 0 {
                0_i32 as u64
            } else {
                0xffffffffffffffff_u64
            };
        } else {
            sigExtra.v = sig << -shiftDist;
            sigExtra.extra = 0_i32 as u64;
        }
    } else {
        sigExtra = softfloat_shiftRightJam64Extra(sig, 0_i32 as u64, shiftDist as u64);
    }
    softfloat_roundToUI64(sign, sigExtra.v, sigExtra.extra, roundingMode, exact)
}
