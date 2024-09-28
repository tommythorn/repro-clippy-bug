pub unsafe fn softfloat_shiftRightJam32(mut a: u32, mut dist: u64) -> u32 {
    if dist < 31_i32 as u64 {
        a >> dist | (a << (dist.wrapping_neg() & 31_i32 as u64) != 0_i32 as u32) as i32 as u32
    } else {
        (a != 0_i32 as u32) as i32 as u32
    }
}
