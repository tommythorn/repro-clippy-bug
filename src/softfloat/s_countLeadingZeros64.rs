use super::*;
pub unsafe fn softfloat_countLeadingZeros64(mut a: u64) -> u8 {
    let mut count: u8;
    let mut a32: u32;
    count = 0_i32 as u8;
    a32 = (a >> 32_i32) as u32;
    if a32 == 0 {
        count = 32_i32 as u8;
        a32 = a as u32;
    }
    if a32 < 0x10000_i32 as u32 {
        count = (count as i32 + 16_i32) as u8;
        a32 <<= 16_i32;
    }
    if a32 < 0x1000000_i32 as u32 {
        count = (count as i32 + 8_i32) as u8;
        a32 <<= 8_i32;
    }
    count = (count as i32 + softfloat_countLeadingZeros8[(a32 >> 24_i32) as usize] as i32) as u8;
    count
}

#[test]
fn test_softfload_countLeadingZeros64() {
    for i in 0..63 {
        let x: u64 = 1 << i;
        let r = unsafe { softfloat_countLeadingZeros64(x) } as isize;
        assert_eq!(r, 63 - i);

        // XXX This implies that softfloat_countLeadingZerosX is
        // redundant and should be replaced by .leading_zeros()
        assert_eq!(r, x.leading_zeros() as isize);
    }

    assert_eq!(unsafe { softfloat_countLeadingZeros64(0_u64) } as isize, 64);
    assert_eq!(unsafe { softfloat_countLeadingZeros64(!0) } as isize, 0);
    assert_eq!(
        unsafe { softfloat_countLeadingZeros64(42_u64) } as isize,
        58
    );
}
