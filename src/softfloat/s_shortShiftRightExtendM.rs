pub unsafe fn softfloat_shortShiftRightExtendM(
    mut size_words: u8,
    mut aPtr: *const u32,
    mut dist: u8,
    mut zPtr: *mut u32,
) {
    let mut uNegDist: u8;
    let mut indexA: u32;
    let mut lastIndexA: u32;
    let mut partWordZ: u32;
    let mut wordA: u32;
    uNegDist = -(dist as i32) as u8;
    indexA = 0_i32 as u32;
    lastIndexA = (size_words as i32 - 1_i32) as u32;
    zPtr = zPtr.offset(0_i32 as isize);
    partWordZ = 0_i32 as u32;
    loop {
        wordA = *aPtr.offset(indexA as isize);
        *zPtr = wordA << (uNegDist as i32 & 31_i32) | partWordZ;
        zPtr = zPtr.offset(1_i32 as isize);
        partWordZ = wordA >> dist as i32;
        if indexA == lastIndexA {
            break;
        }
        indexA = indexA.wrapping_add(1_i32 as u32);
    }
    *zPtr = partWordZ;
}
