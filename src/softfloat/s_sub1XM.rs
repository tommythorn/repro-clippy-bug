pub unsafe fn softfloat_sub1XM(mut size_words: u8, mut zPtr: *mut u32) {
    let mut index: u32;
    let mut lastIndex: u32;
    let mut wordA: u32;
    index = 0_i32 as u32;
    lastIndex = (size_words as i32 - 1_i32) as u32;
    loop {
        wordA = *zPtr.offset(index as isize);
        *zPtr.offset(index as isize) = wordA.wrapping_sub(1_i32 as u32);
        if wordA != 0 || index == lastIndex {
            break;
        }
        index = index.wrapping_add(1_i32 as u32);
    }
}
