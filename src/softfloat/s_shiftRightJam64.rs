pub fn softfloat_shiftRightJam64(a: u64, dist: u64) -> u64 {
    if dist < 63 {
        a >> dist | (a << (dist.wrapping_neg() & 63) != 0) as i32 as u64
    } else {
        (a != 0) as u64
    }
}
