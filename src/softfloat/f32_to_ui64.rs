use super::*;

pub unsafe fn f32_to_ui64(mut a: float32_t, mut roundingMode: u8, mut exact: bool) -> u64 {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut sig: u64;
    let mut shiftDist: i64;
    let mut sig64: u64;
    let mut extra: u64;
    let mut sig64Extra: uint64_extra;
    uA.f = a;
    uiA = uA.ui as u64;
    sign = uiA as u32 >> 31_i32 != 0;
    exp = (uiA >> 23_i32) as i64 & 0xff_i32 as i64;
    sig = uiA & 0x7fffff_i32 as u64;
    shiftDist = 0xbe_i32 as i64 - exp;
    if shiftDist < 0_i32 as i64 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        return if exp == 0xff_i32 as i64 && sig != 0 {
            0xffffffffffffffff_u64
        } else if sign as i32 != 0 {
            0_i32 as u64
        } else {
            0xffffffffffffffff_u64
        };
    }
    if exp != 0 {
        sig |= 0x800000_i32 as u64;
    }
    sig64 = sig << 40_i32;
    extra = 0_i32 as u64;
    if shiftDist != 0 {
        sig64Extra = softfloat_shiftRightJam64Extra(sig64, 0_i32 as u64, shiftDist as u64);
        sig64 = sig64Extra.v;
        extra = sig64Extra.extra;
    }
    softfloat_roundToUI64(sign, sig64, extra, roundingMode, exact)
}
