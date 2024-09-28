pub unsafe fn softfloat_add256M(mut aPtr: *const u64, mut bPtr: *const u64, mut zPtr: *mut u64) {
    let mut index: u32;
    let mut carry: u8;
    let mut wordA: u64;
    let mut wordZ: u64;
    index = 0_i32 as u32;
    carry = 0_i32 as u8;
    loop {
        wordA = *aPtr.offset(index as isize);
        wordZ = wordA
            .wrapping_add(*bPtr.offset(index as isize))
            .wrapping_add(carry as u64);
        *zPtr.offset(index as isize) = wordZ;
        if index == (4_i32 - 1_i32) as u32 {
            break;
        }
        if wordZ != wordA {
            carry = (wordZ < wordA) as i32 as u8;
        }
        index = index.wrapping_add(1_i32 as u32);
    }
}
