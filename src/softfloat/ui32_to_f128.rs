use super::*;

pub unsafe fn ui32_to_f128(a: u32) -> float128_t {
    let mut uiZ64: u64;
    let mut shiftDist: i8;
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    uiZ64 = 0_i32 as u64;
    if a != 0 {
        shiftDist = (softfloat_countLeadingZeros32(a) as i32 + 17_i32) as i8;
        uiZ64 = ((0_i32 as u64) << 63_i32)
            .wrapping_add(((0x402e_i32 - shiftDist as i32) as u64) << 48_i32)
            .wrapping_add((a as u64) << shiftDist as i32);
    }
    uZ.ui.v64 = uiZ64;
    uZ.ui.v0 = 0_i32 as u64;
    uZ.f
}
