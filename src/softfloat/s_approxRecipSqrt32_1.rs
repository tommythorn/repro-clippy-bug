use super::*;

pub unsafe fn softfloat_approxRecipSqrt32_1(mut oddExpA: u32, mut a: u32) -> u32 {
    let mut index: i32;
    let mut eps: u16;
    let mut r0: u16;
    let mut ESqrR0: u64;
    let mut sigma0: u32;
    let mut r: u64;
    let mut sqrSigma0: u32;
    index = (a >> 27_i32 & 0xe_i32 as u32).wrapping_add(oddExpA) as i32;
    eps = (a >> 12_i32) as u16;
    r0 = (*softfloat_approxRecipSqrt_1k0s
        .as_ptr()
        .offset(index as isize) as u64)
        .wrapping_sub(
            (*softfloat_approxRecipSqrt_1k1s
                .as_ptr()
                .offset(index as isize) as u64
                * eps as u64)
                >> 20_i32,
        ) as u16;
    ESqrR0 = r0 as u64 * r0 as u64;
    if oddExpA == 0 {
        ESqrR0 <<= 1_i32;
    }
    sigma0 = !((ESqrR0 as u32 as u64 * a as u64) >> 23_i32) as u32;
    r = ((r0 as u64) << 16_i32).wrapping_add((r0 as u64 * sigma0 as u64) >> 25_i32);
    sqrSigma0 = ((sigma0 as u64 * sigma0 as u64) >> 32_i32) as u32;
    r = r.wrapping_add(
        ((r >> 1_i32)
            .wrapping_add(r >> 3_i32)
            .wrapping_sub((r0 as u64) << 14_i32) as u32 as u64
            * sqrSigma0 as u64)
            >> 48_i32,
    );
    if r & 0x80000000_u32 as u64 == 0 {
        r = 0x80000000_u32 as u64;
    }
    r as u32
}
