use super::*;

pub unsafe fn f16_to_i32_r_minMag(mut a: float16_t, mut exact: bool) -> i64 {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: u64;
    let mut exp: i8;
    let mut frac: u64;
    let mut shiftDist: i8;
    let mut sign: bool;
    let mut alignedSig: i64;
    uA.f = a;
    uiA = uA.ui as u64;
    exp = ((uiA >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    frac = uiA & 0x3ff_i32 as u64;
    shiftDist = (exp as i32 - 0xf_i32) as i8;
    if (shiftDist as i32) < 0_i32 {
        if exact as i32 != 0 && exp as u64 | frac != 0 {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
        }
        return 0_i32 as i64;
    }
    sign = uiA as u16 as i32 >> 15_i32 != 0;
    if exp as i32 == 0x1f_i32 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        return (if exp as i32 == 0x1f_i32 && frac != 0 {
            0x7fffffff_i32
        } else if sign as i32 != 0 {
            -0x7fffffff_i32 - 1_i32
        } else {
            0x7fffffff_i32
        }) as i64;
    }
    alignedSig = ((frac | 0x400_i32 as u64) as i64) << shiftDist as i32;
    if exact as i32 != 0 && alignedSig & 0x3ff_i32 as i64 != 0 {
        softfloat_exceptionFlags =
            (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
    }
    alignedSig >>= 10_i32;
    if sign as i32 != 0 {
        -alignedSig
    } else {
        alignedSig
    }
}
