use super::*;

pub unsafe fn f32_to_i32(a: float32_t, roundingMode: u8, exact: bool) -> i64 {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut sig: u64;
    let mut sig64: u64;
    let mut shiftDist: i64;
    uA.f = a;
    uiA = uA.ui as u64;
    sign = uiA as u32 >> 31_i32 != 0;
    exp = (uiA >> 23_i32) as i64 & 0xff_i32 as i64;
    sig = uiA & 0x7fffff_i32 as u64;
    if exp == 0xff_i32 as i64 && sig != 0 {
        sign = false;
    }
    if exp != 0 {
        sig |= 0x800000_i32 as u64;
    }
    sig64 = sig << 32_i32;
    shiftDist = 0xaa_i32 as i64 - exp;
    if (0_i32 as i64) < shiftDist {
        sig64 = softfloat_shiftRightJam64(sig64, shiftDist as u64);
    }
    softfloat_roundToI32(sign, sig64, roundingMode, exact)
}
