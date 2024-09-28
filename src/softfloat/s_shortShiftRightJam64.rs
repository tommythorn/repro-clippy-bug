pub unsafe fn softfloat_shortShiftRightJam64(a: u64, dist: u8) -> u64 {
    a >> dist | (a & (1_u64 << dist).wrapping_sub(1) != 0) as u64
}
