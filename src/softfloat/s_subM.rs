pub unsafe fn softfloat_subM(
    mut size_words: u8,
    mut aPtr: *const u32,
    mut bPtr: *const u32,
    mut zPtr: *mut u32,
) {
    let mut index: u32;
    let mut lastIndex: u32;
    let mut borrow: u8;
    let mut wordA: u32;
    let mut wordB: u32;
    index = 0_i32 as u32;
    lastIndex = (size_words as i32 - 1_i32) as u32;
    borrow = 0_i32 as u8;
    loop {
        wordA = *aPtr.offset(index as isize);
        wordB = *bPtr.offset(index as isize);
        *zPtr.offset(index as isize) = wordA.wrapping_sub(wordB).wrapping_sub(borrow as u32);
        if index == lastIndex {
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
