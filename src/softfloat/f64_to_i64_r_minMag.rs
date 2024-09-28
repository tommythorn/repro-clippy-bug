use super::*;

pub unsafe fn f64_to_i64_r_minMag(mut a: float64_t, mut exact: bool) -> i64 {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut sig: u64;
    let mut shiftDist: i64;
    let mut absZ: i64;
    uA.f = a;
    uiA = uA.ui;
    sign = uiA >> 63_i32 != 0;
    exp = (uiA >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sig = uiA & 0xfffffffffffff;
    shiftDist = 0x433_i32 as i64 - exp;
    if shiftDist <= 0_i32 as i64 {
        if shiftDist < -10_i32 as i64 {
            if uiA
                == ((1_i32 as u64) << 63_i32)
                    .wrapping_add((0x43e_i32 as u64) << 52_i32)
                    .wrapping_add(0_i32 as u64)
            {
                return -0x7fffffffffffffff_i64 - 1_i32 as i64;
            }
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            return (if exp == 0x7ff_i32 as i64 && sig != 0 {
                0x7fffffffffffffff_u64
            } else if sign as i32 != 0 {
                0x7fffffffffffffff_u64
                    .wrapping_neg()
                    .wrapping_sub(1_i32 as u64)
            } else {
                0x7fffffffffffffff_u64
            }) as i64;
        }
        sig |= 0x10000000000000_u64;
        absZ = (sig << -shiftDist) as i64;
    } else {
        if 53_i32 as i64 <= shiftDist {
            if exact as i32 != 0 && exp as u64 | sig != 0 {
                softfloat_exceptionFlags =
                    (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
            }
            return 0_i32 as i64;
        }
        sig |= 0x10000000000000_u64;
        absZ = (sig >> shiftDist) as i64;
        if exact as i32 != 0 && absZ << shiftDist != sig as i64 {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
        }
    }
    if sign as i32 != 0 {
        -absZ
    } else {
        absZ
    }
}
