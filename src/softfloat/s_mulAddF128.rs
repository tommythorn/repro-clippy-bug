use super::*;

unsafe fn softfloat_shortShiftRight128(mut a64: u64, mut a0: u64, mut dist: u8) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v64 = a64 >> dist as i32;
    z.v0 = a64 << (-(dist as i32) & 63_i32) | a0 >> dist as i32;
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
pub unsafe fn softfloat_mulAddF128(
    mut uiA64: u64,
    mut uiA0: u64,
    mut uiB64: u64,
    mut uiB0: u64,
    mut uiC64: u64,
    mut uiC0: u64,
    mut op: u8,
) -> float128_t {
    let mut current_block: u64;
    let mut signA: bool;
    let mut expA: i64;
    let mut sigA: uint128 = uint128 { v0: 0, v64: 0 };
    let mut signB: bool;
    let mut expB: i64;
    let mut sigB: uint128 = uint128 { v0: 0, v64: 0 };
    let mut signC: bool;
    let mut expC: i64;
    let mut sigC: uint128 = uint128 { v0: 0, v64: 0 };
    let mut signZ: bool;
    let mut magBits: u64 = 0;
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut normExpSig: exp32_sig128;
    let mut expZ: i64;
    let mut sig256Z: [u64; 4] = [0; 4];
    let mut sigZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut shiftDist: i64;
    let mut expDiff: i64;
    let mut x128: uint128;
    let mut sig256C: [u64; 4] = [0; 4];
    static mut zero256: [u64; 4] = [0_i32 as u64, 0_i32 as u64, 0_i32 as u64, 0_i32 as u64];
    let mut sigZExtra: u64 = 0;
    let mut sig256Z0: u64;
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    signA = uiA64 >> 63_i32 != 0;
    expA = (uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sigA.v64 = uiA64 & 0xffffffffffff_u64;
    sigA.v0 = uiA0;
    signB = uiB64 >> 63_i32 != 0;
    expB = (uiB64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sigB.v64 = uiB64 & 0xffffffffffff_u64;
    sigB.v0 = uiB0;
    signC =
        (uiC64 >> 63_i32 != 0) as i32 ^ (op as i32 & softfloat_mulAdd_subC as i32 != 0) as i32 != 0;
    expC = (uiC64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sigC.v64 = uiC64 & 0xffffffffffff_u64;
    sigC.v0 = uiC0;
    signZ = signA as i32 ^ signB as i32 ^ (op as i32 & softfloat_mulAdd_subProd as i32 != 0) as i32
        != 0;
    if expA == 0x7fff_i32 as i64 {
        if sigA.v64 | sigA.v0 != 0 || expB == 0x7fff_i32 as i64 && sigB.v64 | sigB.v0 != 0 {
            current_block = 8392289706439947825;
        } else {
            magBits = expB as u64 | sigB.v64 | sigB.v0;
            current_block = 4080187094696179670;
        }
    } else if expB == 0x7fff_i32 as i64 {
        if sigB.v64 | sigB.v0 != 0 {
            current_block = 8392289706439947825;
        } else {
            magBits = expA as u64 | sigA.v64 | sigA.v0;
            current_block = 4080187094696179670;
        }
    } else if expC == 0x7fff_i32 as i64 {
        if sigC.v64 | sigC.v0 != 0 {
            uiZ.v64 = 0_i32 as u64;
            uiZ.v0 = 0_i32 as u64;
            current_block = 12339431241327163586;
        } else {
            uiZ.v64 = uiC64;
            uiZ.v0 = uiC0;
            current_block = 11686949731212533496;
        }
    } else {
        if expA == 0 {
            if sigA.v64 | sigA.v0 == 0 {
                current_block = 6907636495220783645;
            } else {
                normExpSig = softfloat_normSubnormalF128Sig(sigA.v64, sigA.v0);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
                current_block = 652864300344834934;
            }
        } else {
            current_block = 652864300344834934;
        }
        if current_block == 652864300344834934 {
            if expB == 0 {
                if sigB.v64 | sigB.v0 == 0 {
                    current_block = 6907636495220783645;
                } else {
                    normExpSig = softfloat_normSubnormalF128Sig(sigB.v64, sigB.v0);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 1836292691772056875;
                }
            } else {
                current_block = 1836292691772056875;
            }
            match current_block {
                6907636495220783645 => {}
                _ => {
                    expZ = expA + expB - 0x3ffe_i32 as i64;
                    sigA.v64 |= 0x1000000000000_u64;
                    sigB.v64 |= 0x1000000000000_u64;
                    sigA = softfloat_shortShiftLeft128(sigA.v64, sigA.v0, 8_i32 as u8);
                    sigB = softfloat_shortShiftLeft128(sigB.v64, sigB.v0, 15_i32 as u8);
                    softfloat_mul128To256M(
                        sigA.v64,
                        sigA.v0,
                        sigB.v64,
                        sigB.v0,
                        sig256Z.as_mut_ptr(),
                    );
                    sigZ.v64 = sig256Z[3];
                    sigZ.v0 = sig256Z[2];
                    shiftDist = 0_i32 as i64;
                    if sigZ.v64 & 0x100000000000000_u64 == 0 {
                        expZ -= 1;
                        shiftDist = -1_i32 as i64;
                    }
                    if expC == 0 {
                        if sigC.v64 | sigC.v0 == 0 {
                            shiftDist += 8_i32 as i64;
                            current_block = 11291966035886851253;
                        } else {
                            normExpSig = softfloat_normSubnormalF128Sig(sigC.v64, sigC.v0);
                            expC = normExpSig.exp;
                            sigC = normExpSig.sig;
                            current_block = 13678349939556791712;
                        }
                    } else {
                        current_block = 13678349939556791712;
                    }
                    if current_block == 13678349939556791712 {
                        sigC.v64 |= 0x1000000000000_u64;
                        sigC = softfloat_shortShiftLeft128(sigC.v64, sigC.v0, 8_i32 as u8);
                        expDiff = expZ - expC;
                        if expDiff < 0_i32 as i64 {
                            expZ = expC;
                            if signZ as i32 == signC as i32 || expDiff < -1_i32 as i64 {
                                shiftDist -= expDiff;
                                if shiftDist != 0 {
                                    sigZ = softfloat_shiftRightJam128(
                                        sigZ.v64,
                                        sigZ.v0,
                                        shiftDist as u64,
                                    );
                                }
                            } else if shiftDist == 0 {
                                x128 = softfloat_shortShiftRight128(
                                    sig256Z[1],
                                    sig256Z[0],
                                    1_i32 as u8,
                                );
                                sig256Z[1] = sigZ.v0 << 63_i32 | x128.v64;
                                sig256Z[0] = x128.v0;
                                sigZ = softfloat_shortShiftRight128(sigZ.v64, sigZ.v0, 1_i32 as u8);
                                sig256Z[3] = sigZ.v64;
                                sig256Z[2] = sigZ.v0;
                            }
                        } else {
                            if shiftDist != 0 {
                                softfloat_add256M(
                                    sig256Z.as_mut_ptr(),
                                    sig256Z.as_mut_ptr(),
                                    sig256Z.as_mut_ptr(),
                                );
                            }
                            if expDiff == 0 {
                                sigZ.v64 = sig256Z[3];
                                sigZ.v0 = sig256Z[2];
                            } else {
                                sig256C[3] = sigC.v64;
                                sig256C[2] = sigC.v0;
                                sig256C[1] = 0_i32 as u64;
                                sig256C[0] = 0_i32 as u64;
                                softfloat_shiftRightJam256M(
                                    sig256C.as_mut_ptr(),
                                    expDiff as u64,
                                    sig256C.as_mut_ptr(),
                                );
                            }
                        }
                        shiftDist = 8_i32 as i64;
                        if signZ as i32 == signC as i32 {
                            if expDiff <= 0_i32 as i64 {
                                sigZ = softfloat_add128(sigC.v64, sigC.v0, sigZ.v64, sigZ.v0);
                            } else {
                                softfloat_add256M(
                                    sig256Z.as_mut_ptr(),
                                    sig256C.as_mut_ptr(),
                                    sig256Z.as_mut_ptr(),
                                );
                                sigZ.v64 = sig256Z[3];
                                sigZ.v0 = sig256Z[2];
                            }
                            if sigZ.v64 & 0x200000000000000_u64 != 0 {
                                expZ += 1;
                                shiftDist = 9_i32 as i64;
                            }
                            current_block = 11291966035886851253;
                        } else {
                            if expDiff < 0_i32 as i64 {
                                signZ = signC;
                                if expDiff < -1_i32 as i64 {
                                    sigZ = softfloat_sub128(sigC.v64, sigC.v0, sigZ.v64, sigZ.v0);
                                    sigZExtra = sig256Z[1] | sig256Z[0];
                                    if sigZExtra != 0 {
                                        sigZ = softfloat_sub128(
                                            sigZ.v64,
                                            sigZ.v0,
                                            0_i32 as u64,
                                            1_i32 as u64,
                                        );
                                    }
                                    if sigZ.v64 & 0x100000000000000_u64 == 0 {
                                        expZ -= 1;
                                        shiftDist = 7_i32 as i64;
                                    }
                                    current_block = 11185749240851133986;
                                } else {
                                    sig256C[3] = sigC.v64;
                                    sig256C[2] = sigC.v0;
                                    sig256C[1] = 0_i32 as u64;
                                    sig256C[0] = 0_i32 as u64;
                                    softfloat_sub256M(
                                        sig256C.as_mut_ptr(),
                                        sig256Z.as_mut_ptr(),
                                        sig256Z.as_mut_ptr(),
                                    );
                                    current_block = 6478348674394853609;
                                }
                            } else if expDiff == 0 {
                                sigZ = softfloat_sub128(sigZ.v64, sigZ.v0, sigC.v64, sigC.v0);
                                if sigZ.v64 | sigZ.v0 == 0 && sig256Z[1] == 0 && sig256Z[0] == 0 {
                                    current_block = 9631856976064065242;
                                } else {
                                    sig256Z[3] = sigZ.v64;
                                    sig256Z[2] = sigZ.v0;
                                    if sigZ.v64 & 0x8000000000000000_u64 != 0 {
                                        signZ = !signZ;
                                        softfloat_sub256M(
                                            zero256.as_ptr(),
                                            sig256Z.as_mut_ptr(),
                                            sig256Z.as_mut_ptr(),
                                        );
                                    }
                                    current_block = 6478348674394853609;
                                }
                            } else {
                                softfloat_sub256M(
                                    sig256Z.as_mut_ptr(),
                                    sig256C.as_mut_ptr(),
                                    sig256Z.as_mut_ptr(),
                                );
                                if (1_i32 as i64) < expDiff {
                                    sigZ.v64 = sig256Z[3];
                                    sigZ.v0 = sig256Z[2];
                                    if sigZ.v64 & 0x100000000000000_u64 == 0 {
                                        expZ -= 1;
                                        shiftDist = 7_i32 as i64;
                                    }
                                    current_block = 11291966035886851253;
                                } else {
                                    current_block = 6478348674394853609;
                                }
                            }
                            match current_block {
                                11185749240851133986 => {}
                                11291966035886851253 => {}
                                9631856976064065242 => {}
                                _ => {
                                    sigZ.v64 = sig256Z[3];
                                    sigZ.v0 = sig256Z[2];
                                    sigZExtra = sig256Z[1];
                                    sig256Z0 = sig256Z[0];
                                    if sigZ.v64 != 0 {
                                        if sig256Z0 != 0 {
                                            sigZExtra |= 1_i32 as u64;
                                        }
                                    } else {
                                        expZ -= 64_i32 as i64;
                                        sigZ.v64 = sigZ.v0;
                                        sigZ.v0 = sigZExtra;
                                        sigZExtra = sig256Z0;
                                        if sigZ.v64 == 0 {
                                            expZ -= 64_i32 as i64;
                                            sigZ.v64 = sigZ.v0;
                                            sigZ.v0 = sigZExtra;
                                            sigZExtra = 0_i32 as u64;
                                            if sigZ.v64 == 0 {
                                                expZ -= 64_i32 as i64;
                                                sigZ.v64 = sigZ.v0;
                                                sigZ.v0 = 0_i32 as u64;
                                            }
                                        }
                                    }
                                    shiftDist = softfloat_countLeadingZeros64(sigZ.v64) as i64;
                                    expZ += 7_i32 as i64 - shiftDist;
                                    shiftDist = 15_i32 as i64 - shiftDist;
                                    if (0_i32 as i64) < shiftDist {
                                        current_block = 11185749240851133986;
                                    } else {
                                        if shiftDist != 0 {
                                            shiftDist = -shiftDist;
                                            sigZ = softfloat_shortShiftLeft128(
                                                sigZ.v64,
                                                sigZ.v0,
                                                shiftDist as u8,
                                            );
                                            x128 = softfloat_shortShiftLeft128(
                                                0_i32 as u64,
                                                sigZExtra,
                                                shiftDist as u8,
                                            );
                                            sigZ.v0 |= x128.v64;
                                            sigZExtra = x128.v0;
                                        }
                                        current_block = 577773522850017442;
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        9631856976064065242 => {}
                        _ => {
                            if current_block == 11291966035886851253 {
                                sigZExtra = sig256Z[1] | sig256Z[0];
                                current_block = 11185749240851133986;
                            }
                            if current_block == 11185749240851133986 {
                                sigZExtra = sigZ.v0 << (64_i32 as i64 - shiftDist)
                                    | (sigZExtra != 0_i32 as u64) as i32 as u64;
                                sigZ = softfloat_shortShiftRight128(
                                    sigZ.v64,
                                    sigZ.v0,
                                    shiftDist as u8,
                                );
                            }
                            return softfloat_roundPackToF128(
                                signZ,
                                expZ - 1_i32 as i64,
                                sigZ.v64,
                                sigZ.v0,
                                sigZExtra,
                            );
                        }
                    }
                }
            }
        }
        if current_block == 6907636495220783645 {
            uiZ.v64 = uiC64;
            uiZ.v0 = uiC0;
            if expC as u64 | sigC.v64 | sigC.v0 == 0 && signZ as i32 != signC as i32 {
                current_block = 9631856976064065242;
            } else {
                current_block = 11686949731212533496;
            }
        }
        match current_block {
            11686949731212533496 => {}
            _ => {
                uiZ.v64 = (((softfloat_roundingMode as i32 == softfloat_round_min as i32) as i32
                    as u64)
                    << 63_i32)
                    .wrapping_add((0_i32 as u64) << 48_i32)
                    .wrapping_add(0_i32 as u64);
                uiZ.v0 = 0_i32 as u64;
                current_block = 11686949731212533496;
            }
        }
    }
    match current_block {
        4080187094696179670 => {
            if magBits != 0 {
                uiZ.v64 = ((signZ as u64) << 63_i32)
                    .wrapping_add((0x7fff_i32 as u64) << 48_i32)
                    .wrapping_add(0_i32 as u64);
                uiZ.v0 = 0_i32 as u64;
                if expC != 0x7fff_i32 as i64 {
                    current_block = 11686949731212533496;
                } else if sigC.v64 | sigC.v0 != 0 {
                    current_block = 12339431241327163586;
                } else if signZ as i32 == signC as i32 {
                    current_block = 11686949731212533496;
                } else {
                    current_block = 4330759529560430365;
                }
            } else {
                current_block = 4330759529560430365;
            }
            match current_block {
                11686949731212533496 => {}
                12339431241327163586 => {}
                _ => {
                    softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
                    uiZ.v64 = 0x7fff800000000000_u64;
                    uiZ.v0 = 0_u64;
                    current_block = 12339431241327163586;
                }
            }
        }
        8392289706439947825 => {
            uiZ = softfloat_propagateNaNF128UI(uiA64, uiA0, uiB64, uiB0);
            current_block = 12339431241327163586;
        }
        _ => {}
    }
    if current_block == 12339431241327163586 {
        uiZ = softfloat_propagateNaNF128UI(uiZ.v64, uiZ.v0, uiC64, uiC0);
    }
    uZ.ui = uiZ;
    uZ.f
}
