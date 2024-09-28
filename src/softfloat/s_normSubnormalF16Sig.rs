use super::*;

pub unsafe fn softfloat_normSubnormalF16Sig(mut sig: u64) -> exp8_sig16 {
    let mut shiftDist: i8;
    let mut z: exp8_sig16 = exp8_sig16 { exp: 0, sig: 0 };
    shiftDist = (softfloat_countLeadingZeros16(sig as u16) as i32 - 5_i32) as i8;
    z.exp = (1_i32 - shiftDist as i32) as i8;
    z.sig = sig << shiftDist as i32;
    z
}
