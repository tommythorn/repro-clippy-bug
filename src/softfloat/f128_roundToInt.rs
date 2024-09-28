use super::*;

pub unsafe fn f128_roundToInt(
    mut a: float128_t,
    mut roundingMode: u8,
    mut exact: bool,
) -> float128_t {
    let mut current_block: u64;
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: u64;
    let mut uiA0: u64;
    let mut exp: i64;
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut lastBitMask: u64;
    let mut roundBitsMask: u64;
    let mut roundNearEven: bool;
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    exp = (uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    if 0x402f_i32 as i64 <= exp {
        if 0x406f_i32 as i64 <= exp {
            if exp == 0x7fff_i32 as i64 && uiA64 & 0xffffffffffff_u64 | uiA0 != 0 {
                uiZ = softfloat_propagateNaNF128UI(uiA64, uiA0, 0_i32 as u64, 0_i32 as u64);
            } else {
                return a;
            }
            current_block = 9298455923113512519;
        } else {
            lastBitMask = (2_i32 as u64) << (0x406e_i32 as i64 - exp);
            roundBitsMask = lastBitMask.wrapping_sub(1_i32 as u64);
            uiZ.v64 = uiA64;
            uiZ.v0 = uiA0;
            roundNearEven = roundingMode as i32 == softfloat_round_near_even as i32;
            if roundNearEven as i32 != 0
                || roundingMode as i32 == softfloat_round_near_maxMag as i32
            {
                if exp == 0x402f_i32 as i64 {
                    if 0x8000000000000000_u64 <= uiZ.v0 {
                        uiZ.v64 = (uiZ.v64).wrapping_add(1);
                        if roundNearEven as i32 != 0 && uiZ.v0 == 0x8000000000000000_u64 {
                            uiZ.v64 &= !1_i32 as u64;
                        }
                    }
                } else {
                    uiZ = softfloat_add128(uiZ.v64, uiZ.v0, 0_i32 as u64, lastBitMask >> 1_i32);
                    if roundNearEven as i32 != 0 && uiZ.v0 & roundBitsMask == 0 {
                        uiZ.v0 &= !lastBitMask;
                    }
                }
            } else if roundingMode as i32
                == (if uiZ.v64 >> 63_i32 != 0 {
                    softfloat_round_min as i32
                } else {
                    softfloat_round_max as i32
                })
            {
                uiZ = softfloat_add128(uiZ.v64, uiZ.v0, 0_i32 as u64, roundBitsMask);
            }
            uiZ.v0 &= !roundBitsMask;
            current_block = 6560072651652764009;
        }
    } else if exp < 0x3fff_i32 as i64 {
        if uiA64 & 0x7fffffffffffffff_u64 | uiA0 == 0 {
            return a;
        }
        if exact {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
        }
        uiZ.v64 = uiA64
            & ((1_i32 as u64) << 63_i32)
                .wrapping_add((0_i32 as u64) << 48_i32)
                .wrapping_add(0_i32 as u64);
        uiZ.v0 = 0_i32 as u64;
        let mut current_block_42: u64;
        match roundingMode as i32 {
            0 => {
                if uiA64 & 0xffffffffffff_u64 | uiA0 == 0 {
                    current_block_42 = 12997042908615822766;
                } else {
                    current_block_42 = 3361040475592822182;
                }
            }
            4 => {
                current_block_42 = 3361040475592822182;
            }
            2 => {
                if uiZ.v64 != 0 {
                    uiZ.v64 = ((1_i32 as u64) << 63_i32)
                        .wrapping_add((0x3fff_i32 as u64) << 48_i32)
                        .wrapping_add(0_i32 as u64);
                }
                current_block_42 = 12997042908615822766;
            }
            3 => {
                if uiZ.v64 == 0 {
                    uiZ.v64 = ((0_i32 as u64) << 63_i32)
                        .wrapping_add((0x3fff_i32 as u64) << 48_i32)
                        .wrapping_add(0_i32 as u64);
                }
                current_block_42 = 12997042908615822766;
            }
            _ => {
                current_block_42 = 12997042908615822766;
            }
        }
        if current_block_42 == 3361040475592822182 && exp == 0x3ffe_i32 as i64 {
            uiZ.v64 |= ((0_i32 as u64) << 63_i32)
                .wrapping_add((0x3fff_i32 as u64) << 48_i32)
                .wrapping_add(0_i32 as u64);
        }
        current_block = 9298455923113512519;
    } else {
        uiZ.v64 = uiA64;
        uiZ.v0 = 0_i32 as u64;
        lastBitMask = (1_i32 as u64) << (0x402f_i32 as i64 - exp);
        roundBitsMask = lastBitMask.wrapping_sub(1_i32 as u64);
        if roundingMode as i32 == softfloat_round_near_maxMag as i32 {
            uiZ.v64 = uiZ.v64.wrapping_add(lastBitMask >> 1_i32);
        } else if roundingMode as i32 == softfloat_round_near_even as i32 {
            uiZ.v64 = uiZ.v64.wrapping_add(lastBitMask >> 1_i32);
            if uiZ.v64 & roundBitsMask | uiA0 == 0 {
                uiZ.v64 &= !lastBitMask;
            }
        } else if roundingMode as i32
            == (if uiZ.v64 >> 63_i32 != 0 {
                softfloat_round_min as i32
            } else {
                softfloat_round_max as i32
            })
        {
            uiZ.v64 = (uiZ.v64 | (uiA0 != 0_i32 as u64) as i32 as u64).wrapping_add(roundBitsMask);
        }
        uiZ.v64 &= !roundBitsMask;
        current_block = 6560072651652764009;
    }
    if current_block == 6560072651652764009
        && exact as i32 != 0
        && (uiZ.v64 != uiA64 || uiZ.v0 != uiA0)
    {
        softfloat_exceptionFlags =
            (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
    }
    uZ.ui = uiZ;
    uZ.f
}
