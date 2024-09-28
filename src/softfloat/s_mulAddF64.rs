use super::*;

unsafe fn softfloat_shortShiftLeft128(mut a64: u64, mut a0: u64, mut dist: u8) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v64 = a64 << dist as i32 | a0 >> (-(dist as i32) & 63_i32);
    z.v0 = a0 << dist as i32;
    z
}
unsafe fn softfloat_shortShiftRightJam128(mut a64: u64, mut a0: u64, mut dist: u8) -> uint128 {
    let mut negDist: u8 = -(dist as i32) as u8;
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v64 = a64 >> dist as i32;
    z.v0 = a64 << (negDist as i32 & 63_i32)
        | a0 >> dist as i32
        | (a0 << (negDist as i32 & 63_i32) != 0_i32 as u64) as i32 as u64;
    z
}
unsafe fn softfloat_add128(mut a64: u64, mut a0: u64, mut b64: u64, mut b0: u64) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v0 = a0.wrapping_add(b0);
    z.v64 = a64
        .wrapping_add(b64)
        .wrapping_add((z.v0 < a0) as i32 as u64);
    z
}
unsafe fn softfloat_sub128(mut a64: u64, mut a0: u64, mut b64: u64, mut b0: u64) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v0 = a0.wrapping_sub(b0);
    z.v64 = a64.wrapping_sub(b64);
    z.v64 = (z.v64).wrapping_sub((a0 < b0) as i32 as u64);
    z
}
pub unsafe fn softfloat_mulAddF64(uiA: u64, uiB: u64, uiC: u64, op: u8) -> float64_t {
    let mut current_block: u64;
    let mut signA: bool;
    let mut expA: i64;
    let mut sigA: u64;
    let mut signB: bool;
    let mut expB: i64;
    let mut sigB: u64;
    let mut signC: bool;
    let mut expC: i64;
    let mut sigC: u64;
    let mut signZ: bool;
    let mut magBits: u64 = 0;
    let mut uiZ: u64 = 0;
    let mut normExpSig: exp16_sig64;
    let mut expZ: i64;
    let mut sig128Z: uint128;
    let mut sigZ: u64 = 0;
    let mut expDiff: i64;
    let mut sig128C: uint128 = uint128 { v0: 0, v64: 0 };
    let mut shiftDist: i8;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    signA = uiA >> 63_i32 != 0;
    expA = (uiA >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sigA = uiA & 0xfffffffffffff;
    signB = uiB >> 63_i32 != 0;
    expB = (uiB >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sigB = uiB & 0xfffffffffffff;
    signC =
        (uiC >> 63_i32 != 0) as i32 ^ (op as i32 & softfloat_mulAdd_subC as i32 != 0) as i32 != 0;
    expC = (uiC >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sigC = uiC & 0xfffffffffffff;
    signZ = signA as i32 ^ signB as i32 ^ (op as i32 & softfloat_mulAdd_subProd as i32 != 0) as i32
        != 0;
    if expA == 0x7ff_i32 as i64 {
        if sigA != 0 || expB == 0x7ff_i32 as i64 && sigB != 0 {
            current_block = 17866270555544197390;
        } else {
            magBits = expB as u64 | sigB;
            current_block = 5006632112578730736;
        }
    } else if expB == 0x7ff_i32 as i64 {
        if sigB != 0 {
            current_block = 17866270555544197390;
        } else {
            magBits = expA as u64 | sigA;
            current_block = 5006632112578730736;
        }
    } else if expC == 0x7ff_i32 as i64 {
        if sigC != 0 {
            uiZ = 0_i32 as u64;
            current_block = 13127819012675379487;
        } else {
            uiZ = uiC;
            current_block = 3628094004714986470;
        }
    } else {
        if expA == 0 {
            if sigA == 0 {
                current_block = 17015605727881603292;
            } else {
                normExpSig = softfloat_normSubnormalF64Sig(sigA);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
                current_block = 15925075030174552612;
            }
        } else {
            current_block = 15925075030174552612;
        }
        if current_block == 15925075030174552612 {
            if expB == 0 {
                if sigB == 0 {
                    current_block = 17015605727881603292;
                } else {
                    normExpSig = softfloat_normSubnormalF64Sig(sigB);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 5494826135382683477;
                }
            } else {
                current_block = 5494826135382683477;
            }
            match current_block {
                17015605727881603292 => {}
                _ => {
                    expZ = expA + expB
                        - 0x3fe_i32 as i64
                        - (op as i32 & softfloat_mulAdd_divProd as i32 != 0) as i32 as i64;
                    sigA = (sigA | 0x10000000000000_u64) << 10_i32;
                    sigB = (sigB | 0x10000000000000_u64) << 10_i32;
                    sig128Z = softfloat_mul64To128(sigA, sigB);
                    if sig128Z.v64 < 0x2000000000000000_u64 {
                        expZ -= 1;
                        sig128Z =
                            softfloat_add128(sig128Z.v64, sig128Z.v0, sig128Z.v64, sig128Z.v0);
                    }
                    if expC == 0 {
                        if sigC == 0 {
                            expZ -= 1;
                            sigZ =
                                sig128Z.v64 << 1_i32 | (sig128Z.v0 != 0_i32 as u64) as i32 as u64;
                            current_block = 310749485071864444;
                        } else {
                            normExpSig = softfloat_normSubnormalF64Sig(sigC);
                            expC = normExpSig.exp;
                            sigC = normExpSig.sig;
                            current_block = 2480299350034459858;
                        }
                    } else {
                        current_block = 2480299350034459858;
                    }
                    if current_block == 2480299350034459858 {
                        sigC = (sigC | 0x10000000000000_u64) << 9_i32;
                        expDiff = expZ - expC;
                        if expDiff < 0_i32 as i64 {
                            expZ = expC;
                            if signZ as i32 == signC as i32 || expDiff < -1_i32 as i64 {
                                sig128Z.v64 =
                                    softfloat_shiftRightJam64(sig128Z.v64, -expDiff as u64);
                            } else {
                                sig128Z = softfloat_shortShiftRightJam128(
                                    sig128Z.v64,
                                    sig128Z.v0,
                                    1_i32 as u8,
                                );
                            }
                        } else if expDiff != 0 {
                            sig128C =
                                softfloat_shiftRightJam128(sigC, 0_i32 as u64, expDiff as u64);
                        }
                        if signZ as i32 == signC as i32 {
                            if expDiff <= 0_i32 as i64 {
                                sigZ = sigC.wrapping_add(sig128Z.v64)
                                    | (sig128Z.v0 != 0_i32 as u64) as i32 as u64;
                            } else {
                                sig128Z = softfloat_add128(
                                    sig128Z.v64,
                                    sig128Z.v0,
                                    sig128C.v64,
                                    sig128C.v0,
                                );
                                sigZ = sig128Z.v64 | (sig128Z.v0 != 0_i32 as u64) as i32 as u64;
                            }
                            if sigZ < 0x4000000000000000_u64 {
                                expZ -= 1;
                                sigZ <<= 1_i32;
                            }
                            current_block = 310749485071864444;
                        } else {
                            if expDiff < 0_i32 as i64 {
                                signZ = signC;
                                sig128Z =
                                    softfloat_sub128(sigC, 0_i32 as u64, sig128Z.v64, sig128Z.v0);
                                current_block = 7158658067966855297;
                            } else if expDiff == 0 {
                                sig128Z.v64 = (sig128Z.v64).wrapping_sub(sigC);
                                if sig128Z.v64 | sig128Z.v0 == 0 {
                                    current_block = 1213874517124433823;
                                } else {
                                    if sig128Z.v64 & 0x8000000000000000_u64 != 0 {
                                        signZ = !signZ;
                                        sig128Z = softfloat_sub128(
                                            0_i32 as u64,
                                            0_i32 as u64,
                                            sig128Z.v64,
                                            sig128Z.v0,
                                        );
                                    }
                                    current_block = 7158658067966855297;
                                }
                            } else {
                                sig128Z = softfloat_sub128(
                                    sig128Z.v64,
                                    sig128Z.v0,
                                    sig128C.v64,
                                    sig128C.v0,
                                );
                                current_block = 7158658067966855297;
                            }
                            match current_block {
                                1213874517124433823 => {}
                                _ => {
                                    if sig128Z.v64 == 0 {
                                        expZ -= 64_i32 as i64;
                                        sig128Z.v64 = sig128Z.v0;
                                        sig128Z.v0 = 0_i32 as u64;
                                    }
                                    shiftDist = (softfloat_countLeadingZeros64(sig128Z.v64) as i32
                                        - 1_i32)
                                        as i8;
                                    expZ -= shiftDist as i64;
                                    if (shiftDist as i32) < 0_i32 {
                                        sigZ = softfloat_shortShiftRightJam64(
                                            sig128Z.v64,
                                            -(shiftDist as i32) as u8,
                                        );
                                    } else {
                                        sig128Z = softfloat_shortShiftLeft128(
                                            sig128Z.v64,
                                            sig128Z.v0,
                                            shiftDist as u8,
                                        );
                                        sigZ = sig128Z.v64;
                                    }
                                    sigZ |= (sig128Z.v0 != 0_i32 as u64) as i32 as u64;
                                    current_block = 310749485071864444;
                                }
                            }
                        }
                    }
                    match current_block {
                        1213874517124433823 => {}
                        _ => return softfloat_roundPackToF64(signZ, expZ, sigZ),
                    }
                }
            }
        }
        if current_block == 17015605727881603292 {
            uiZ = uiC;
            if expC as u64 | sigC == 0 && signZ as i32 != signC as i32 {
                current_block = 1213874517124433823;
            } else {
                current_block = 3628094004714986470;
            }
        }
        match current_block {
            3628094004714986470 => {}
            _ => {
                uiZ = (((softfloat_roundingMode as i32 == softfloat_round_min as i32) as i32
                    as u64)
                    << 63_i32)
                    .wrapping_add((0_i32 as u64) << 52_i32)
                    .wrapping_add(0_i32 as u64);
                current_block = 3628094004714986470;
            }
        }
    }
    match current_block {
        5006632112578730736 => {
            if magBits != 0 {
                uiZ = ((signZ as u64) << 63_i32)
                    .wrapping_add((0x7ff_i32 as u64) << 52_i32)
                    .wrapping_add(0_i32 as u64);
                if expC != 0x7ff_i32 as i64 {
                    current_block = 3628094004714986470;
                } else if sigC != 0 {
                    current_block = 13127819012675379487;
                } else if signZ as i32 == signC as i32 {
                    current_block = 3628094004714986470;
                } else {
                    current_block = 10887629115603254199;
                }
            } else {
                current_block = 10887629115603254199;
            }
            match current_block {
                3628094004714986470 => {}
                13127819012675379487 => {}
                _ => {
                    softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
                    uiZ = 0x7ff8000000000000_u64;
                    current_block = 13127819012675379487;
                }
            }
        }
        17866270555544197390 => {
            uiZ = softfloat_propagateNaNF64UI(uiA, uiB);
            current_block = 13127819012675379487;
        }
        _ => {}
    }
    if current_block == 13127819012675379487 {
        uiZ = softfloat_propagateNaNF64UI(uiZ, uiC);
    }
    uZ.ui = uiZ;
    uZ.f
}
