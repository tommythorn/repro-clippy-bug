use super::*;

pub unsafe fn softfloat_roundPackToI32(
    mut sign: bool,
    mut sig: u64,
    mut roundingMode: u8,
    mut exact: bool,
) -> i64 {
    let mut roundNearEven: bool;
    let mut roundIncrement: u8;
    let mut roundBits: u8;
    let mut sig32: u64;
    let mut uZ: ui32_i32 = ui32_i32 { ui: 0 };
    let mut z: i64;
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
    sig = sig.wrapping_add(roundIncrement as u64);
    if sig & 0xffffff8000000000_u64 == 0 {
        sig32 = sig >> 7_i32;
        sig32 &= !(((roundBits as i32 ^ 0x40_i32 == 0) as i32 & roundNearEven as i32) as u64);
        uZ.ui = (if sign as i32 != 0 {
            sig32.wrapping_neg()
        } else {
            sig32
        }) as u32;
        z = uZ.i as i64;
        if !(z != 0 && (z < 0_i32 as i64) as i32 ^ sign as i32 != 0) {
            if exact as i32 != 0 && roundBits as i32 != 0 {
                softfloat_exceptionFlags =
                    (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
            }
            return z;
        }
    }
    softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
    (if sign as i32 != 0 {
        -0x7fffffff_i32 - 1_i32
    } else {
        0x7fffffff_i32
    }) as i64
}
