use super::*;

pub unsafe fn softfloat_mul64ByShifted32To128(mut a: u64, mut b: u32) -> uint128 {
    let mut mid: u64;
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    mid = a as u32 as u64 * b as u64;
    z.v0 = mid << 32_i32;
    z.v64 = ((a >> 32_i32) as u32 as u64 * b as u64).wrapping_add(mid >> 32_i32);
    z
}
