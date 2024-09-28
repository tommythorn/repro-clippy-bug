pub unsafe fn softfloat_shortShiftLeft64To96M(mut a: u64, mut dist: u8, mut zPtr: *mut u32) {
    *zPtr.offset(0_i32 as isize) = (a as u32) << dist as i32;
    a >>= 32_i32 - dist as i32;
    *zPtr.offset(2_i32 as isize) = (a >> 32_i32) as u32;
    *zPtr.offset(1_i32 as isize) = a as u32;
}
