use super::*;

pub unsafe fn f64_to_ui64_r_minMag(mut a: float64_t, mut exact: bool) -> u64 {
    let mut current_block: u64;
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut exp: i64;
    let mut sig: u64;
    let mut shiftDist: i64;
    let mut sign: bool;
    let mut z: u64 = 0;
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
        return 0_i32 as u64;
    }
    sign = uiA >> 63_i32 != 0;
    if !sign {
        if shiftDist <= 0_i32 as i64 {
            if shiftDist < -11_i32 as i64 {
                current_block = 5881857425740982783;
            } else {
                z = (sig | 0x10000000000000_u64) << -shiftDist;
                current_block = 13242334135786603907;
            }
        } else {
            sig |= 0x10000000000000_u64;
            z = sig >> shiftDist;
            if exact as i32 != 0 && sig << (-shiftDist & 63_i32 as i64) != 0 {
                softfloat_exceptionFlags =
                    (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
            }
            current_block = 13242334135786603907;
        }
        match current_block {
            5881857425740982783 => {}
            _ => return z,
        }
    }
    softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
    if exp == 0x7ff_i32 as i64 && sig != 0 {
        0xffffffffffffffff_u64
    } else if sign as i32 != 0 {
        0_i32 as u64
    } else {
        0xffffffffffffffff_u64
    }
}
