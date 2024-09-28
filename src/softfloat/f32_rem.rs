use super::*;

pub unsafe fn f32_rem(mut a: float32_t, mut b: float32_t) -> float32_t {
    let mut current_block: u64;
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: u64;
    let mut signA: bool;
    let mut expA: i64;
    let mut sigA: u64;
    let mut uB: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiB: u64;
    let mut expB: i64;
    let mut sigB: u64;
    let mut normExpSig: exp16_sig32;
    let mut rem: u32;
    let mut expDiff: i64;
    let mut q: u32;
    let mut recip32: u32;
    let mut altRem: u32;
    let mut meanRem: u32;
    let mut signRem: bool;
    let mut uiZ: u64;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as u64;
    signA = uiA as u32 >> 31_i32 != 0;
    expA = (uiA >> 23_i32) as i64 & 0xff_i32 as i64;
    sigA = uiA & 0x7fffff_i32 as u64;
    uB.f = b;
    uiB = uB.ui as u64;
    expB = (uiB >> 23_i32) as i64 & 0xff_i32 as i64;
    sigB = uiB & 0x7fffff_i32 as u64;
    if expA == 0xff_i32 as i64 {
        if sigA != 0 || expB == 0xff_i32 as i64 && sigB != 0 {
            current_block = 2368704757508771949;
        } else {
            current_block = 17279350063348542575;
        }
    } else if expB == 0xff_i32 as i64 {
        if sigB != 0 {
            current_block = 2368704757508771949;
        } else {
            return a;
        }
    } else {
        if expB == 0 {
            if sigB == 0 {
                current_block = 17279350063348542575;
            } else {
                normExpSig = softfloat_normSubnormalF32Sig(sigB);
                expB = normExpSig.exp;
                sigB = normExpSig.sig;
                current_block = 12147880666119273379;
            }
        } else {
            current_block = 12147880666119273379;
        }
        match current_block {
            17279350063348542575 => {}
            _ => {
                if expA == 0 {
                    if sigA == 0 {
                        return a;
                    }
                    normExpSig = softfloat_normSubnormalF32Sig(sigA);
                    expA = normExpSig.exp;
                    sigA = normExpSig.sig;
                }
                rem = (sigA | 0x800000_i32 as u64) as u32;
                sigB |= 0x800000_i32 as u64;
                expDiff = expA - expB;
                if expDiff < 1_i32 as i64 {
                    if expDiff < -1_i32 as i64 {
                        return a;
                    }
                    sigB <<= 6_i32;
                    if expDiff != 0 {
                        rem <<= 5_i32;
                        q = 0_i32 as u32;
                    } else {
                        rem <<= 6_i32;
                        q = (sigB <= rem as u64) as i32 as u32;
                        if q != 0 {
                            rem = (rem as u64).wrapping_sub(sigB) as u32;
                        }
                    }
                } else {
                    recip32 =
                        0x7fffffffffffffff_u64.wrapping_div((sigB << 8_i32) as u32 as u64) as u32;
                    rem <<= 7_i32;
                    expDiff -= 31_i32 as i64;
                    sigB <<= 6_i32;
                    loop {
                        q = ((rem as u64 * recip32 as u64) >> 32_i32) as u32;
                        if expDiff < 0_i32 as i64 {
                            break;
                        }
                        rem = (q * sigB as u32).wrapping_neg();
                        expDiff -= 29_i32 as i64;
                    }
                    q >>= !expDiff & 31_i32 as i64;
                    rem = (rem << (expDiff + 30_i32 as i64)).wrapping_sub(q * sigB as u32);
                }
                loop {
                    altRem = rem;
                    q = q.wrapping_add(1);
                    rem = (rem as u64).wrapping_sub(sigB) as u32;
                    if rem & 0x80000000_u32 != 0 {
                        break;
                    }
                }
                meanRem = rem.wrapping_add(altRem);
                if meanRem & 0x80000000_u32 != 0 || meanRem == 0 && q & 1_i32 as u32 != 0 {
                    rem = altRem;
                }
                signRem = signA;
                if 0x80000000_u32 <= rem {
                    signRem = !signRem;
                    rem = rem.wrapping_neg();
                }
                return softfloat_normRoundPackToF32(signRem, expB, rem as u64);
            }
        }
    }
    match current_block {
        2368704757508771949 => {
            uiZ = softfloat_propagateNaNF32UI(uiA, uiB);
        }
        _ => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            uiZ = 0x7fc00000_i32 as u64;
        }
    }
    uZ.ui = uiZ as u32;
    uZ.f
}
