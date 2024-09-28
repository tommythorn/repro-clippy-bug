use super::*;

pub unsafe fn i32_to_f128(a: i32) -> float128_t {
    let mut uiZ64: u64;
    let mut sign: bool;
    let mut absA: u64;
    let mut shiftDist: i8;
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    uiZ64 = 0_i32 as u64;
    if a != 0 {
        sign = a < 0_i32;
        absA = if sign as i32 != 0 {
            (a as u64).wrapping_neg()
        } else {
            a as u64
        };
        shiftDist = (softfloat_countLeadingZeros32(absA as u32) as i32 + 17_i32) as i8;
        uiZ64 = ((sign as u64) << 63_i32)
            .wrapping_add(((0x402e_i32 - shiftDist as i32) as u64) << 48_i32)
            .wrapping_add(absA << shiftDist as i32);
    }
    uZ.ui.v64 = uiZ64;
    uZ.ui.v0 = 0_i32 as u64;
    uZ.f
}
