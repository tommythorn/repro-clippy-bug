use super::*;

pub unsafe fn softfloat_approxRecip32_1(mut a: u32) -> u32 {
    let mut index: i32;
    let mut eps: u16;
    let mut r0: u16;
    let mut sigma0: u32;
    let mut r: u64;
    let mut sqrSigma0: u32;
    index = (a >> 27_i32 & 0xf_i32 as u32) as i32;
    eps = (a >> 11_i32) as u16;
    r0 = (softfloat_approxRecip_1k0s[index as usize] as u64)
        .wrapping_sub((softfloat_approxRecip_1k1s[index as usize] as u64 * eps as u64) >> 20_i32)
        as u16;
    sigma0 = !((r0 as u64 * a as u64) >> 7_i32) as u32;
    r = ((r0 as u64) << 16_i32).wrapping_add((r0 as u64 * sigma0 as u64) >> 24_i32);
    sqrSigma0 = ((sigma0 as u64 * sigma0 as u64) >> 32_i32) as u32;
    r = r.wrapping_add((r as u32 as u64 * sqrSigma0 as u64) >> 48_i32);
    r as u32
}
