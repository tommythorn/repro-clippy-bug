use super::*;

pub unsafe fn f16_rem(mut a: float16_t, mut b: float16_t) -> float16_t {
    let mut current_block: u64;
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: u64;
    let mut signA: bool;
    let mut expA: i8;
    let mut sigA: u64;
    let mut uB: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiB: u64;
    let mut expB: i8;
    let mut sigB: u64;
    let mut normExpSig: exp8_sig16;
    let mut rem: u16;
    let mut expDiff: i8;
    let mut q: u64;
    let mut recip32: u32;
    let mut q32: u32;
    let mut altRem: u16;
    let mut meanRem: u16;
    let mut signRem: bool;
    let mut uiZ: u64;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as u64;
    signA = uiA as u16 as i32 >> 15_i32 != 0;
    expA = ((uiA >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    sigA = uiA & 0x3ff_i32 as u64;
    uB.f = b;
    uiB = uB.ui as u64;
    expB = ((uiB >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    sigB = uiB & 0x3ff_i32 as u64;
    if expA as i32 == 0x1f_i32 {
        if sigA != 0 || expB as i32 == 0x1f_i32 && sigB != 0 {
            current_block = 11959088493454959076;
        } else {
            current_block = 16339100164532869450;
        }
    } else if expB as i32 == 0x1f_i32 {
        if sigB != 0 {
            current_block = 11959088493454959076;
        } else {
            return a;
        }
    } else {
        if expB == 0 {
            if sigB == 0 {
                current_block = 16339100164532869450;
            } else {
                normExpSig = softfloat_normSubnormalF16Sig(sigB);
                expB = normExpSig.exp;
                sigB = normExpSig.sig;
                current_block = 15089075282327824602;
            }
        } else {
            current_block = 15089075282327824602;
        }
        match current_block {
            16339100164532869450 => {}
            _ => {
                if expA == 0 {
                    if sigA == 0 {
                        return a;
                    }
                    normExpSig = softfloat_normSubnormalF16Sig(sigA);
                    expA = normExpSig.exp;
                    sigA = normExpSig.sig;
                }
                rem = (sigA | 0x400_i32 as u64) as u16;
                sigB |= 0x400_i32 as u64;
                expDiff = (expA as i32 - expB as i32) as i8;
                if (expDiff as i32) < 1_i32 {
                    if (expDiff as i32) < -1_i32 {
                        return a;
                    }
                    sigB <<= 3_i32;
                    if expDiff != 0 {
                        rem = ((rem as i32) << 2_i32) as u16;
                        q = 0_i32 as u64;
                    } else {
                        rem = ((rem as i32) << 3_i32) as u16;
                        q = (sigB <= rem as u64) as i32 as u64;
                        if q != 0 {
                            rem = (rem as u64).wrapping_sub(sigB) as u16;
                        }
                    }
                } else {
                    recip32 =
                        0x7fffffffffffffff_u64.wrapping_div((sigB << 21_i32) as u32 as u64) as u32;
                    rem = ((rem as i32) << 4_i32) as u16;
                    expDiff = (expDiff as i32 - 31_i32) as i8;
                    sigB <<= 3_i32;
                    loop {
                        q32 = ((rem as u64 * recip32 as u64) >> 16_i32) as u32;
                        if (expDiff as i32) < 0_i32 {
                            break;
                        }
                        rem = (q32 as u64 * sigB).wrapping_neg() as u16;
                        expDiff = (expDiff as i32 - 29_i32) as i8;
                    }
                    q32 >>= !(expDiff as i32) & 31_i32;
                    q = q32 as u64;
                    rem = (((rem as i32) << (expDiff as i32 + 30_i32)) as u64)
                        .wrapping_sub(q * sigB) as u16;
                }
                loop {
                    altRem = rem;
                    q = q.wrapping_add(1);
                    rem = (rem as u64).wrapping_sub(sigB) as u16;
                    if rem as i32 & 0x8000 != 0 {
                        break;
                    }
                }
                meanRem = (rem as i32 + altRem as i32) as u16;
                if meanRem as i32 & 0x8000 != 0 || meanRem == 0 && q & 1_i32 as u64 != 0 {
                    rem = altRem;
                }
                signRem = signA;
                if 0x8000_i32 <= rem as i32 {
                    signRem = !signRem;
                    rem = -(rem as i32) as u16;
                }
                return softfloat_normRoundPackToF16(signRem, expB as i64, rem as u64);
            }
        }
    }
    match current_block {
        11959088493454959076 => {
            uiZ = softfloat_propagateNaNF16UI(uiA, uiB);
        }
        _ => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            uiZ = 0x7e00_i32 as u64;
        }
    }
    uZ.ui = uiZ as u16;
    uZ.f
}
