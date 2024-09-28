use super::*;

pub unsafe fn softfloat_subMagsF32(uiA: u64, uiB: u64) -> float32_t {
    let mut current_block: u64;
    let mut expA: i64;
    let mut sigA: u64;
    let mut expB: i64;
    let mut sigB: u64;
    let mut expDiff: i64;
    let mut uiZ: u64 = 0;
    let mut sigDiff: i64;
    let mut signZ: bool;
    let mut shiftDist: i8;
    let mut expZ: i64 = 0;
    let mut sigX: u64 = 0;
    let mut sigY: u64 = 0;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    expA = (uiA >> 23_i32) as i64 & 0xff_i32 as i64;
    sigA = uiA & 0x7fffff_i32 as u64;
    expB = (uiB >> 23_i32) as i64 & 0xff_i32 as i64;
    sigB = uiB & 0x7fffff_i32 as u64;
    expDiff = expA - expB;
    if expDiff == 0 {
        if expA == 0xff_i32 as i64 {
            if sigA | sigB != 0 {
                current_block = 18067775777583584507;
            } else {
                softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
                uiZ = 0x7fc00000_i32 as u64;
                current_block = 6711831499037151783;
            }
        } else {
            sigDiff = sigA.wrapping_sub(sigB) as i64;
            if sigDiff == 0 {
                uiZ = (((softfloat_roundingMode as i32 == softfloat_round_min as i32) as i32
                    as u32)
                    << 31_i32)
                    .wrapping_add((0_i32 as u32) << 23_i32)
                    .wrapping_add(0_i32 as u32) as u64;
            } else {
                if expA != 0 {
                    expA -= 1;
                }
                signZ = uiA as u32 >> 31_i32 != 0;
                if sigDiff < 0_i32 as i64 {
                    signZ = !signZ;
                    sigDiff = -sigDiff;
                }
                shiftDist = (softfloat_countLeadingZeros32(sigDiff as u32) as i32 - 8_i32) as i8;
                expZ = expA - shiftDist as i64;
                if expZ < 0_i32 as i64 {
                    shiftDist = expA as i8;
                    expZ = 0_i32 as i64;
                }
                uiZ = (((signZ as u32) << 31_i32).wrapping_add((expZ as u32) << 23_i32) as i64
                    + (sigDiff << shiftDist as i32)) as u64;
            }
            current_block = 6711831499037151783;
        }
    } else {
        signZ = uiA as u32 >> 31_i32 != 0;
        sigA <<= 7_i32;
        sigB <<= 7_i32;
        if expDiff < 0_i32 as i64 {
            signZ = !signZ;
            if expB == 0xff_i32 as i64 {
                if sigB != 0 {
                    current_block = 18067775777583584507;
                } else {
                    uiZ = ((signZ as u32) << 31_i32)
                        .wrapping_add((0xff_i32 as u32) << 23_i32)
                        .wrapping_add(0_i32 as u32) as u64;
                    current_block = 6711831499037151783;
                }
            } else {
                expZ = expB - 1_i32 as i64;
                sigX = sigB | 0x40000000_i32 as u64;
                sigY = sigA.wrapping_add(if expA != 0 {
                    0x40000000_i32 as u64
                } else {
                    sigA
                });
                expDiff = -expDiff;
                current_block = 15090052786889560393;
            }
        } else if expA == 0xff_i32 as i64 {
            if sigA != 0 {
                current_block = 18067775777583584507;
            } else {
                uiZ = uiA;
                current_block = 6711831499037151783;
            }
        } else {
            expZ = expA - 1_i32 as i64;
            sigX = sigA | 0x40000000_i32 as u64;
            sigY = sigB.wrapping_add(if expB != 0 {
                0x40000000_i32 as u64
            } else {
                sigB
            });
            current_block = 15090052786889560393;
        }
        match current_block {
            6711831499037151783 => {}
            18067775777583584507 => {}
            _ => {
                return softfloat_normRoundPackToF32(
                    signZ,
                    expZ,
                    sigX.wrapping_sub(softfloat_shiftRightJam32(sigY as u32, expDiff as u64) as u64),
                );
            }
        }
    }
    if current_block == 18067775777583584507 {
        uiZ = softfloat_propagateNaNF32UI(uiA, uiB);
    }
    uZ.ui = uiZ as u32;
    uZ.f
}
