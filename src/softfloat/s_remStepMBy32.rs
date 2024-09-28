pub unsafe fn softfloat_remStepMBy32(
    mut size_words: u8,
    mut remPtr: *const u32,
    mut dist: u8,
    mut bPtr: *const u32,
    mut q: u32,
    mut zPtr: *mut u32,
) {
    let mut index: u32;
    let mut lastIndex: u32;
    let mut dwordProd: u64;
    let mut wordRem: u32;
    let mut wordShiftedRem: u32;
    let mut wordProd: u32;
    let mut uNegDist: u8;
    let mut borrow: u8;
    index = 0_i32 as u32;
    lastIndex = (size_words as i32 - 1_i32) as u32;
    dwordProd = *bPtr.offset(index as isize) as u64 * q as u64;
    wordRem = *remPtr.offset(index as isize);
    wordShiftedRem = wordRem << dist as i32;
    wordProd = dwordProd as u32;
    *zPtr.offset(index as isize) = wordShiftedRem.wrapping_sub(wordProd);
    if index != lastIndex {
        uNegDist = -(dist as i32) as u8;
        borrow = (wordShiftedRem < wordProd) as i32 as u8;
        loop {
            wordShiftedRem = wordRem >> (uNegDist as i32 & 31_i32);
            index = index.wrapping_add(1_i32 as u32);
            dwordProd =
                (*bPtr.offset(index as isize) as u64 * q as u64).wrapping_add(dwordProd >> 32_i32);
            wordRem = *remPtr.offset(index as isize);
            wordShiftedRem |= wordRem << dist as i32;
            wordProd = dwordProd as u32;
            *zPtr.offset(index as isize) = wordShiftedRem
                .wrapping_sub(wordProd)
                .wrapping_sub(borrow as u32);
            if index == lastIndex {
                break;
            }
            borrow = (if borrow as i32 != 0 {
                (wordShiftedRem <= wordProd) as i32
            } else {
                (wordShiftedRem < wordProd) as i32
            }) as u8;
        }
    }
}
