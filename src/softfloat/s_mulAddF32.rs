use super::*;

pub unsafe fn softfloat_mulAddF32(uiA: u64, uiB: u64, uiC: u64, op: u8) -> float32_t {
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
    let mut signProd: bool;
    let mut magBits: u64 = 0;
    let mut uiZ: u64 = 0;
    let mut normExpSig: exp16_sig32;
    let mut expProd: i64;
    let mut sigProd: u64;
    let mut signZ: bool;
    let mut expZ: i64 = 0;
    let mut sigZ: u64 = 0;
    let mut expDiff: i64;
    let mut sig64Z: u64;
    let mut sig64C: u64;
    let mut shiftDist: i8;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    signA = uiA as u32 >> 31_i32 != 0;
    expA = (uiA >> 23_i32) as i64 & 0xff_i32 as i64;
    sigA = uiA & 0x7fffff_i32 as u64;
    signB = uiB as u32 >> 31_i32 != 0;
    expB = (uiB >> 23_i32) as i64 & 0xff_i32 as i64;
    sigB = uiB & 0x7fffff_i32 as u64;
    signC = (uiC as u32 >> 31_i32 != 0) as i32
        ^ (op as i32 & softfloat_mulAdd_subC as i32 != 0) as i32
        != 0;
    expC = (uiC >> 23_i32) as i64 & 0xff_i32 as i64;
    sigC = uiC & 0x7fffff_i32 as u64;
    signProd =
        signA as i32 ^ signB as i32 ^ (op as i32 & softfloat_mulAdd_subProd as i32 != 0) as i32
            != 0;
    if expA == 0xff_i32 as i64 {
        if sigA != 0 || expB == 0xff_i32 as i64 && sigB != 0 {
            current_block = 10764608489117203088;
        } else {
            magBits = expB as u64 | sigB;
            current_block = 5385507874081489488;
        }
    } else if expB == 0xff_i32 as i64 {
        if sigB != 0 {
            current_block = 10764608489117203088;
        } else {
            magBits = expA as u64 | sigA;
            current_block = 5385507874081489488;
        }
    } else if expC == 0xff_i32 as i64 {
        if sigC != 0 {
            uiZ = 0_i32 as u64;
            current_block = 16549504898602067278;
        } else {
            uiZ = uiC;
            current_block = 8312099986019096830;
        }
    } else {
        if expA == 0 {
            if sigA == 0 {
                current_block = 1744461665653307565;
            } else {
                normExpSig = softfloat_normSubnormalF32Sig(sigA);
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
                    current_block = 1744461665653307565;
                } else {
                    normExpSig = softfloat_normSubnormalF32Sig(sigB);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 18377268871191777778;
                }
            } else {
                current_block = 18377268871191777778;
            }
            match current_block {
                1744461665653307565 => {}
                _ => {
                    expProd = expA + expB
                        - 0x7e_i32 as i64
                        - (op as i32 & softfloat_mulAdd_divProd as i32 != 0) as i32 as i64;
                    sigA = (sigA | 0x800000_i32 as u64) << 7_i32;
                    sigB = (sigB | 0x800000_i32 as u64) << 7_i32;
                    sigProd = sigA.wrapping_mul(sigB);
                    if sigProd < 0x2000000000000000_u64 {
                        expProd -= 1;
                        sigProd <<= 1_i32;
                    }
                    signZ = signProd;
                    if expC == 0 {
                        if sigC == 0 {
                            expZ = expProd - 1_i32 as i64;
                            sigZ = softfloat_shortShiftRightJam64(sigProd, 31_i32 as u8);
                            current_block = 17525380575600059357;
                        } else {
                            normExpSig = softfloat_normSubnormalF32Sig(sigC);
                            expC = normExpSig.exp;
                            sigC = normExpSig.sig;
                            current_block = 1356832168064818221;
                        }
                    } else {
                        current_block = 1356832168064818221;
                    }
                    if current_block == 1356832168064818221 {
                        sigC = (sigC | 0x800000_i32 as u64) << 6_i32;
                        expDiff = expProd - expC;
                        if signProd as i32 == signC as i32 {
                            if expDiff <= 0_i32 as i64 {
                                expZ = expC;
                                sigZ = sigC.wrapping_add(softfloat_shiftRightJam64(
                                    sigProd,
                                    (32_i32 as i64 - expDiff) as u64,
                                ));
                            } else {
                                expZ = expProd;
                                sig64Z = sigProd.wrapping_add(softfloat_shiftRightJam64(
                                    sigC << 32_i32,
                                    expDiff as u64,
                                ));
                                sigZ = softfloat_shortShiftRightJam64(sig64Z, 32_i32 as u8);
                            }
                            if sigZ < 0x40000000_i32 as u64 {
                                expZ -= 1;
                                sigZ <<= 1_i32;
                            }
                            current_block = 17525380575600059357;
                        } else {
                            sig64C = sigC << 32_i32;
                            if expDiff < 0_i32 as i64 {
                                signZ = signC;
                                expZ = expC;
                                sig64Z = sig64C.wrapping_sub(softfloat_shiftRightJam64(
                                    sigProd,
                                    -expDiff as u64,
                                ));
                                current_block = 10853015579903106591;
                            } else if expDiff == 0 {
                                expZ = expProd;
                                sig64Z = sigProd.wrapping_sub(sig64C);
                                if sig64Z == 0 {
                                    current_block = 17455168223632645183;
                                } else {
                                    if sig64Z & 0x8000000000000000_u64 != 0 {
                                        signZ = !signZ;
                                        sig64Z = sig64Z.wrapping_neg();
                                    }
                                    current_block = 10853015579903106591;
                                }
                            } else {
                                expZ = expProd;
                                sig64Z = sigProd.wrapping_sub(softfloat_shiftRightJam64(
                                    sig64C,
                                    expDiff as u64,
                                ));
                                current_block = 10853015579903106591;
                            }
                            match current_block {
                                17455168223632645183 => {}
                                _ => {
                                    shiftDist = (softfloat_countLeadingZeros64(sig64Z) as i32
                                        - 1_i32)
                                        as i8;
                                    expZ -= shiftDist as i64;
                                    shiftDist = (shiftDist as i32 - 32_i32) as i8;
                                    if (shiftDist as i32) < 0_i32 {
                                        sigZ = softfloat_shortShiftRightJam64(
                                            sig64Z,
                                            -(shiftDist as i32) as u8,
                                        );
                                    } else {
                                        sigZ = sig64Z << shiftDist as i32;
                                    }
                                    current_block = 17525380575600059357;
                                }
                            }
                        }
                    }
                    match current_block {
                        17455168223632645183 => {}
                        _ => return softfloat_roundPackToF32(signZ, expZ, sigZ),
                    }
                }
            }
        }
        if current_block == 1744461665653307565 {
            uiZ = uiC;
            if expC as u64 | sigC == 0 && signProd as i32 != signC as i32 {
                current_block = 17455168223632645183;
            } else {
                current_block = 8312099986019096830;
            }
        }
        match current_block {
            8312099986019096830 => {}
            _ => {
                uiZ = (((softfloat_roundingMode as i32 == softfloat_round_min as i32) as i32
                    as u32)
                    << 31_i32)
                    .wrapping_add((0_i32 as u32) << 23_i32)
                    .wrapping_add(0_i32 as u32) as u64;
                current_block = 8312099986019096830;
            }
        }
    }
    match current_block {
        5385507874081489488 => {
            if magBits != 0 {
                uiZ = ((signProd as u32) << 31_i32)
                    .wrapping_add((0xff_i32 as u32) << 23_i32)
                    .wrapping_add(0_i32 as u32) as u64;
                if expC != 0xff_i32 as i64 {
                    current_block = 8312099986019096830;
                } else if sigC != 0 {
                    current_block = 16549504898602067278;
                } else if signProd as i32 == signC as i32 {
                    current_block = 8312099986019096830;
                } else {
                    current_block = 14648606000749551097;
                }
            } else {
                current_block = 14648606000749551097;
            }
            match current_block {
                8312099986019096830 => {}
                16549504898602067278 => {}
                _ => {
                    softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
                    uiZ = 0x7fc00000_i32 as u64;
                    current_block = 16549504898602067278;
                }
            }
        }
        10764608489117203088 => {
            uiZ = softfloat_propagateNaNF32UI(uiA, uiB);
            current_block = 16549504898602067278;
        }
        _ => {}
    }
    if current_block == 16549504898602067278 {
        uiZ = softfloat_propagateNaNF32UI(uiZ, uiC);
    }
    uZ.ui = uiZ as u32;
    uZ.f
}
