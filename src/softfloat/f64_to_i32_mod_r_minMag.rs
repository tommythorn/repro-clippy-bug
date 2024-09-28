use super::*;

pub unsafe fn f64_to_i32_mod_r_minMag(mut a: float64_t) -> i64 {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut frac: u64;
    let mut inexact: bool = false;
    let mut invalid: bool = false;
    uA.f = a;
    uiA = uA.ui;
    sign = uiA >> 63_i32 != 0;
    exp = (uiA >> 52_i32) as i64 & 0x7ff_i32 as i64;
    frac = uiA & 0xfffffffffffff;
    if exp == 0_i32 as i64 {
        inexact = frac != 0_i32 as u64;
        frac = 0_i32 as u64;
    } else if exp == 0x7ff_i32 as i64 {
        invalid = 1_i32 != 0;
        frac = 0_i32 as u64;
    } else {
        let mut true_exp: i32 = (exp - 1023_i32 as i64) as i32;
        let mut shift: i32 = true_exp - 52_i32;
        frac |= 1_u64 << 52_i32;
        if shift >= 64_i32 {
            frac = 0_i32 as u64;
        } else if (0_i32..64_i32).contains(&shift) {
            frac <<= shift;
        } else if shift > -64_i32 && shift < 0_i32 {
            inexact = frac << (64_i32 + shift) != 0_i32 as u64;
            frac >>= -shift;
        } else {
            frac = 0_i32 as u64;
            inexact = 1_i32 != 0;
        }
        if true_exp > 31_i32
            || frac
                > (if sign as i32 != 0 {
                    0x80000000_u64
                } else {
                    0x7fffffff_i32 as u64
                })
        {
            invalid = 1_i32 != 0;
            inexact = false;
        }
        if sign {
            frac = frac.wrapping_neg();
        }
    }
    if inexact {
        softfloat_exceptionFlags =
            (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
    }
    if invalid {
        softfloat_exceptionFlags =
            (softfloat_exceptionFlags as i32 | softfloat_flag_invalid as i32) as u8;
    }
    frac as i64
}
