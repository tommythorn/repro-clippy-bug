use super::*;

pub unsafe fn softfloat_mulAddF16(uiA: u64, uiB: u64, uiC: u64, op: u8) -> float16_t {
    let mut current_block: u64;
    let mut signA: bool;
    let mut expA: i8;
    let mut sigA: u64;
    let mut signB: bool;
    let mut expB: i8;
    let mut sigB: u64;
    let mut signC: bool;
    let mut expC: i8;
    let mut sigC: u64;
    let mut signProd: bool;
    let mut magBits: u64 = 0;
    let mut uiZ: u64 = 0;
    let mut normExpSig: exp8_sig16;
    let mut expProd: i8;
    let mut sigProd: u64;
    let mut signZ: bool;
    let mut expZ: i8 = 0;
    let mut sigZ: u64 = 0;
    let mut expDiff: i8;
    let mut sig32Z: u64;
    let mut sig32C: u64;
    let mut shiftDist: i8;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    signA = uiA as u16 as i32 >> 15_i32 != 0;
    expA = ((uiA >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    sigA = uiA & 0x3ff_i32 as u64;
    signB = uiB as u16 as i32 >> 15_i32 != 0;
    expB = ((uiB >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    sigB = uiB & 0x3ff_i32 as u64;
    signC = (uiC as u16 as i32 >> 15_i32 != 0) as i32
        ^ (op as i32 & softfloat_mulAdd_subC as i32 != 0) as i32
        != 0;
    expC = ((uiC >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    sigC = uiC & 0x3ff_i32 as u64;
    signProd =
        signA as i32 ^ signB as i32 ^ (op as i32 & softfloat_mulAdd_subProd as i32 != 0) as i32
            != 0;
    if expA as i32 == 0x1f_i32 {
        if sigA != 0 || expB as i32 == 0x1f_i32 && sigB != 0 {
            current_block = 126393551249954250;
        } else {
            magBits = expB as u64 | sigB;
            current_block = 15187801862633971406;
        }
    } else if expB as i32 == 0x1f_i32 {
        if sigB != 0 {
            current_block = 126393551249954250;
        } else {
            magBits = expA as u64 | sigA;
            current_block = 15187801862633971406;
        }
    } else if expC as i32 == 0x1f_i32 {
        if sigC != 0 {
            uiZ = 0_i32 as u64;
            current_block = 10466619782625587572;
        } else {
            uiZ = uiC;
            current_block = 847182605244177763;
        }
    } else {
        if expA == 0 {
            if sigA == 0 {
                current_block = 17913173302991806887;
            } else {
                normExpSig = softfloat_normSubnormalF16Sig(sigA);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
                current_block = 3437258052017859086;
            }
        } else {
            current_block = 3437258052017859086;
        }
        if current_block == 3437258052017859086 {
            if expB == 0 {
                if sigB == 0 {
                    current_block = 17913173302991806887;
                } else {
                    normExpSig = softfloat_normSubnormalF16Sig(sigB);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 18377268871191777778;
                }
            } else {
                current_block = 18377268871191777778;
            }
            match current_block {
                17913173302991806887 => {}
                _ => {
                    expProd = (expA as i32 + expB as i32
                        - 0xe_i32
                        - (op as i32 & softfloat_mulAdd_divProd as i32 != 0) as i32)
                        as i8;
                    sigA = (sigA | 0x400_i32 as u64) << 4_i32;
                    sigB = (sigB | 0x400_i32 as u64) << 4_i32;
                    sigProd = sigA.wrapping_mul(sigB);
                    if sigProd < 0x20000000_i32 as u64 {
                        expProd -= 1;
                        sigProd <<= 1_i32;
                    }
                    signZ = signProd;
                    if expC == 0 {
                        if sigC == 0 {
                            expZ = (expProd as i32 - 1_i32) as i8;
                            sigZ = sigProd >> 15_i32
                                | (sigProd & 0x7fff_i32 as u64 != 0_i32 as u64) as i32 as u64;
                            current_block = 11816798777447075466;
                        } else {
                            normExpSig = softfloat_normSubnormalF16Sig(sigC);
                            expC = normExpSig.exp;
                            sigC = normExpSig.sig;
                            current_block = 1356832168064818221;
                        }
                    } else {
                        current_block = 1356832168064818221;
                    }
                    if current_block == 1356832168064818221 {
                        sigC = (sigC | 0x400_i32 as u64) << 3_i32;
                        expDiff = (expProd as i32 - expC as i32) as i8;
                        if signProd as i32 == signC as i32 {
                            if expDiff as i32 <= 0_i32 {
                                expZ = expC;
                                sigZ = sigC.wrapping_add(softfloat_shiftRightJam32(
                                    sigProd as u32,
                                    (16_i32 - expDiff as i32) as u64,
                                ) as u64);
                            } else {
                                expZ = expProd;
                                sig32Z = sigProd.wrapping_add(softfloat_shiftRightJam32(
                                    (sigC << 16_i32) as u32,
                                    expDiff as u64,
                                )
                                    as u64);
                                sigZ = sig32Z >> 16_i32
                                    | (sig32Z & 0xffff_i32 as u64 != 0_i32 as u64) as i32 as u64;
                            }
                            if sigZ < 0x4000_i32 as u64 {
                                expZ -= 1;
                                sigZ <<= 1_i32;
                            }
                            current_block = 11816798777447075466;
                        } else {
                            sig32C = sigC << 16_i32;
                            if (expDiff as i32) < 0_i32 {
                                signZ = signC;
                                expZ = expC;
                                sig32Z = sig32C.wrapping_sub(softfloat_shiftRightJam32(
                                    sigProd as u32,
                                    -(expDiff as i32) as u64,
                                )
                                    as u64);
                                current_block = 10853015579903106591;
                            } else if expDiff == 0 {
                                expZ = expProd;
                                sig32Z = sigProd.wrapping_sub(sig32C);
                                if sig32Z == 0 {
                                    current_block = 14322457122001261141;
                                } else {
                                    if sig32Z & 0x80000000_u32 as u64 != 0 {
                                        signZ = !signZ;
                                        sig32Z = sig32Z.wrapping_neg();
                                    }
                                    current_block = 10853015579903106591;
                                }
                            } else {
                                expZ = expProd;
                                sig32Z = sigProd.wrapping_sub(softfloat_shiftRightJam32(
                                    sig32C as u32,
                                    expDiff as u64,
                                )
                                    as u64);
                                current_block = 10853015579903106591;
                            }
                            match current_block {
                                14322457122001261141 => {}
                                _ => {
                                    shiftDist =
                                        (softfloat_countLeadingZeros32(sig32Z as u32) as i32
                                            - 1_i32) as i8;
                                    expZ = (expZ as i32 - shiftDist as i32) as i8;
                                    shiftDist = (shiftDist as i32 - 16_i32) as i8;
                                    if (shiftDist as i32) < 0_i32 {
                                        sigZ = sig32Z >> -(shiftDist as i32)
                                            | ((sig32Z << (shiftDist as i32 & 31_i32)) as u32
                                                != 0_i32 as u32)
                                                as i32
                                                as u64;
                                    } else {
                                        sigZ = sig32Z << shiftDist as i32;
                                    }
                                    current_block = 11816798777447075466;
                                }
                            }
                        }
                    }
                    match current_block {
                        14322457122001261141 => {}
                        _ => {
                            return softfloat_roundPackToF16(signZ, expZ as i64, sigZ);
                        }
                    }
                }
            }
        }
        if current_block == 17913173302991806887 {
            uiZ = uiC;
            if expC as u64 | sigC == 0 && signProd as i32 != signC as i32 {
                current_block = 14322457122001261141;
            } else {
                current_block = 847182605244177763;
            }
        }
        match current_block {
            847182605244177763 => {}
            _ => {
                uiZ = ((((softfloat_roundingMode as i32 == softfloat_round_min as i32) as i32 as u16
                    as i32)
                    << 15_i32)
                    + ((0_i32 as u16 as i32) << 10_i32)
                    + 0_i32) as u64;
                current_block = 847182605244177763;
            }
        }
    }
    match current_block {
        15187801862633971406 => {
            if magBits != 0 {
                uiZ = (((signProd as u16 as i32) << 15_i32)
                    + ((0x1f_i32 as u16 as i32) << 10_i32)
                    + 0_i32) as u64;
                if expC as i32 != 0x1f_i32 {
                    current_block = 847182605244177763;
                } else if sigC != 0 {
                    current_block = 10466619782625587572;
                } else if signProd as i32 == signC as i32 {
                    current_block = 847182605244177763;
                } else {
                    current_block = 14648606000749551097;
                }
            } else {
                current_block = 14648606000749551097;
            }
            match current_block {
                847182605244177763 => {}
                10466619782625587572 => {}
                _ => {
                    softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
                    uiZ = 0x7e00_i32 as u64;
                    current_block = 10466619782625587572;
                }
            }
        }
        126393551249954250 => {
            uiZ = softfloat_propagateNaNF16UI(uiA, uiB);
            current_block = 10466619782625587572;
        }
        _ => {}
    }
    if current_block == 10466619782625587572 {
        uiZ = softfloat_propagateNaNF16UI(uiZ, uiC);
    }
    uZ.ui = uiZ as u16;
    uZ.f
}
