use super::*;

pub unsafe fn f128_to_i64(mut a: float128_t, mut roundingMode: u8, mut exact: bool) -> i64 {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: u64;
    let mut uiA0: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut sig64: u64;
    let mut sig0: u64;
    let mut shiftDist: i64;
    let mut sig128: uint128;
    let mut sigExtra: uint64_extra;
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    sign = uiA64 >> 63_i32 != 0;
    exp = (uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sig64 = uiA64 & 0xffffffffffff_u64;
    sig0 = uiA0;
    shiftDist = 0x402f_i32 as i64 - exp;
    if shiftDist <= 0_i32 as i64 {
        if shiftDist < -15_i32 as i64 {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            return (if exp == 0x7fff_i32 as i64 && sig64 | sig0 != 0 {
                0x7fffffffffffffff_u64
            } else if sign as i32 != 0 {
                0x7fffffffffffffff_u64
                    .wrapping_neg()
                    .wrapping_sub(1_i32 as u64)
            } else {
                0x7fffffffffffffff_u64
            }) as i64;
        }
        sig64 |= 0x1000000000000_u64;
        if shiftDist != 0 {
            sig128 = softfloat_shortShiftLeft128(sig64, sig0, -shiftDist as u8);
            sig64 = sig128.v64;
            sig0 = sig128.v0;
        }
    } else {
        if exp != 0 {
            sig64 |= 0x1000000000000_u64;
        }
        sigExtra = softfloat_shiftRightJam64Extra(sig64, sig0, shiftDist as u64);
        sig64 = sigExtra.v;
        sig0 = sigExtra.extra;
    }
    softfloat_roundToI64(sign, sig64, sig0, roundingMode, exact)
}
