pub unsafe fn softfloat_sub256M(mut aPtr: *const u64, mut bPtr: *const u64, mut zPtr: *mut u64) {
    let mut index: u32;
    let mut borrow: u8;
    let mut wordA: u64;
    let mut wordB: u64;
    index = 0_i32 as u32;
    borrow = 0_i32 as u8;
    loop {
        wordA = *aPtr.offset(index as isize);
        wordB = *bPtr.offset(index as isize);
        *zPtr.offset(index as isize) = wordA.wrapping_sub(wordB).wrapping_sub(borrow as u64);
        if index == (4_i32 - 1_i32) as u32 {
            break;
        }
        borrow = (if borrow as i32 != 0 {
            (wordA <= wordB) as i32
        } else {
            (wordA < wordB) as i32
        }) as u8;
        index = index.wrapping_add(1_i32 as u32);
    }
}
