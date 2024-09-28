pub unsafe fn softfloat_negXM(mut size_words: u8, mut zPtr: *mut u32) {
    let mut index: u32;
    let mut lastIndex: u32;
    let mut carry: u8;
    let mut word: u32;
    index = 0_i32 as u32;
    lastIndex = (size_words as i32 - 1_i32) as u32;
    carry = 1_i32 as u8;
    loop {
        word = (!*zPtr.offset(index as isize)).wrapping_add(carry as u32);
        *zPtr.offset(index as isize) = word;
        if index == lastIndex {
            break;
        }
        index = index.wrapping_add(1_i32 as u32);
        if word != 0 {
            carry = 0_i32 as u8;
        }
    }
}
