use super::*;

pub unsafe fn f64_to_i32_r_minMag(mut a: float64_t, mut exact: bool) -> i64 {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut exp: i64;
    let mut sig: u64;
    let mut shiftDist: i64;
    let mut sign: bool;
    let mut absZ: i64;
    uA.f = a;
    uiA = uA.ui;
    exp = (uiA >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sig = uiA & 0xfffffffffffff;
    shiftDist = 0x433_i32 as i64 - exp;
    if 53_i32 as i64 <= shiftDist {
        if exact as i32 != 0 && exp as u64 | sig != 0 {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
        }
        return 0_i32 as i64;
    }
    sign = uiA >> 63_i32 != 0;
    if shiftDist < 22_i32 as i64 {
        if sign as i32 != 0 && exp == 0x41e_i32 as i64 && sig < 0x200000_u64 {
            if exact as i32 != 0 && sig != 0 {
                softfloat_exceptionFlags =
                    (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
            }
            return (-0x7fffffff_i32 - 1_i32) as i64;
        }
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        return (if exp == 0x7ff_i32 as i64 && sig != 0 {
            0x7fffffff_i32
        } else if sign as i32 != 0 {
            -0x7fffffff_i32 - 1_i32
        } else {
            0x7fffffff_i32
        }) as i64;
    }
    sig |= 0x10000000000000_u64;
    absZ = (sig >> shiftDist) as i64;
    if exact as i32 != 0 && (absZ as u64) << shiftDist != sig {
        softfloat_exceptionFlags =
            (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
    }
    if sign as i32 != 0 {
        -absZ
    } else {
        absZ
    }
}
