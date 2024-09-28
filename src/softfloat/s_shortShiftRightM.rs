pub unsafe fn softfloat_shortShiftRightM(
    mut size_words: u8,
    mut aPtr: *const u32,
    mut dist: u8,
    mut zPtr: *mut u32,
) {
    let mut uNegDist: u8;
    let mut index: u32;
    let mut lastIndex: u32;
    let mut partWordZ: u32;
    let mut wordA: u32;
    uNegDist = -(dist as i32) as u8;
    index = 0_i32 as u32;
    lastIndex = (size_words as i32 - 1_i32) as u32;
    partWordZ = *aPtr.offset(index as isize) >> dist as i32;
    while index != lastIndex {
        wordA = *aPtr.offset(index.wrapping_add(1_i32 as u32) as isize);
        *zPtr.offset(index as isize) = wordA << (uNegDist as i32 & 31_i32) | partWordZ;
        index = index.wrapping_add(1_i32 as u32);
        partWordZ = wordA >> dist as i32;
    }
    *zPtr.offset(index as isize) = partWordZ;
}
