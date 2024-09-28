use super::*;

unsafe fn softfloat_sub128(mut a64: u64, mut a0: u64, mut b64: u64, mut b0: u64) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v0 = a0.wrapping_sub(b0);
    z.v64 = a64.wrapping_sub(b64);
    z.v64 = (z.v64).wrapping_sub((a0 < b0) as i32 as u64);
    z
}
pub unsafe fn softfloat_subMagsF128(
    mut uiA64: u64,
    mut uiA0: u64,
    mut uiB64: u64,
    mut uiB0: u64,
    mut signZ: bool,
) -> float128_t {
    let mut current_block: u64;
    let mut expA: i64;
    let mut sigA: uint128 = uint128 { v0: 0, v64: 0 };
    let mut expB: i64;
    let mut sigB: uint128 = uint128 { v0: 0, v64: 0 };
    let mut sigZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut expDiff: i64;
    let mut expZ: i64 = 0;
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    expA = (uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sigA.v64 = uiA64 & 0xffffffffffff_u64;
    sigA.v0 = uiA0;
    expB = (uiB64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sigB.v64 = uiB64 & 0xffffffffffff_u64;
    sigB.v0 = uiB0;
    sigA = softfloat_shortShiftLeft128(sigA.v64, sigA.v0, 4_i32 as u8);
    sigB = softfloat_shortShiftLeft128(sigB.v64, sigB.v0, 4_i32 as u8);
    expDiff = expA - expB;
    if (0_i32 as i64) < expDiff {
        if expA == 0x7fff_i32 as i64 {
            if sigA.v64 | sigA.v0 != 0 {
                current_block = 8872083469674517140;
            } else {
                uiZ.v64 = uiA64;
                uiZ.v0 = uiA0;
                current_block = 4589634476707811940;
            }
        } else {
            if expB != 0 {
                sigB.v64 |= 0x10000000000000_u64;
                current_block = 12997042908615822766;
            } else {
                expDiff -= 1;
                if expDiff == 0 {
                    current_block = 13443690604571934275;
                } else {
                    current_block = 12997042908615822766;
                }
            }
            if current_block == 12997042908615822766 {
                sigB = softfloat_shiftRightJam128(sigB.v64, sigB.v0, expDiff as u64);
            }
            expZ = expA;
            sigA.v64 |= 0x10000000000000_u64;
            current_block = 10773517525802023192;
        }
    } else {
        if expDiff < 0_i32 as i64 {
            if expB == 0x7fff_i32 as i64 {
                if sigB.v64 | sigB.v0 != 0 {
                    current_block = 8872083469674517140;
                } else {
                    uiZ.v64 = (((signZ as i32 ^ 1_i32) as u64) << 63_i32)
                        .wrapping_add((0x7fff_i32 as u64) << 48_i32)
                        .wrapping_add(0_i32 as u64);
                    uiZ.v0 = 0_i32 as u64;
                    current_block = 4589634476707811940;
                }
            } else {
                if expA != 0 {
                    sigA.v64 |= 0x10000000000000_u64;
                    current_block = 8693738493027456495;
                } else {
                    expDiff += 1;
                    if expDiff == 0 {
                        current_block = 5848014816482081386;
                    } else {
                        current_block = 8693738493027456495;
                    }
                }
                if current_block == 8693738493027456495 {
                    sigA = softfloat_shiftRightJam128(sigA.v64, sigA.v0, -expDiff as u64);
                }
                expZ = expB;
                sigB.v64 |= 0x10000000000000_u64;
                current_block = 1401634934106713100;
            }
        } else if expA == 0x7fff_i32 as i64 {
            if sigA.v64 | sigA.v0 | sigB.v64 | sigB.v0 != 0 {
                current_block = 8872083469674517140;
            } else {
                softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
                uiZ.v64 = 0x7fff800000000000_u64;
                uiZ.v0 = 0_u64;
                current_block = 4589634476707811940;
            }
        } else {
            expZ = expA;
            if expZ == 0 {
                expZ = 1_i32 as i64;
            }
            if sigB.v64 < sigA.v64 {
                current_block = 10773517525802023192;
            } else if sigA.v64 < sigB.v64 {
                current_block = 1401634934106713100;
            } else if sigB.v0 < sigA.v0 {
                current_block = 10773517525802023192;
            } else if sigA.v0 < sigB.v0 {
                current_block = 1401634934106713100;
            } else {
                uiZ.v64 = (((softfloat_roundingMode as i32 == softfloat_round_min as i32) as i32
                    as u64)
                    << 63_i32)
                    .wrapping_add((0_i32 as u64) << 48_i32)
                    .wrapping_add(0_i32 as u64);
                uiZ.v0 = 0_i32 as u64;
                current_block = 4589634476707811940;
            }
        }
        match current_block {
            8872083469674517140 => {}
            4589634476707811940 => {}
            10773517525802023192 => {}
            _ => {
                signZ = !signZ;
                sigZ = softfloat_sub128(sigB.v64, sigB.v0, sigA.v64, sigA.v0);
                current_block = 9586450114633277242;
            }
        }
    }
    match current_block {
        8872083469674517140 => {
            uiZ = softfloat_propagateNaNF128UI(uiA64, uiA0, uiB64, uiB0);
            current_block = 4589634476707811940;
        }
        10773517525802023192 => {
            sigZ = softfloat_sub128(sigA.v64, sigA.v0, sigB.v64, sigB.v0);
            current_block = 9586450114633277242;
        }
        _ => {}
    }
    match current_block {
        9586450114633277242 => {
            softfloat_normRoundPackToF128(signZ, expZ - 5_i32 as i64, sigZ.v64, sigZ.v0)
        }
        _ => {
            uZ.ui = uiZ;
            uZ.f
        }
    }
}
