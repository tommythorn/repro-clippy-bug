use super::*;

pub unsafe fn f64_roundToInt(mut a: float64_t, mut roundingMode: u8, mut exact: bool) -> float64_t {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut exp: i64;
    let mut uiZ: u64;
    let mut lastBitMask: u64;
    let mut roundBitsMask: u64;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    uiA = uA.ui;
    exp = (uiA >> 52_i32) as i64 & 0x7ff_i32 as i64;
    if exp <= 0x3fe_i32 as i64 {
        if uiA & 0x7fffffffffffffff_u64 == 0 {
            return a;
        }
        if exact {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
        }
        uiZ = uiA
            & ((1_i32 as u64) << 63_i32)
                .wrapping_add((0_i32 as u64) << 52_i32)
                .wrapping_add(0_i32 as u64);
        let mut current_block_13: u64;
        match roundingMode as i32 {
            0 => {
                if uiA & 0xfffffffffffff == 0 {
                    current_block_13 = 4956146061682418353;
                } else {
                    current_block_13 = 5964040738800004592;
                }
            }
            4 => {
                current_block_13 = 5964040738800004592;
            }
            2 => {
                if uiZ != 0 {
                    uiZ = ((1_i32 as u64) << 63_i32)
                        .wrapping_add((0x3ff_i32 as u64) << 52_i32)
                        .wrapping_add(0_i32 as u64);
                }
                current_block_13 = 4956146061682418353;
            }
            3 => {
                if uiZ == 0 {
                    uiZ = ((0_i32 as u64) << 63_i32)
                        .wrapping_add((0x3ff_i32 as u64) << 52_i32)
                        .wrapping_add(0_i32 as u64);
                }
                current_block_13 = 4956146061682418353;
            }
            _ => {
                current_block_13 = 4956146061682418353;
            }
        }
        if current_block_13 == 5964040738800004592 && exp == 0x3fe_i32 as i64 {
            uiZ |= ((0_i32 as u64) << 63_i32)
                .wrapping_add((0x3ff_i32 as u64) << 52_i32)
                .wrapping_add(0_i32 as u64);
        }
    } else if 0x433_i32 as i64 <= exp {
        if exp == 0x7ff_i32 as i64 && uiA & 0xfffffffffffff != 0 {
            uiZ = softfloat_propagateNaNF64UI(uiA, 0_i32 as u64);
        } else {
            return a;
        }
    } else {
        uiZ = uiA;
        lastBitMask = (1_i32 as u64) << (0x433_i32 as i64 - exp);
        roundBitsMask = lastBitMask.wrapping_sub(1_i32 as u64);
        if roundingMode as i32 == softfloat_round_near_maxMag as i32 {
            uiZ = uiZ.wrapping_add(lastBitMask >> 1_i32);
        } else if roundingMode as i32 == softfloat_round_near_even as i32 {
            uiZ = uiZ.wrapping_add(lastBitMask >> 1_i32);
            if uiZ & roundBitsMask == 0 {
                uiZ &= !lastBitMask;
            }
        } else if roundingMode as i32
            == (if uiZ >> 63_i32 != 0 {
                softfloat_round_min as i32
            } else {
                softfloat_round_max as i32
            })
        {
            uiZ = uiZ.wrapping_add(roundBitsMask);
        }
        uiZ &= !roundBitsMask;
        if exact as i32 != 0 && uiZ != uiA {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
        }
    }
    uZ.ui = uiZ;
    uZ.f
}
