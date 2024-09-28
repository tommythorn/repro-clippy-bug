use super::*;

pub unsafe fn softfloat_normRoundPackToF64(
    mut sign: bool,
    mut exp: i64,
    mut sig: u64,
) -> float64_t {
    let mut shiftDist: i8;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    shiftDist = (softfloat_countLeadingZeros64(sig) as i32 - 1_i32) as i8;
    exp -= shiftDist as i64;
    if 10_i32 <= shiftDist as i32 && (exp as u32) < 0x7fd_i32 as u32 {
        uZ.ui = ((sign as u64) << 63_i32)
            .wrapping_add(((if sig != 0 { exp } else { 0_i32 as i64 }) as u64) << 52_i32)
            .wrapping_add(sig << (shiftDist as i32 - 10_i32));
        uZ.f
    } else {
        softfloat_roundPackToF64(sign, exp, sig << shiftDist as i32)
    }
}
