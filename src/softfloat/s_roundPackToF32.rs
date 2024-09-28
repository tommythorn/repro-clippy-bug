use super::*;

pub unsafe fn softfloat_roundPackToF32(mut sign: bool, mut exp: i64, mut sig: u64) -> float32_t {
    let mut current_block: u64;
    let mut roundingMode: u8;
    let mut roundNearEven: bool;
    let mut roundIncrement: u8;
    let mut roundBits: u8;
    let mut isTiny: bool;
    let mut uiZ: u64 = 0;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    roundingMode = softfloat_roundingMode;
    roundNearEven = roundingMode as i32 == softfloat_round_near_even as i32;
    roundIncrement = 0x40_i32 as u8;
    if !roundNearEven && roundingMode as i32 != softfloat_round_near_maxMag as i32 {
        roundIncrement = (if roundingMode as i32
            == (if sign as i32 != 0 {
                softfloat_round_min as i32
            } else {
                softfloat_round_max as i32
            }) {
            0x7f_i32
        } else {
            0_i32
        }) as u8;
    }
    roundBits = (sig & 0x7f_i32 as u64) as u8;
    if 0xfd_i32 as u32 <= exp as u32 {
        if exp < 0_i32 as i64 {
            isTiny = softfloat_detectTininess as i32 == softfloat_tininess_beforeRounding as i32
                || exp < -1_i32 as i64
                || sig.wrapping_add(roundIncrement as u64) < 0x80000000_u32 as u64;
            sig = softfloat_shiftRightJam32(sig as u32, -exp as u64) as u64;
            exp = 0_i32 as i64;
            roundBits = (sig & 0x7f_i32 as u64) as u8;
            if isTiny as i32 != 0 && roundBits as i32 != 0 {
                softfloat_raiseFlags(softfloat_flag_underflow as i32 as u8);
            }
            current_block = 17833034027772472439;
        } else if (0xfd_i32 as i64) < exp
            || 0x80000000_u32 as u64 <= sig.wrapping_add(roundIncrement as u64)
        {
            softfloat_raiseFlags(
                (softfloat_flag_overflow as i32 | softfloat_flag_inexact as i32) as u8,
            );
            uiZ = ((sign as u32) << 31_i32)
                .wrapping_add((0xff_i32 as u32) << 23_i32)
                .wrapping_add(0_i32 as u32)
                .wrapping_sub((roundIncrement == 0) as i32 as u32) as u64;
            current_block = 14329232067213711789;
        } else {
            current_block = 17833034027772472439;
        }
    } else {
        current_block = 17833034027772472439;
    }
    if current_block == 17833034027772472439 {
        sig = sig.wrapping_add(roundIncrement as u64) >> 7_i32;
        if roundBits != 0 {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
            if roundingMode as i32 == softfloat_round_odd as i32 {
                sig |= 1_i32 as u64;
                current_block = 779730699048069558;
            } else {
                current_block = 11042950489265723346;
            }
        } else {
            current_block = 11042950489265723346;
        }
        if current_block == 11042950489265723346 {
            sig &= !(((roundBits as i32 ^ 0x40_i32 == 0) as i32 & roundNearEven as i32) as u64);
            if sig == 0 {
                exp = 0_i32 as i64;
            }
        }
        uiZ = (((sign as u32) << 31_i32).wrapping_add((exp as u32) << 23_i32) as u64)
            .wrapping_add(sig);
    }
    uZ.ui = uiZ as u32;
    uZ.f
}
