use super::*;

pub unsafe fn softfloat_countLeadingZeros16(mut a: u16) -> u8 {
    let mut count: u8;
    count = 8_i32 as u8;
    if 0x100_i32 <= a as i32 {
        count = 0_i32 as u8;
        a = (a as i32 >> 8_i32) as u16;
    }
    count = (count as i32 + softfloat_countLeadingZeros8[a as usize] as i32) as u8;
    count
}
