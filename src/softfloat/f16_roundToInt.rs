use super::*;

pub unsafe fn f16_roundToInt(mut a: float16_t, mut roundingMode: u8, mut exact: bool) -> float16_t {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: u64;
    let mut exp: i8;
    let mut uiZ: u64;
    let mut lastBitMask: u64;
    let mut roundBitsMask: u64;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as u64;
    exp = ((uiA >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    if exp as i32 <= 0xe_i32 {
        if (uiA << 1_i32) as u16 == 0 {
            return a;
        }
        if exact {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
        }
        uiZ = uiA
            & (((1_i32 as u16 as i32) << 15_i32) + ((0_i32 as u16 as i32) << 10_i32) + 0_i32)
                as u64;
        let mut current_block_13: u64;
        match roundingMode as i32 {
            0 => {
                if uiA & 0x3ff_i32 as u64 == 0 {
                    current_block_13 = 4956146061682418353;
                } else {
                    current_block_13 = 2853107117687574469;
                }
            }
            4 => {
                current_block_13 = 2853107117687574469;
            }
            2 => {
                if uiZ != 0 {
                    uiZ = (((1_i32 as u16 as i32) << 15_i32)
                        + ((0xf_i32 as u16 as i32) << 10_i32)
                        + 0_i32) as u64;
                }
                current_block_13 = 4956146061682418353;
            }
            3 => {
                if uiZ == 0 {
                    uiZ = (((0_i32 as u16 as i32) << 15_i32)
                        + ((0xf_i32 as u16 as i32) << 10_i32)
                        + 0_i32) as u64;
                }
                current_block_13 = 4956146061682418353;
            }
            _ => {
                current_block_13 = 4956146061682418353;
            }
        }
        if current_block_13 == 2853107117687574469 && exp as i32 == 0xe_i32 {
            uiZ |= (((0_i32 as u16 as i32) << 15_i32) + ((0xf_i32 as u16 as i32) << 10_i32) + 0_i32)
                as u64;
        }
    } else if 0x19_i32 <= exp as i32 {
        if exp as i32 == 0x1f_i32 && uiA & 0x3ff_i32 as u64 != 0 {
            uiZ = softfloat_propagateNaNF16UI(uiA, 0_i32 as u64);
        } else {
            return a;
        }
    } else {
        uiZ = uiA;
        lastBitMask = (1_i32 as u64) << (0x19_i32 - exp as i32);
        roundBitsMask = lastBitMask.wrapping_sub(1_i32 as u64);
        if roundingMode as i32 == softfloat_round_near_maxMag as i32 {
            uiZ = uiZ.wrapping_add(lastBitMask >> 1_i32);
        } else if roundingMode as i32 == softfloat_round_near_even as i32 {
            uiZ = uiZ.wrapping_add(lastBitMask >> 1_i32);
            if uiZ & roundBitsMask == 0 {
                uiZ &= !lastBitMask;
            }
        } else if roundingMode as i32
            == (if uiZ as u16 as i32 >> 15_i32 != 0 {
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
    uZ.ui = uiZ as u16;
    uZ.f
}
