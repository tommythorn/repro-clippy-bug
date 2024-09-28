use super::*;

unsafe fn softfloat_shortShiftRightJam128Extra(
    mut a64: u64,
    mut a0: u64,
    mut extra: u64,
    mut dist: u8,
) -> uint128_extra {
    let mut negDist: u8 = -(dist as i32) as u8;
    let mut z: uint128_extra = uint128_extra {
        extra: 0,
        v: uint128 { v0: 0, v64: 0 },
    };
    z.v.v64 = a64 >> dist as i32;
    z.v.v0 = a64 << (negDist as i32 & 63_i32) | a0 >> dist as i32;
    z.extra = a0 << (negDist as i32 & 63_i32) | (extra != 0_i32 as u64) as i32 as u64;
    z
}
pub unsafe fn softfloat_normRoundPackToF128(
    mut sign: bool,
    mut exp: i64,
    mut sig64: u64,
    mut sig0: u64,
) -> float128_t {
    let mut shiftDist: i8;
    let mut sig128: uint128;
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut sigExtra: u64;
    let mut sig128Extra: uint128_extra;
    if sig64 == 0 {
        exp -= 64_i32 as i64;
        sig64 = sig0;
        sig0 = 0_i32 as u64;
    }
    shiftDist = (softfloat_countLeadingZeros64(sig64) as i32 - 15_i32) as i8;
    exp -= shiftDist as i64;
    if 0_i32 <= shiftDist as i32 {
        if shiftDist != 0 {
            sig128 = softfloat_shortShiftLeft128(sig64, sig0, shiftDist as u8);
            sig64 = sig128.v64;
            sig0 = sig128.v0;
        }
        if (exp as u32) < 0x7ffd_i32 as u32 {
            uZ.ui.v64 = ((sign as u64) << 63_i32)
                .wrapping_add(
                    ((if sig64 | sig0 != 0 { exp } else { 0_i32 as i64 }) as u64) << 48_i32,
                )
                .wrapping_add(sig64);
            uZ.ui.v0 = sig0;
            return uZ.f;
        }
        sigExtra = 0_i32 as u64;
    } else {
        sig128Extra = softfloat_shortShiftRightJam128Extra(
            sig64,
            sig0,
            0_i32 as u64,
            -(shiftDist as i32) as u8,
        );
        sig64 = sig128Extra.v.v64;
        sig0 = sig128Extra.v.v0;
        sigExtra = sig128Extra.extra;
    }
    softfloat_roundPackToF128(sign, exp, sig64, sig0, sigExtra)
}
