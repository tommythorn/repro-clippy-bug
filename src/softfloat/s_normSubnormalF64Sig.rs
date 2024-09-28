use super::*;

pub unsafe fn softfloat_normSubnormalF64Sig(mut sig: u64) -> exp16_sig64 {
    let mut shiftDist: i8;
    let mut z: exp16_sig64 = exp16_sig64 { exp: 0, sig: 0 };
    shiftDist = (softfloat_countLeadingZeros64(sig) as i32 - 11_i32) as i8;
    z.exp = (1_i32 - shiftDist as i32) as i64;
    z.sig = sig << shiftDist as i32;
    z
}
