use super::*;

pub unsafe fn softfloat_mul128To256M(
    mut a64: u64,
    mut a0: u64,
    mut b64: u64,
    mut b0: u64,
    mut zPtr: *mut u64,
) {
    let mut p0: uint128;
    let mut p64: uint128;
    let mut p128: uint128;
    let mut z64: u64;
    let mut z128: u64;
    let mut z192: u64;
    p0 = softfloat_mul64To128(a0, b0);
    *zPtr.offset(0_i32 as isize) = p0.v0;
    p64 = softfloat_mul64To128(a64, b0);
    z64 = (p64.v0).wrapping_add(p0.v64);
    z128 = (p64.v64).wrapping_add((z64 < p64.v0) as i32 as u64);
    p128 = softfloat_mul64To128(a64, b64);
    z128 = z128.wrapping_add(p128.v0);
    z192 = (p128.v64).wrapping_add((z128 < p128.v0) as i32 as u64);
    p64 = softfloat_mul64To128(a0, b64);
    z64 = z64.wrapping_add(p64.v0);
    *zPtr.offset(1_i32 as isize) = z64;
    p64.v64 = (p64.v64).wrapping_add((z64 < p64.v0) as i32 as u64);
    z128 = z128.wrapping_add(p64.v64);
    *zPtr.offset(2_i32 as isize) = z128;
    *zPtr.offset(3_i32 as isize) = z192.wrapping_add((z128 < p64.v64) as i32 as u64);
}
