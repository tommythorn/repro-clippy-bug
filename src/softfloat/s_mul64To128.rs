use super::*;

pub unsafe fn softfloat_mul64To128(mut a: u64, mut b: u64) -> uint128 {
    let mut a32: u32;
    let mut a0: u32;
    let mut b32: u32;
    let mut b0: u32;
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    let mut mid1: u64;
    let mut mid: u64;
    a32 = (a >> 32_i32) as u32;
    a0 = a as u32;
    b32 = (b >> 32_i32) as u32;
    b0 = b as u32;
    z.v0 = a0 as u64 * b0 as u64;
    mid1 = a32 as u64 * b0 as u64;
    mid = mid1.wrapping_add(a0 as u64 * b32 as u64);
    z.v64 = a32 as u64 * b32 as u64;
    z.v64 = z
        .v64
        .wrapping_add(((mid < mid1) as i32 as u64) << 32_i32 | mid >> 32_i32);
    mid <<= 32_i32;
    z.v0 = (z.v0).wrapping_add(mid);
    z.v64 = (z.v64).wrapping_add((z.v0 < mid) as i32 as u64);
    z
}
