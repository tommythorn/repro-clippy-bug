pub unsafe fn softfloat_mul64To128M(mut a: u64, mut b: u64, mut zPtr: *mut u32) {
    let mut a32: u32;
    let mut a0: u32;
    let mut b32: u32;
    let mut b0: u32;
    let mut z0: u64;
    let mut mid1: u64;
    let mut z64: u64;
    let mut mid: u64;
    a32 = (a >> 32_i32) as u32;
    a0 = a as u32;
    b32 = (b >> 32_i32) as u32;
    b0 = b as u32;
    z0 = a0 as u64 * b0 as u64;
    mid1 = a32 as u64 * b0 as u64;
    mid = mid1.wrapping_add(a0 as u64 * b32 as u64);
    z64 = a32 as u64 * b32 as u64;
    z64 = z64.wrapping_add(((mid < mid1) as i32 as u64) << 32_i32 | mid >> 32_i32);
    mid <<= 32_i32;
    z0 = z0.wrapping_add(mid);
    *zPtr.offset(1_i32 as isize) = (z0 >> 32_i32) as u32;
    *zPtr.offset(0_i32 as isize) = z0 as u32;
    z64 = z64.wrapping_add((z0 < mid) as i32 as u64);
    *zPtr.offset(3_i32 as isize) = (z64 >> 32_i32) as u32;
    *zPtr.offset(2_i32 as isize) = z64 as u32;
}
