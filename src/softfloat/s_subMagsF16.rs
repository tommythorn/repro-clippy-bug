use super::*;

pub unsafe fn softfloat_subMagsF16(uiA: u64, uiB: u64) -> float16_t {
    let mut current_block: u64;
    let mut expA: i8;
    let mut sigA: u64;
    let mut expB: i8;
    let mut sigB: u64;
    let mut expDiff: i8;
    let mut uiZ: u64 = 0;
    let mut sigDiff: i64;
    let mut signZ: bool = false;
    let mut shiftDist: i8;
    let mut expZ: i8 = 0;
    let mut sigZ: u64 = 0;
    let mut sigX: u64 = 0;
    let mut sigY: u64 = 0;
    let mut sig32Z: u64;
    let mut roundingMode: i8;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    expA = ((uiA >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    sigA = uiA & 0x3ff_i32 as u64;
    expB = ((uiB >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    sigB = uiB & 0x3ff_i32 as u64;
    expDiff = (expA as i32 - expB as i32) as i8;
    if expDiff == 0 {
        if expA as i32 == 0x1f_i32 {
            if sigA | sigB != 0 {
                current_block = 17052621310746810613;
            } else {
                softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
                uiZ = 0x7e00_i32 as u64;
                current_block = 18342692452641517219;
            }
        } else {
            sigDiff = sigA.wrapping_sub(sigB) as i64;
            if sigDiff == 0 {
                uiZ = ((((softfloat_roundingMode as i32 == softfloat_round_min as i32) as i32 as u16
                    as i32)
                    << 15_i32)
                    + ((0_i32 as u16 as i32) << 10_i32)
                    + 0_i32) as u64;
                current_block = 18342692452641517219;
            } else {
                if expA != 0 {
                    expA -= 1;
                }
                signZ = uiA as u16 as i32 >> 15_i32 != 0;
                if sigDiff < 0_i32 as i64 {
                    signZ = !signZ;
                    sigDiff = -sigDiff;
                }
                shiftDist = (softfloat_countLeadingZeros16(sigDiff as u16) as i32 - 5_i32) as i8;
                expZ = (expA as i32 - shiftDist as i32) as i8;
                if (expZ as i32) < 0_i32 {
                    shiftDist = expA;
                    expZ = 0_i32 as i8;
                }
                sigZ = (sigDiff << shiftDist as i32) as u64;
                current_block = 14752396144403658623;
            }
        }
    } else {
        signZ = uiA as u16 as i32 >> 15_i32 != 0;
        if (expDiff as i32) < 0_i32 {
            signZ = !signZ;
            if expB as i32 == 0x1f_i32 {
                if sigB != 0 {
                    current_block = 17052621310746810613;
                } else {
                    uiZ = (((signZ as u16 as i32) << 15_i32)
                        + ((0x1f_i32 as u16 as i32) << 10_i32)
                        + 0_i32) as u64;
                    current_block = 18342692452641517219;
                }
            } else if expDiff as i32 <= -13_i32 {
                uiZ = ((((signZ as u16 as i32) << 15_i32) + ((expB as u16 as i32) << 10_i32))
                    as u64)
                    .wrapping_add(sigB);
                if expA as u64 | sigA != 0 {
                    current_block = 13879597433296872295;
                } else {
                    current_block = 18342692452641517219;
                }
            } else {
                expZ = (expA as i32 + 19_i32) as i8;
                sigX = sigB | 0x400_i32 as u64;
                sigY = sigA.wrapping_add(if expA as i32 != 0 {
                    0x400_i32 as u64
                } else {
                    sigA
                });
                expDiff = -(expDiff as i32) as i8;
                current_block = 1622411330066726685;
            }
        } else {
            uiZ = uiA;
            if expA as i32 == 0x1f_i32 {
                if sigA != 0 {
                    current_block = 17052621310746810613;
                } else {
                    current_block = 18342692452641517219;
                }
            } else if 13_i32 <= expDiff as i32 {
                if expB as u64 | sigB != 0 {
                    current_block = 13879597433296872295;
                } else {
                    current_block = 18342692452641517219;
                }
            } else {
                expZ = (expB as i32 + 19_i32) as i8;
                sigX = sigA | 0x400_i32 as u64;
                sigY = sigB.wrapping_add(if expB as i32 != 0 {
                    0x400_i32 as u64
                } else {
                    sigB
                });
                current_block = 1622411330066726685;
            }
        }
        match current_block {
            18342692452641517219 => {}
            17052621310746810613 => {}
            _ => match current_block {
                13879597433296872295 => {
                    roundingMode = softfloat_roundingMode as i8;
                    if roundingMode as i32 != softfloat_round_near_even as i32 {
                        if roundingMode as i32 == softfloat_round_minMag as i32
                            || roundingMode as i32
                                == (if uiZ as u16 as i32 >> 15_i32 != 0 {
                                    softfloat_round_max as i32
                                } else {
                                    softfloat_round_min as i32
                                })
                        {
                            uiZ = uiZ.wrapping_sub(1);
                        } else if roundingMode as i32 == softfloat_round_odd as i32 {
                            uiZ = uiZ.wrapping_sub(1_i32 as u64) | 1_i32 as u64;
                        }
                    }
                    softfloat_exceptionFlags =
                        (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
                    current_block = 18342692452641517219;
                }
                _ => {
                    sig32Z = (sigX << expDiff as i32).wrapping_sub(sigY);
                    shiftDist = (softfloat_countLeadingZeros32(sig32Z as u32) as i32 - 1_i32) as i8;
                    sig32Z <<= shiftDist as i32;
                    expZ = (expZ as i32 - shiftDist as i32) as i8;
                    sigZ = sig32Z >> 16_i32;
                    if sig32Z & 0xffff_i32 as u64 != 0 {
                        sigZ |= 1_i32 as u64;
                        current_block = 5892776923941496671;
                    } else if sigZ & 0xf_i32 as u64 == 0 && (expZ as u32) < 0x1e_i32 as u32 {
                        sigZ >>= 4_i32;
                        current_block = 14752396144403658623;
                    } else {
                        current_block = 5892776923941496671;
                    }
                    match current_block {
                        14752396144403658623 => {}
                        _ => {
                            return softfloat_roundPackToF16(signZ, expZ as i64, sigZ);
                        }
                    }
                }
            },
        }
    }
    match current_block {
        14752396144403658623 => {
            uiZ = ((((signZ as u16 as i32) << 15_i32) + ((expZ as u16 as i32) << 10_i32)) as u64)
                .wrapping_add(sigZ);
        }
        17052621310746810613 => {
            uiZ = softfloat_propagateNaNF16UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ as u16;
    uZ.f
}
