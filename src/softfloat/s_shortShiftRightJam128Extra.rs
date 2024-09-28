use super::*;

pub unsafe fn softfloat_shortShiftRightJam128Extra(
    mut a64: u64,
    mut a0: u64,
    mut extra: u64,
    mut dist: u8,
) -> uint128_extra {
    let mut uNegDist: u8;
    let mut z: uint128_extra = uint128_extra {
        extra: 0,
        v: uint128 { v0: 0, v64: 0 },
    };
    uNegDist = -(dist as i32) as u8;
    z.v.v64 = a64 >> dist as i32;
    z.v.v0 = a64 << (uNegDist as i32 & 63_i32) | a0 >> dist as i32;
    z.extra = a0 << (uNegDist as i32 & 63_i32) | (extra != 0_i32 as u64) as i32 as u64;
    z
}
