use super::*;

pub unsafe fn f128_to_i32_r_minMag(mut a: float128_t, mut exact: bool) -> i64 {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: u64;
    let mut uiA0: u64;
    let mut exp: i64;
    let mut sig64: u64;
    let mut shiftDist: i64;
    let mut sign: bool;
    let mut absZ: i64;
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    exp = (uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sig64 = uiA64 & 0xffffffffffff_u64 | (uiA0 != 0_i32 as u64) as i32 as u64;
    shiftDist = 0x402f_i32 as i64 - exp;
    if 49_i32 as i64 <= shiftDist {
        if exact as i32 != 0 && exp as u64 | sig64 != 0 {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
        }
        return 0_i32 as i64;
    }
    sign = uiA64 >> 63_i32 != 0;
    if shiftDist < 18_i32 as i64 {
        if sign as i32 != 0 && shiftDist == 17_i32 as i64 && sig64 < 0x20000_u64 {
            if exact as i32 != 0 && sig64 != 0 {
                softfloat_exceptionFlags =
                    (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
            }
            return (-0x7fffffff_i32 - 1_i32) as i64;
        }
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        return (if exp == 0x7fff_i32 as i64 && sig64 != 0 {
            0x7fffffff_i32
        } else if sign as i32 != 0 {
            -0x7fffffff_i32 - 1_i32
        } else {
            0x7fffffff_i32
        }) as i64;
    }
    sig64 |= 0x1000000000000_u64;
    absZ = (sig64 >> shiftDist) as i64;
    if exact as i32 != 0 && (absZ as u64) << shiftDist != sig64 {
        softfloat_exceptionFlags =
            (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
    }
    if sign as i32 != 0 {
        -absZ
    } else {
        absZ
    }
}
