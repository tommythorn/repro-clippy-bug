use super::*;

pub unsafe fn f16_div(mut a: float16_t, mut b: float16_t) -> float16_t {
    let mut current_block: u64;
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: u64;
    let mut signA: bool;
    let mut expA: i8;
    let mut sigA: u64;
    let mut uB: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiB: u64;
    let mut signB: bool;
    let mut expB: i8;
    let mut sigB: u64;
    let mut signZ: bool;
    let mut normExpSig: exp8_sig16;
    let mut expZ: i8;
    let mut index: i32;
    let mut r0: u16;
    let mut sigZ: u64;
    let mut rem: u64;
    let mut uiZ: u64 = 0;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as u64;
    signA = uiA as u16 as i32 >> 15_i32 != 0;
    expA = ((uiA >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    sigA = uiA & 0x3ff_i32 as u64;
    uB.f = b;
    uiB = uB.ui as u64;
    signB = uiB as u16 as i32 >> 15_i32 != 0;
    expB = ((uiB >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    sigB = uiB & 0x3ff_i32 as u64;
    signZ = signA as i32 ^ signB as i32 != 0;
    if expA as i32 == 0x1f_i32 {
        if sigA != 0 {
            current_block = 17061621464629125492;
        } else if expB as i32 == 0x1f_i32 {
            if sigB != 0 {
                current_block = 17061621464629125492;
            } else {
                current_block = 17083517903683056759;
            }
        } else {
            current_block = 11252435696621018053;
        }
    } else {
        if expB as i32 == 0x1f_i32 {
            if sigB != 0 {
                current_block = 17061621464629125492;
            } else {
                current_block = 3537533804688903942;
            }
        } else {
            if expB == 0 {
                if sigB == 0 {
                    if expA as u64 | sigA == 0 {
                        current_block = 17083517903683056759;
                    } else {
                        softfloat_raiseFlags(softfloat_flag_infinite as i32 as u8);
                        current_block = 11252435696621018053;
                    }
                } else {
                    normExpSig = softfloat_normSubnormalF16Sig(sigB);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 15125582407903384992;
                }
            } else {
                current_block = 15125582407903384992;
            }
            match current_block {
                11252435696621018053 => {}
                17083517903683056759 => {}
                _ => {
                    if expA == 0 {
                        if sigA == 0 {
                            current_block = 3537533804688903942;
                        } else {
                            normExpSig = softfloat_normSubnormalF16Sig(sigA);
                            expA = normExpSig.exp;
                            sigA = normExpSig.sig;
                            current_block = 2569451025026770673;
                        }
                    } else {
                        current_block = 2569451025026770673;
                    }
                    match current_block {
                        3537533804688903942 => {}
                        _ => {
                            expZ = (expA as i32 - expB as i32 + 0xe_i32) as i8;
                            sigA |= 0x400_i32 as u64;
                            sigB |= 0x400_i32 as u64;
                            if sigA < sigB {
                                expZ -= 1;
                                sigA <<= 5_i32;
                            } else {
                                sigA <<= 4_i32;
                            }
                            index = (sigB >> 6_i32 & 0xf_i32 as u64) as i32;
                            r0 = (softfloat_approxRecip_1k0s[index as usize] as u64).wrapping_sub(
                                (softfloat_approxRecip_1k1s[index as usize] as u64)
                                    .wrapping_mul(sigB & 0x3f_i32 as u64)
                                    >> 10_i32,
                            ) as u16;
                            sigZ = (sigA * r0 as u64) >> 16_i32;
                            rem = (sigA << 10_i32).wrapping_sub(sigZ * sigB);
                            sigZ = sigZ.wrapping_add(rem.wrapping_mul(r0 as u64) >> 26_i32);
                            sigZ = sigZ.wrapping_add(1);
                            if sigZ & 7_i32 as u64 == 0 {
                                sigZ &= !1_i32 as u64;
                                rem = (sigA << 10_i32).wrapping_sub(sigZ * sigB);
                                if rem & 0x8000_i32 as u64 != 0 {
                                    sigZ = sigZ.wrapping_sub(2_i32 as u64);
                                } else if rem != 0 {
                                    sigZ |= 1_i32 as u64;
                                }
                            }
                            return softfloat_roundPackToF16(signZ, expZ as i64, sigZ);
                        }
                    }
                }
            }
        }
        match current_block {
            17061621464629125492 => {}
            11252435696621018053 => {}
            17083517903683056759 => {}
            _ => {
                uiZ = (((signZ as u16 as i32) << 15_i32)
                    + ((0_i32 as u16 as i32) << 10_i32)
                    + 0_i32) as u64;
                current_block = 16316101449799193626;
            }
        }
    }
    match current_block {
        17083517903683056759 => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            uiZ = 0x7e00_i32 as u64;
        }
        11252435696621018053 => {
            uiZ = (((signZ as u16 as i32) << 15_i32) + ((0x1f_i32 as u16 as i32) << 10_i32) + 0_i32)
                as u64;
        }
        17061621464629125492 => {
            uiZ = softfloat_propagateNaNF16UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ as u16;
    uZ.f
}
