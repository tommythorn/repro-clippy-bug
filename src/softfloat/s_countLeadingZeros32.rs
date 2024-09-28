use super::*;

pub unsafe fn softfloat_countLeadingZeros32(mut a: u32) -> u8 {
    let mut count: u8 = 0;
    if a < 0x10000 {
        count = 16;
        a <<= 16;
    }
    if a < 0x1000000 {
        count += 8;
        a <<= 8;
    }
    count + softfloat_countLeadingZeros8[(a >> 24) as usize]
}
