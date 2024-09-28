use super::*;

pub unsafe fn f32_to_i64_r_minMag(mut a: float32_t, mut exact: bool) -> i64 {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: u64;
    let mut exp: i64;
    let mut sig: u64;
    let mut shiftDist: i64;
    let mut sign: bool;
    let mut sig64: u64;
    let mut absZ: i64;
    uA.f = a;
    uiA = uA.ui as u64;
    exp = (uiA >> 23_i32) as i64 & 0xff_i32 as i64;
    sig = uiA & 0x7fffff_i32 as u64;
    shiftDist = 0xbe_i32 as i64 - exp;
    if 64_i32 as i64 <= shiftDist {
        if exact as i32 != 0 && exp as u64 | sig != 0 {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
        }
        return 0_i32 as i64;
    }
    sign = uiA as u32 >> 31_i32 != 0;
    if shiftDist <= 0_i32 as i64 {
        if uiA
            == ((1_i32 as u32) << 31_i32)
                .wrapping_add((0xbe_i32 as u32) << 23_i32)
                .wrapping_add(0_i32 as u32) as u64
        {
            return -0x7fffffffffffffff_i64 - 1_i32 as i64;
        }
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        return (if exp == 0xff_i32 as i64 && sig != 0 {
            0x7fffffffffffffff_u64
        } else if sign as i32 != 0 {
            0x7fffffffffffffff_u64
                .wrapping_neg()
                .wrapping_sub(1_i32 as u64)
        } else {
            0x7fffffffffffffff_u64
        }) as i64;
    }
    sig |= 0x800000_i32 as u64;
    sig64 = sig << 40_i32;
    absZ = (sig64 >> shiftDist) as i64;
    shiftDist = 40_i32 as i64 - shiftDist;
    if exact as i32 != 0
        && shiftDist < 0_i32 as i64
        && (sig << (shiftDist & 31_i32 as i64)) as u32 != 0
    {
        softfloat_exceptionFlags =
            (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
    }
    if sign as i32 != 0 {
        -absZ
    } else {
        absZ
    }
}
