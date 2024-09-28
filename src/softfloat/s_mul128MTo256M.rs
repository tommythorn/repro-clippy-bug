pub unsafe fn softfloat_mul128MTo256M(
    mut aPtr: *const u32,
    mut bPtr: *const u32,
    mut zPtr: *mut u32,
) {
    let mut lastZPtr: *mut u32;
    let mut wordB: u32;
    let mut dwordProd: u64;
    let mut wordZ: u32;
    let mut carry: u8;
    bPtr = bPtr.offset(0_i32 as isize);
    lastZPtr = zPtr.offset((8_i32 - 5_i32) as isize);
    zPtr = zPtr.offset(0_i32 as isize);
    wordB = *bPtr;
    dwordProd = *aPtr.offset(0_i32 as isize) as u64 * wordB as u64;
    *zPtr.offset(0_i32 as isize) = dwordProd as u32;
    dwordProd =
        (*aPtr.offset(1_i32 as isize) as u64 * wordB as u64).wrapping_add(dwordProd >> 32_i32);
    *zPtr.offset(1_i32 as isize) = dwordProd as u32;
    dwordProd =
        (*aPtr.offset(2_i32 as isize) as u64 * wordB as u64).wrapping_add(dwordProd >> 32_i32);
    *zPtr.offset(2_i32 as isize) = dwordProd as u32;
    dwordProd =
        (*aPtr.offset(3_i32 as isize) as u64 * wordB as u64).wrapping_add(dwordProd >> 32_i32);
    *zPtr.offset(3_i32 as isize) = dwordProd as u32;
    *zPtr.offset(4_i32 as isize) = (dwordProd >> 32_i32) as u32;
    loop {
        bPtr = bPtr.offset(1_i32 as isize);
        zPtr = zPtr.offset(1_i32 as isize);
        wordB = *bPtr;
        dwordProd = *aPtr.offset(0_i32 as isize) as u64 * wordB as u64;
        wordZ = (*zPtr.offset(0_i32 as isize)).wrapping_add(dwordProd as u32);
        *zPtr.offset(0_i32 as isize) = wordZ;
        carry = (wordZ < dwordProd as u32) as i32 as u8;
        dwordProd =
            (*aPtr.offset(1_i32 as isize) as u64 * wordB as u64).wrapping_add(dwordProd >> 32_i32);
        wordZ = (*zPtr.offset(1_i32 as isize))
            .wrapping_add(dwordProd as u32)
            .wrapping_add(carry as u32);
        *zPtr.offset(1_i32 as isize) = wordZ;
        if wordZ != dwordProd as u32 {
            carry = (wordZ < dwordProd as u32) as i32 as u8;
        }
        dwordProd =
            (*aPtr.offset(2_i32 as isize) as u64 * wordB as u64).wrapping_add(dwordProd >> 32_i32);
        wordZ = (*zPtr.offset(2_i32 as isize))
            .wrapping_add(dwordProd as u32)
            .wrapping_add(carry as u32);
        *zPtr.offset(2_i32 as isize) = wordZ;
        if wordZ != dwordProd as u32 {
            carry = (wordZ < dwordProd as u32) as i32 as u8;
        }
        dwordProd =
            (*aPtr.offset(3_i32 as isize) as u64 * wordB as u64).wrapping_add(dwordProd >> 32_i32);
        wordZ = (*zPtr.offset(3_i32 as isize))
            .wrapping_add(dwordProd as u32)
            .wrapping_add(carry as u32);
        *zPtr.offset(3_i32 as isize) = wordZ;
        if wordZ != dwordProd as u32 {
            carry = (wordZ < dwordProd as u32) as i32 as u8;
        }
        *zPtr.offset(4_i32 as isize) = (dwordProd >> 32_i32).wrapping_add(carry as u64) as u32;
        if zPtr == lastZPtr {
            break;
        }
    }
}
