pub unsafe fn softfloat_addM(
    mut size_words: u8,
    mut aPtr: *const u32,
    mut bPtr: *const u32,
    mut zPtr: *mut u32,
) {
    let mut index: u32;
    let mut lastIndex: u32;
    let mut carry: u8;
    let mut wordA: u32;
    let mut wordZ: u32;
    index = 0_i32 as u32;
    lastIndex = (size_words as i32 - 1_i32) as u32;
    carry = 0_i32 as u8;
    loop {
        wordA = *aPtr.offset(index as isize);
        wordZ = wordA
            .wrapping_add(*bPtr.offset(index as isize))
            .wrapping_add(carry as u32);
        *zPtr.offset(index as isize) = wordZ;
        if index == lastIndex {
            break;
        }
        if wordZ != wordA {
            carry = (wordZ < wordA) as i32 as u8;
        }
        index = index.wrapping_add(1_i32 as u32);
    }
}
