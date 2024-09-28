use super::*;

pub unsafe fn f128_to_ui64_r_minMag(mut a: float128_t, mut exact: bool) -> u64 {
    let mut current_block: u64;
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: u64;
    let mut uiA0: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut sig64: u64;
    let mut sig0: u64;
    let mut shiftDist: i64;
    let mut negShiftDist: i8;
    let mut z: u64 = 0;
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    sign = uiA64 >> 63_i32 != 0;
    exp = (uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sig64 = uiA64 & 0xffffffffffff_u64;
    sig0 = uiA0;
    shiftDist = 0x402f_i32 as i64 - exp;
    if shiftDist < 0_i32 as i64 {
        if sign as i32 != 0 || shiftDist < -15_i32 as i64 {
            current_block = 7853997131108350903;
        } else {
            sig64 |= 0x1000000000000_u64;
            negShiftDist = -shiftDist as i8;
            z = sig64 << negShiftDist as i32 | sig0 >> (shiftDist & 63_i32 as i64);
            if exact as i32 != 0 && sig0 << negShiftDist as i32 != 0 {
                softfloat_exceptionFlags =
                    (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
            }
            current_block = 10043043949733653460;
        }
    } else {
        if 49_i32 as i64 <= shiftDist {
            if exact as i32 != 0 && exp as u64 | sig64 | sig0 != 0 {
                softfloat_exceptionFlags =
                    (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
            }
            return 0_i32 as u64;
        }
        if sign {
            current_block = 7853997131108350903;
        } else {
            sig64 |= 0x1000000000000_u64;
            z = sig64 >> shiftDist;
            if exact as i32 != 0 && (sig0 != 0 || z << shiftDist != sig64) {
                softfloat_exceptionFlags =
                    (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
            }
            current_block = 10043043949733653460;
        }
    }
    match current_block {
        10043043949733653460 => z,
        _ => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            if exp == 0x7fff_i32 as i64 && sig64 | sig0 != 0 {
                0xffffffffffffffff_u64
            } else if sign as i32 != 0 {
                0_i32 as u64
            } else {
                0xffffffffffffffff_u64
            }
        }
    }
}
