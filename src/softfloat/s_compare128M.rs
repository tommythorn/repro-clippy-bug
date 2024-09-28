pub unsafe fn softfloat_compare128M(mut aPtr: *const u32, mut bPtr: *const u32) -> i8 {
    let mut index: u32;
    let mut lastIndex: u32;
    let mut wordA: u32;
    let mut wordB: u32;
    index = (4_i32 - 1_i32) as u32;
    lastIndex = 0_i32 as u32;
    loop {
        wordA = *aPtr.offset(index as isize);
        wordB = *bPtr.offset(index as isize);
        if wordA != wordB {
            return (if wordA < wordB { -1_i32 } else { 1_i32 }) as i8;
        }
        if index == lastIndex {
            break;
        }
        index = index.wrapping_sub(1_i32 as u32);
    }
    0_i32 as i8
}
