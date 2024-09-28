use super::*;

pub unsafe fn softfloat_mul128By32(mut a64: u64, mut a0: u64, mut b: u32) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    let mut mid: u64;
    let mut carry: u64;
    z.v0 = a0 * b as u64;
    mid = (a0 >> 32_i32) as u32 as u64 * b as u64;
    carry = (z.v0 >> 32_i32).wrapping_sub(mid) as u32 as u64;
    z.v64 = (a64 * b as u64).wrapping_add(mid.wrapping_add(carry) >> 32_i32);
    z
}
