unsafe fn softfloat_shortShiftRightJamM(
    mut size_words: u8,
    mut aPtr: *const u64,
    mut dist: u8,
    mut zPtr: *mut u64,
) {
    let mut uNegDist: u8;
    let mut index: u32;
    let mut lastIndex: u32;
    let mut partWordZ: u64;
    let mut wordA: u64;
    uNegDist = -(dist as i32) as u8;
    index = 0_i32 as u32;
    lastIndex = (size_words as i32 - 1_i32) as u32;
    wordA = *aPtr.offset(index as isize);
    partWordZ = wordA >> dist as i32;
    if partWordZ << dist as i32 != wordA {
        partWordZ |= 1_i32 as u64;
    }
    while index != lastIndex {
        wordA = *aPtr.offset(index.wrapping_add(1_i32 as u32) as isize);
        *zPtr.offset(index as isize) = wordA << (uNegDist as i32 & 63_i32) | partWordZ;
        index = index.wrapping_add(1_i32 as u32);
        partWordZ = wordA >> dist as i32;
    }
    *zPtr.offset(index as isize) = partWordZ;
}
pub unsafe fn softfloat_shiftRightJam256M(mut aPtr: *const u64, mut dist: u64, mut zPtr: *mut u64) {
    let mut current_block: u64;
    let mut wordJam: u64;
    let mut wordDist: u64;
    let mut ptr: *mut u64 = std::ptr::null_mut::<u64>();
    let mut i: u8;
    let mut innerDist: u8;
    wordJam = 0_i32 as u64;
    wordDist = dist >> 6_i32;
    if wordDist != 0 {
        if (4_i32 as u64) < wordDist {
            wordDist = 4_i32 as u64;
        }
        ptr = aPtr.offset(0_i32 as isize) as *mut u64;
        i = wordDist as u8;
        loop {
            let fresh0 = ptr;
            ptr = ptr.offset(1);
            wordJam = *fresh0;
            if wordJam != 0 {
                break;
            }
            i = i.wrapping_sub(1);
            if i == 0 {
                break;
            }
        }
        ptr = zPtr;
    }
    if wordDist < 4_i32 as u64 {
        aPtr = aPtr.offset(wordDist as isize);
        innerDist = (dist & 63_i32 as u64) as u8;
        if innerDist != 0 {
            softfloat_shortShiftRightJamM(
                (4_i32 as u64).wrapping_sub(wordDist) as u8,
                aPtr,
                innerDist,
                zPtr.offset(0_i32 as isize),
            );
            if wordDist == 0 {
                current_block = 5745001795824039682;
            } else {
                current_block = 11298138898191919651;
            }
        } else {
            aPtr = aPtr.offset(0_i32 as isize);
            ptr = zPtr.offset(0_i32 as isize);
            i = (4_i32 as u64).wrapping_sub(wordDist) as u8;
            while i != 0 {
                *ptr = *aPtr;
                aPtr = aPtr.offset(1_i32 as isize);
                ptr = ptr.offset(1_i32 as isize);
                i = i.wrapping_sub(1);
            }
            current_block = 11298138898191919651;
        }
        match current_block {
            5745001795824039682 => {}
            _ => {
                ptr = zPtr.offset((4_i32 as u64).wrapping_sub(wordDist) as isize);
                current_block = 15768484401365413375;
            }
        }
    } else {
        current_block = 15768484401365413375;
    }
    if current_block == 15768484401365413375 {
        loop {
            let fresh1 = ptr;
            ptr = ptr.offset(1);
            *fresh1 = 0_i32 as u64;
            wordDist = wordDist.wrapping_sub(1);
            if wordDist == 0 {
                break;
            }
        }
    }
    if wordJam != 0 {
        *zPtr.offset(0_i32 as isize) |= 1_i32 as u64;
    }
}
