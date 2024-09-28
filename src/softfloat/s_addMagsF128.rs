use super::*;

pub unsafe fn softfloat_addMagsF128(
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
    let mut expDiff: i64;
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut sigZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut expZ: i64 = 0;
    let mut sigZExtra: u64 = 0;
    let mut sig128Extra: uint128_extra;
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    expA = (uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sigA.v64 = uiA64 & 0xffffffffffff_u64;
    sigA.v0 = uiA0;
    expB = (uiB64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sigB.v64 = uiB64 & 0xffffffffffff_u64;
    sigB.v0 = uiB0;
    expDiff = expA - expB;
    if expDiff == 0 {
        if expA == 0x7fff_i32 as i64 {
            if sigA.v64 | sigA.v0 | sigB.v64 | sigB.v0 != 0 {
                current_block = 18406198425875837302;
            } else {
                uiZ.v64 = uiA64;
                uiZ.v0 = uiA0;
                current_block = 7996727877381210085;
            }
        } else {
            sigZ = softfloat_add128(sigA.v64, sigA.v0, sigB.v64, sigB.v0);
            if expA == 0 {
                uiZ.v64 = ((signZ as u64) << 63_i32)
                    .wrapping_add((0_i32 as u64) << 48_i32)
                    .wrapping_add(sigZ.v64);
                uiZ.v0 = sigZ.v0;
                current_block = 7996727877381210085;
            } else {
                expZ = expA;
                sigZ.v64 |= 0x2000000000000_u64;
                sigZExtra = 0_i32 as u64;
                current_block = 2347939273064072389;
            }
        }
    } else {
        if expDiff < 0_i32 as i64 {
            if expB == 0x7fff_i32 as i64 {
                if sigB.v64 | sigB.v0 != 0 {
                    current_block = 18406198425875837302;
                } else {
                    uiZ.v64 = ((signZ as u64) << 63_i32)
                        .wrapping_add((0x7fff_i32 as u64) << 48_i32)
                        .wrapping_add(0_i32 as u64);
                    uiZ.v0 = 0_i32 as u64;
                    current_block = 7996727877381210085;
                }
            } else {
                expZ = expB;
                if expA != 0 {
                    sigA.v64 |= 0x1000000000000_u64;
                    current_block = 14648156034262866959;
                } else {
                    expDiff += 1;
                    sigZExtra = 0_i32 as u64;
                    if expDiff == 0 {
                        current_block = 5327858935536843014;
                    } else {
                        current_block = 14648156034262866959;
                    }
                }
                match current_block {
                    5327858935536843014 => {}
                    _ => {
                        sig128Extra = softfloat_shiftRightJam128Extra(
                            sigA.v64,
                            sigA.v0,
                            0_i32 as u64,
                            -expDiff as u64,
                        );
                        sigA = sig128Extra.v;
                        sigZExtra = sig128Extra.extra;
                        current_block = 5327858935536843014;
                    }
                }
            }
        } else if expA == 0x7fff_i32 as i64 {
            if sigA.v64 | sigA.v0 != 0 {
                current_block = 18406198425875837302;
            } else {
                uiZ.v64 = uiA64;
                uiZ.v0 = uiA0;
                current_block = 7996727877381210085;
            }
        } else {
            expZ = expA;
            if expB != 0 {
                sigB.v64 |= 0x1000000000000_u64;
                current_block = 15090052786889560393;
            } else {
                expDiff -= 1;
                sigZExtra = 0_i32 as u64;
                if expDiff == 0 {
                    current_block = 5327858935536843014;
                } else {
                    current_block = 15090052786889560393;
                }
            }
            match current_block {
                5327858935536843014 => {}
                _ => {
                    sig128Extra = softfloat_shiftRightJam128Extra(
                        sigB.v64,
                        sigB.v0,
                        0_i32 as u64,
                        expDiff as u64,
                    );
                    sigB = sig128Extra.v;
                    sigZExtra = sig128Extra.extra;
                    current_block = 5327858935536843014;
                }
            }
        }
        match current_block {
            18406198425875837302 => {}
            7996727877381210085 => {}
            _ => {
                sigZ = softfloat_add128(sigA.v64 | 0x1000000000000_u64, sigA.v0, sigB.v64, sigB.v0);
                expZ -= 1;
                if sigZ.v64 < 0x2000000000000_u64 {
                    current_block = 2592824566028266318;
                } else {
                    expZ += 1;
                    current_block = 2347939273064072389;
                }
            }
        }
    }
    match current_block {
        2347939273064072389 => {
            sig128Extra =
                softfloat_shortShiftRightJam128Extra(sigZ.v64, sigZ.v0, sigZExtra, 1_i32 as u8);
            sigZ = sig128Extra.v;
            sigZExtra = sig128Extra.extra;
            current_block = 2592824566028266318;
        }
        18406198425875837302 => {
            uiZ = softfloat_propagateNaNF128UI(uiA64, uiA0, uiB64, uiB0);
            current_block = 7996727877381210085;
        }
        _ => {}
    }
    match current_block {
        2592824566028266318 => softfloat_roundPackToF128(signZ, expZ, sigZ.v64, sigZ.v0, sigZExtra),
        _ => {
            uZ.ui = uiZ;
            uZ.f
        }
    }
}
