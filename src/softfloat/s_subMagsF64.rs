use super::*;

pub unsafe fn softfloat_subMagsF64(uiA: u64, uiB: u64, mut signZ: bool) -> float64_t {
    let mut current_block: u64;
    let mut expA: i64;
    let mut sigA: u64;
    let mut expB: i64;
    let mut sigB: u64;
    let mut expDiff: i64;
    let mut uiZ: u64 = 0;
    let mut sigDiff: i64;
    let mut shiftDist: i8;
    let mut expZ: i64 = 0;
    let mut sigZ: u64 = 0;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    expA = (uiA >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sigA = uiA & 0xfffffffffffff;
    expB = (uiB >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sigB = uiB & 0xfffffffffffff;
    expDiff = expA - expB;
    if expDiff == 0 {
        if expA == 0x7ff_i32 as i64 {
            if sigA | sigB != 0 {
                current_block = 8694334197172062235;
            } else {
                softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
                uiZ = 0x7ff8000000000000_u64;
                current_block = 15697564613478109721;
            }
        } else {
            sigDiff = sigA.wrapping_sub(sigB) as i64;
            if sigDiff == 0 {
                uiZ = (((softfloat_roundingMode as i32 == softfloat_round_min as i32) as i32
                    as u64)
                    << 63_i32)
                    .wrapping_add((0_i32 as u64) << 52_i32)
                    .wrapping_add(0_i32 as u64);
            } else {
                if expA != 0 {
                    expA -= 1;
                }
                if sigDiff < 0_i32 as i64 {
                    signZ = !signZ;
                    sigDiff = -sigDiff;
                }
                shiftDist = (softfloat_countLeadingZeros64(sigDiff as u64) as i32 - 11_i32) as i8;
                expZ = expA - shiftDist as i64;
                if expZ < 0_i32 as i64 {
                    shiftDist = expA as i8;
                    expZ = 0_i32 as i64;
                }
                uiZ = ((signZ as u64) << 63_i32)
                    .wrapping_add((expZ as u64) << 52_i32)
                    .wrapping_add((sigDiff << shiftDist as i32) as u64);
            }
            current_block = 15697564613478109721;
        }
    } else {
        sigA <<= 10_i32;
        sigB <<= 10_i32;
        if expDiff < 0_i32 as i64 {
            signZ = !signZ;
            if expB == 0x7ff_i32 as i64 {
                if sigB != 0 {
                    current_block = 8694334197172062235;
                } else {
                    uiZ = ((signZ as u64) << 63_i32)
                        .wrapping_add((0x7ff_i32 as u64) << 52_i32)
                        .wrapping_add(0_i32 as u64);
                    current_block = 15697564613478109721;
                }
            } else {
                sigA = sigA.wrapping_add(if expA != 0 {
                    0x4000000000000000_u64
                } else {
                    sigA
                });
                sigA = softfloat_shiftRightJam64(sigA, -expDiff as u64);
                sigB |= 0x4000000000000000_u64;
                expZ = expB;
                sigZ = sigB.wrapping_sub(sigA);
                current_block = 6450597802325118133;
            }
        } else if expA == 0x7ff_i32 as i64 {
            if sigA != 0 {
                current_block = 8694334197172062235;
            } else {
                uiZ = uiA;
                current_block = 15697564613478109721;
            }
        } else {
            sigB = sigB.wrapping_add(if expB != 0 {
                0x4000000000000000_u64
            } else {
                sigB
            });
            sigB = softfloat_shiftRightJam64(sigB, expDiff as u64);
            sigA |= 0x4000000000000000_u64;
            expZ = expA;
            sigZ = sigA.wrapping_sub(sigB);
            current_block = 6450597802325118133;
        }
        match current_block {
            15697564613478109721 => {}
            8694334197172062235 => {}
            _ => {
                return softfloat_normRoundPackToF64(signZ, expZ - 1_i32 as i64, sigZ);
            }
        }
    }
    if current_block == 8694334197172062235 {
        uiZ = softfloat_propagateNaNF64UI(uiA, uiB);
    }
    uZ.ui = uiZ;
    uZ.f
}
