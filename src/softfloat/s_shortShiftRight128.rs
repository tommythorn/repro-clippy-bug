use super::*;

pub unsafe fn softfloat_shortShiftRight128(mut a64: u64, mut a0: u64, mut dist: u8) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v64 = a64 >> dist as i32;
    z.v0 = a64 << (-(dist as i32) & 63_i32) | a0 >> dist as i32;
    z
}
