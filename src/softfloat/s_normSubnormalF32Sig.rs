use super::*;

pub unsafe fn softfloat_normSubnormalF32Sig(sig: u64) -> exp16_sig32 {
    let mut shiftDist: i8;
    let mut z: exp16_sig32 = exp16_sig32 { exp: 0, sig: 0 };
    shiftDist = (softfloat_countLeadingZeros32(sig as u32) as i32 - 8_i32) as i8;
    z.exp = (1_i32 - shiftDist as i32) as i64;
    z.sig = sig << shiftDist as i32;
    z
}
