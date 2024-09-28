use super::*;

pub unsafe fn f64_rem(mut a: float64_t, mut b: float64_t) -> float64_t {
    let mut current_block: u64;
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut signA: bool;
    let mut expA: i64;
    let mut sigA: u64;
    let mut uB: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiB: u64;
    let mut expB: i64;
    let mut sigB: u64;
    let mut normExpSig: exp16_sig64;
    let mut rem: u64;
    let mut expDiff: i64;
    let mut q: u32;
    let mut recip32: u32;
    let mut q64: u64;
    let mut altRem: u64 = 0;
    let mut meanRem: u64;
    let mut signRem: bool;
    let mut uiZ: u64;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    uiA = uA.ui;
    signA = uiA >> 63_i32 != 0;
    expA = (uiA >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sigA = uiA & 0xfffffffffffff;
    uB.f = b;
    uiB = uB.ui;
    expB = (uiB >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sigB = uiB & 0xfffffffffffff;
    if expA == 0x7ff_i32 as i64 {
        if sigA != 0 || expB == 0x7ff_i32 as i64 && sigB != 0 {
            current_block = 10211953436301165640;
        } else {
            current_block = 280879431114604268;
        }
    } else if expB == 0x7ff_i32 as i64 {
        if sigB != 0 {
            current_block = 10211953436301165640;
        } else {
            return a;
        }
    } else {
        if expA < expB - 1_i32 as i64 {
            return a;
        }
        if expB == 0 {
            if sigB == 0 {
                current_block = 280879431114604268;
            } else {
                normExpSig = softfloat_normSubnormalF64Sig(sigB);
                expB = normExpSig.exp;
                sigB = normExpSig.sig;
                current_block = 14401909646449704462;
            }
        } else {
            current_block = 14401909646449704462;
        }
        match current_block {
            280879431114604268 => {}
            _ => {
                if expA == 0 {
                    if sigA == 0 {
                        return a;
                    }
                    normExpSig = softfloat_normSubnormalF64Sig(sigA);
                    expA = normExpSig.exp;
                    sigA = normExpSig.sig;
                }
                rem = sigA | 0x10000000000000_u64;
                sigB |= 0x10000000000000_u64;
                expDiff = expA - expB;
                if expDiff < 1_i32 as i64 {
                    if expDiff < -1_i32 as i64 {
                        return a;
                    }
                    sigB <<= 9_i32;
                    if expDiff != 0 {
                        rem <<= 8_i32;
                        q = 0_i32 as u32;
                    } else {
                        rem <<= 9_i32;
                        q = (sigB <= rem) as i32 as u32;
                        if q != 0 {
                            rem = rem.wrapping_sub(sigB);
                        }
                    }
                    current_block = 13763002826403452995;
                } else {
                    recip32 =
                        0x7fffffffffffffff_u64.wrapping_div((sigB >> 21_i32) as u32 as u64) as u32;
                    rem <<= 9_i32;
                    expDiff -= 30_i32 as i64;
                    sigB <<= 9_i32;
                    loop {
                        q64 = (rem >> 32_i32) as u32 as u64 * recip32 as u64;
                        if expDiff < 0_i32 as i64 {
                            break;
                        }
                        q = (q64.wrapping_add(0x80000000_u32 as u64) >> 32_i32) as u32;
                        rem <<= 29_i32;
                        rem = rem.wrapping_sub(q as u64 * sigB);
                        if rem & 0x8000000000000000_u64 != 0 {
                            rem = rem.wrapping_add(sigB);
                        }
                        expDiff -= 29_i32 as i64;
                    }
                    q = (q64 >> 32_i32) as u32 >> (!expDiff & 31_i32 as i64);
                    rem = (rem << (expDiff + 30_i32 as i64)).wrapping_sub(q as u64 * sigB);
                    if rem & 0x8000000000000000_u64 != 0 {
                        altRem = rem.wrapping_add(sigB);
                        current_block = 17542195032403455676;
                    } else {
                        current_block = 13763002826403452995;
                    }
                }
                if current_block == 13763002826403452995 {
                    loop {
                        altRem = rem;
                        q = q.wrapping_add(1);
                        rem = rem.wrapping_sub(sigB);
                        if rem & 0x8000000000000000_u64 != 0 {
                            break;
                        }
                    }
                }
                meanRem = rem.wrapping_add(altRem);
                if meanRem & 0x8000000000000000_u64 != 0 || meanRem == 0 && q & 1_i32 as u32 != 0 {
                    rem = altRem;
                }
                signRem = signA;
                if rem & 0x8000000000000000_u64 != 0 {
                    signRem = !signRem;
                    rem = rem.wrapping_neg();
                }
                return softfloat_normRoundPackToF64(signRem, expB, rem);
            }
        }
    }
    match current_block {
        10211953436301165640 => {
            uiZ = softfloat_propagateNaNF64UI(uiA, uiB);
        }
        _ => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            uiZ = 0x7ff8000000000000_u64;
        }
    }
    uZ.ui = uiZ;
    uZ.f
}
