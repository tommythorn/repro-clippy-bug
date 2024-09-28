use super::*;

pub unsafe fn f32_to_ui32_r_minMag(mut a: float32_t, mut exact: bool) -> u64 {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: u64;
    let mut exp: i64;
    let mut sig: u64;
    let mut shiftDist: i64;
    let mut sign: bool;
    let mut z: u64;
    uA.f = a;
    uiA = uA.ui as u64;
    exp = (uiA >> 23_i32) as i64 & 0xff_i32 as i64;
    sig = uiA & 0x7fffff_i32 as u64;
    shiftDist = 0x9e_i32 as i64 - exp;
    if 32_i32 as i64 <= shiftDist {
        if exact as i32 != 0 && exp as u64 | sig != 0 {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
        }
        return 0_i32 as u64;
    }
    sign = uiA as u32 >> 31_i32 != 0;
    if sign as i32 != 0 || shiftDist < 0_i32 as i64 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        return (if exp == 0xff_i32 as i64 && sig != 0 {
            0xffffffff_u32
        } else if sign as i32 != 0 {
            0_i32 as u32
        } else {
            0xffffffff_u32
        }) as u64;
    }
    sig = (sig | 0x800000_i32 as u64) << 8_i32;
    z = sig >> shiftDist;
    if exact as i32 != 0 && z << shiftDist != sig {
        softfloat_exceptionFlags =
            (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
    }
    z
}
