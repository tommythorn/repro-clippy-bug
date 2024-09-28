use super::*;

pub unsafe fn softfloat_roundToUI64(
    mut sign: bool,
    mut sig: u64,
    mut sigExtra: u64,
    mut roundingMode: u8,
    mut exact: bool,
) -> u64 {
    let mut current_block: u64;
    let mut roundNearEven: bool;
    let mut doIncrement: bool;
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
            current_block = 14526227539666223036;
        } else {
            sig &=
                !(((sigExtra & 0x7fffffffffffffff_u64 == 0) as i32 & roundNearEven as i32) as u64);
            current_block = 7351195479953500246;
        }
    } else {
        current_block = 7351195479953500246;
    }
    if current_block == 7351195479953500246 && !(sign as i32 != 0 && sig != 0) {
        if exact as i32 != 0 && sigExtra != 0 {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
        }
        return sig;
    }
    softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
    if sign as i32 != 0 {
        0_i32 as u64
    } else {
        0xffffffffffffffff_u64
    }
}
