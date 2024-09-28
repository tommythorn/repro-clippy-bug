use super::*;

pub unsafe fn softfloat_normSubnormalF128Sig(mut sig64: u64, mut sig0: u64) -> exp32_sig128 {
    let mut shiftDist: i8;
    let mut z: exp32_sig128 = exp32_sig128 {
        exp: 0,
        sig: uint128 { v0: 0, v64: 0 },
    };
    if sig64 == 0 {
        shiftDist = (softfloat_countLeadingZeros64(sig0) as i32 - 15_i32) as i8;
        z.exp = (-63_i32 - shiftDist as i32) as i64;
        if (shiftDist as i32) < 0_i32 {
            z.sig.v64 = sig0 >> -(shiftDist as i32);
            z.sig.v0 = sig0 << (shiftDist as i32 & 63_i32);
        } else {
            z.sig.v64 = sig0 << shiftDist as i32;
            z.sig.v0 = 0_i32 as u64;
        }
    } else {
        shiftDist = (softfloat_countLeadingZeros64(sig64) as i32 - 15_i32) as i8;
        z.exp = (1_i32 - shiftDist as i32) as i64;
        z.sig = softfloat_shortShiftLeft128(sig64, sig0, shiftDist as u8);
    }
    z
}
