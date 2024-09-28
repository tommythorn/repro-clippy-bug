use super::*;

pub unsafe fn ui32_to_f64(a: u32) -> float64_t {
    let mut uiZ: u64;
    let mut shiftDist: i8;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    if a == 0 {
        uiZ = 0_i32 as u64;
    } else {
        shiftDist = (softfloat_countLeadingZeros32(a) as i32 + 21_i32) as i8;
        uiZ = ((0_i32 as u64) << 63_i32)
            .wrapping_add(((0x432_i32 - shiftDist as i32) as u64) << 52_i32)
            .wrapping_add((a as u64) << shiftDist as i32);
    }
    uZ.ui = uiZ;
    uZ.f
}
