use super::*;

pub unsafe fn softfloat_shortShiftRightJam128(mut a64: u64, mut a0: u64, mut dist: u8) -> uint128 {
    let mut uNegDist: u8;
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    uNegDist = -(dist as i32) as u8;
    z.v64 = a64 >> dist as i32;
    z.v0 = a64 << (uNegDist as i32 & 63_i32)
        | a0 >> dist as i32
        | (a0 << (uNegDist as i32 & 63_i32) != 0_i32 as u64) as i32 as u64;
    z
}
