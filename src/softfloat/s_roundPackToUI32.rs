use super::*;

pub unsafe fn softfloat_roundPackToUI32(
    mut sign: bool,
    mut sig: u64,
    mut roundingMode: u8,
    mut exact: bool,
) -> u64 {
    let mut roundNearEven: bool;
    let mut roundIncrement: u8;
    let mut roundBits: u8;
    let mut z: u64;
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
        z = sig >> 7_i32;
        z &= !(((roundBits as i32 ^ 0x40_i32 == 0) as i32 & roundNearEven as i32) as u64);
        if !(sign as i32 != 0 && z != 0) {
            if exact as i32 != 0 && roundBits as i32 != 0 {
                softfloat_exceptionFlags =
                    (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
            }
            return z;
        }
    }
    softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
    (if sign as i32 != 0 {
        0_i32 as u32
    } else {
        0xffffffff_u32
    }) as u64
}
