use super::*;

pub unsafe fn f64_div(mut a: float64_t, mut b: float64_t) -> float64_t {
    let mut current_block: u64;
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut signA: bool;
    let mut expA: i64;
    let mut sigA: u64;
    let mut uB: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiB: u64;
    let mut signB: bool;
    let mut expB: i64;
    let mut sigB: u64;
    let mut signZ: bool;
    let mut normExpSig: exp16_sig64;
    let mut expZ: i64;
    let mut recip32: u32;
    let mut sig32Z: u32;
    let mut doubleTerm: u32;
    let mut rem: u64;
    let mut q: u32;
    let mut sigZ: u64;
    let mut uiZ: u64 = 0;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    uiA = uA.ui;
    signA = uiA >> 63_i32 != 0;
    expA = (uiA >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sigA = uiA & 0xfffffffffffff;
    uB.f = b;
    uiB = uB.ui;
    signB = uiB >> 63_i32 != 0;
    expB = (uiB >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sigB = uiB & 0xfffffffffffff;
    signZ = signA as i32 ^ signB as i32 != 0;
    if expA == 0x7ff_i32 as i64 {
        if sigA != 0 {
            current_block = 14018410097322732368;
        } else if expB == 0x7ff_i32 as i64 {
            if sigB != 0 {
                current_block = 14018410097322732368;
            } else {
                current_block = 6995554918377399529;
            }
        } else {
            current_block = 17784365674037629921;
        }
    } else {
        if expB == 0x7ff_i32 as i64 {
            if sigB != 0 {
                current_block = 14018410097322732368;
            } else {
                current_block = 17062791351044807992;
            }
        } else {
            if expB == 0 {
                if sigB == 0 {
                    if expA as u64 | sigA == 0 {
                        current_block = 6995554918377399529;
                    } else {
                        softfloat_raiseFlags(softfloat_flag_infinite as i32 as u8);
                        current_block = 17784365674037629921;
                    }
                } else {
                    normExpSig = softfloat_normSubnormalF64Sig(sigB);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 14763689060501151050;
                }
            } else {
                current_block = 14763689060501151050;
            }
            match current_block {
                17784365674037629921 => {}
                6995554918377399529 => {}
                _ => {
                    if expA == 0 {
                        if sigA == 0 {
                            current_block = 17062791351044807992;
                        } else {
                            normExpSig = softfloat_normSubnormalF64Sig(sigA);
                            expA = normExpSig.exp;
                            sigA = normExpSig.sig;
                            current_block = 15897653523371991391;
                        }
                    } else {
                        current_block = 15897653523371991391;
                    }
                    match current_block {
                        17062791351044807992 => {}
                        _ => {
                            expZ = expA - expB + 0x3fe_i32 as i64;
                            sigA |= 0x10000000000000_u64;
                            sigB |= 0x10000000000000_u64;
                            if sigA < sigB {
                                expZ -= 1;
                                sigA <<= 11_i32;
                            } else {
                                sigA <<= 10_i32;
                            }
                            sigB <<= 11_i32;
                            recip32 = (0x7fffffffffffffff_u64
                                .wrapping_div((sigB >> 32_i32) as u32 as u64)
                                as u32)
                                .wrapping_sub(2_i32 as u32);
                            sig32Z = (((sigA >> 32_i32) as u32 as u64 * recip32 as u64) >> 32_i32)
                                as u32;
                            doubleTerm = sig32Z << 1_i32;
                            rem = (sigA
                                .wrapping_sub(doubleTerm as u64 * (sigB >> 32_i32) as u32 as u64)
                                << 28_i32)
                                .wrapping_sub(doubleTerm as u64 * (sigB as u32 >> 4_i32) as u64);
                            q = (((rem >> 32_i32) as u32 as u64 * recip32 as u64) >> 32_i32)
                                .wrapping_add(4_i32 as u64) as u32;
                            sigZ = ((sig32Z as u64) << 32_i32).wrapping_add((q as u64) << 4_i32);
                            if (sigZ & 0x1ff_i32 as u64) < (4_i32 << 4_i32) as u64 {
                                q &= !7_i32 as u32;
                                sigZ &= !(0x7f_i32 as u64);
                                doubleTerm = q << 1_i32;
                                rem = (rem.wrapping_sub(
                                    doubleTerm as u64 * (sigB >> 32_i32) as u32 as u64,
                                ) << 28_i32)
                                    .wrapping_sub(
                                        doubleTerm as u64 * (sigB as u32 >> 4_i32) as u64,
                                    );
                                if rem & 0x8000000000000000_u64 != 0 {
                                    sigZ = sigZ.wrapping_sub((1_i32 << 7_i32) as u64);
                                } else if rem != 0 {
                                    sigZ |= 1_i32 as u64;
                                }
                            }
                            return softfloat_roundPackToF64(signZ, expZ, sigZ);
                        }
                    }
                }
            }
        }
        match current_block {
            14018410097322732368 => {}
            17784365674037629921 => {}
            6995554918377399529 => {}
            _ => {
                uiZ = ((signZ as u64) << 63_i32)
                    .wrapping_add((0_i32 as u64) << 52_i32)
                    .wrapping_add(0_i32 as u64);
                current_block = 2573707172668831058;
            }
        }
    }
    match current_block {
        6995554918377399529 => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            uiZ = 0x7ff8000000000000_u64;
        }
        17784365674037629921 => {
            uiZ = ((signZ as u64) << 63_i32)
                .wrapping_add((0x7ff_i32 as u64) << 52_i32)
                .wrapping_add(0_i32 as u64);
        }
        14018410097322732368 => {
            uiZ = softfloat_propagateNaNF64UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ;
    uZ.f
}
