use super::*;

pub unsafe fn f128_to_ui32(a: float128_t, roundingMode: u8, exact: bool) -> u64 {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: u64;
    let mut uiA0: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut sig64: u64;
    let mut shiftDist: i64;
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    sign = uiA64 >> 63_i32 != 0;
    exp = (uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sig64 = uiA64 & 0xffffffffffff_u64 | (uiA0 != 0_i32 as u64) as i32 as u64;
    if exp == 0x7fff_i32 as i64 && sig64 != 0 {
        sign = false;
    }
    if exp != 0 {
        sig64 |= 0x1000000000000;
    }
    shiftDist = 0x4023 - exp;
    if 0 < shiftDist {
        sig64 = softfloat_shiftRightJam64(sig64, shiftDist as u64);
    }
    softfloat_roundToUI32(sign, sig64, roundingMode, exact)
}
