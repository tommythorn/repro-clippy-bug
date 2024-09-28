use super::*;

pub unsafe fn i64_to_f128(mut a: i64) -> float128_t {
    let mut uiZ64: u64;
    let mut uiZ0: u64;
    let mut sign: bool;
    let mut absA: u64;
    let mut shiftDist: i8;
    let mut zSig: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    if a == 0 {
        uiZ64 = 0_i32 as u64;
        uiZ0 = 0_i32 as u64;
    } else {
        sign = a < 0_i32 as i64;
        absA = if sign as i32 != 0 {
            (a as u64).wrapping_neg()
        } else {
            a as u64
        };
        shiftDist = (softfloat_countLeadingZeros64(absA) as i32 + 49_i32) as i8;
        if 64_i32 <= shiftDist as i32 {
            zSig.v64 = absA << (shiftDist as i32 - 64_i32);
            zSig.v0 = 0_i32 as u64;
        } else {
            zSig = softfloat_shortShiftLeft128(0_i32 as u64, absA, shiftDist as u8);
        }
        uiZ64 = ((sign as u64) << 63_i32)
            .wrapping_add(((0x406e_i32 - shiftDist as i32) as u64) << 48_i32)
            .wrapping_add(zSig.v64);
        uiZ0 = zSig.v0;
    }
    uZ.ui.v64 = uiZ64;
    uZ.ui.v0 = uiZ0;
    uZ.f
}
