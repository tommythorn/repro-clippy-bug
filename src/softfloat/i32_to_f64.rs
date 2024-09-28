use super::*;

pub unsafe fn i32_to_f64(a: i32) -> float64_t {
    let mut uiZ: u64;
    let mut sign: bool;
    let mut absA: u64;
    let mut shiftDist: i8;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    if a == 0 {
        uiZ = 0_i32 as u64;
    } else {
        sign = a < 0_i32;
        absA = if sign as i32 != 0 {
            (a as u64).wrapping_neg()
        } else {
            a as u64
        };
        shiftDist = (softfloat_countLeadingZeros32(absA as u32) as i32 + 21_i32) as i8;
        uiZ = ((sign as u64) << 63_i32)
            .wrapping_add(((0x432_i32 - shiftDist as i32) as u64) << 52_i32)
            .wrapping_add(absA << shiftDist as i32);
    }
    uZ.ui = uiZ;
    uZ.f
}
