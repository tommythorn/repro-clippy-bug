use super::*;

pub unsafe fn softfloat_roundPackToI64(
    mut sign: bool,
    mut sig: u64,
    mut sigExtra: u64,
    mut roundingMode: u8,
    mut exact: bool,
) -> i64 {
    let mut current_block: u64;
    let mut roundNearEven: bool;
    let mut doIncrement: bool;
    let mut uZ: ui64_i64 = ui64_i64 { ui: 0 };
    let mut z: i64;
    roundNearEven = roundingMode as i32 == softfloat_round_near_even as i32;
    doIncrement = 0x8000000000000000_u64 <= sigExtra;
    if !roundNearEven && roundingMode as i32 != softfloat_round_near_maxMag as i32 {
        doIncrement = roundingMode as i32
            == (if sign as i32 != 0 {
                softfloat_round_min as i32
            } else {
                softfloat_round_max as i32
            })
            && sigExtra != 0;
    }
    if doIncrement {
        sig = sig.wrapping_add(1);
        if sig == 0 {
            current_block = 7672490038788527446;
        } else {
            sig &=
                !(((sigExtra & 0x7fffffffffffffff_u64 == 0) as i32 & roundNearEven as i32) as u64);
            current_block = 14523784380283086299;
        }
    } else {
        current_block = 14523784380283086299;
    }
    if current_block == 14523784380283086299 {
        uZ.ui = if sign as i32 != 0 {
            sig.wrapping_neg()
        } else {
            sig
        };
        z = uZ.i;
        if !(z != 0 && (z < 0_i32 as i64) as i32 ^ sign as i32 != 0) {
            if exact as i32 != 0 && sigExtra != 0 {
                softfloat_exceptionFlags =
                    (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
            }
            return z;
        }
    }
    softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
    (if sign as i32 != 0 {
        0x7fffffffffffffff_u64
            .wrapping_neg()
            .wrapping_sub(1_i32 as u64)
    } else {
        0x7fffffffffffffff_u64
    }) as i64
}
