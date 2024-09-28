use super::*;

pub unsafe fn f128_rem(mut a: float128_t, mut b: float128_t) -> float128_t {
    let mut current_block: u64;
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: u64;
    let mut uiA0: u64;
    let mut signA: bool;
    let mut expA: i64;
    let mut sigA: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uB: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiB64: u64;
    let mut uiB0: u64;
    let mut expB: i64;
    let mut sigB: uint128 = uint128 { v0: 0, v64: 0 };
    let mut normExpSig: exp32_sig128;
    let mut rem: uint128;
    let mut expDiff: i64;
    let mut q: u64;
    let mut recip32: u64;
    let mut q64: u64;
    let mut term: uint128;
    let mut altRem: uint128 = uint128 { v0: 0, v64: 0 };
    let mut meanRem: uint128;
    let mut signRem: bool;
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    signA = uiA64 >> 63_i32 != 0;
    expA = (uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sigA.v64 = uiA64 & 0xffffffffffff;
    sigA.v0 = uiA0;
    uB.f = b;
    uiB64 = uB.ui.v64;
    uiB0 = uB.ui.v0;
    expB = (uiB64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sigB.v64 = uiB64 & 0xffffffffffff;
    sigB.v0 = uiB0;
    if expA == 0x7fff_i32 as i64 {
        if sigA.v64 | sigA.v0 != 0 || expB == 0x7fff_i32 as i64 && sigB.v64 | sigB.v0 != 0 {
            current_block = 2938443861848483734;
        } else {
            current_block = 6177188189607132427;
        }
    } else if expB == 0x7fff_i32 as i64 {
        if sigB.v64 | sigB.v0 != 0 {
            current_block = 2938443861848483734;
        } else {
            return a;
        }
    } else {
        if expB == 0 {
            if sigB.v64 | sigB.v0 == 0 {
                current_block = 6177188189607132427;
            } else {
                normExpSig = softfloat_normSubnormalF128Sig(sigB.v64, sigB.v0);
                expB = normExpSig.exp;
                sigB = normExpSig.sig;
                current_block = 13472856163611868459;
            }
        } else {
            current_block = 13472856163611868459;
        }
        match current_block {
            6177188189607132427 => {}
            _ => {
                if expA == 0 {
                    if sigA.v64 | sigA.v0 == 0 {
                        return a;
                    }
                    normExpSig = softfloat_normSubnormalF128Sig(sigA.v64, sigA.v0);
                    expA = normExpSig.exp;
                    sigA = normExpSig.sig;
                }
                sigA.v64 |= 0x1000000000000;
                sigB.v64 |= 0x1000000000000_u64;
                rem = sigA;
                expDiff = expA - expB;
                if expDiff < 1_i32 as i64 {
                    if expDiff < -1_i32 as i64 {
                        return a;
                    }
                    if expDiff != 0 {
                        expB -= 1;
                        sigB = softfloat_add128(sigB.v64, sigB.v0, sigB.v64, sigB.v0);
                        q = 0_i32 as u64;
                    } else {
                        q = softfloat_le128(sigB.v64, sigB.v0, rem.v64, rem.v0) as u64;
                        if q != 0 {
                            rem = softfloat_sub128(rem.v64, rem.v0, sigB.v64, sigB.v0);
                        }
                    }
                    current_block = 5159818223158340697;
                } else {
                    recip32 = 0x7fffffffffffffff_u64 as u32 as u64;
                    expDiff -= 30_i32 as i64;
                    loop {
                        q64 = ((rem.v64 >> 19_i32) as u32 as u64).wrapping_mul(recip32);
                        if expDiff < 0_i32 as i64 {
                            break;
                        }
                        q = q64.wrapping_add(0x80000000_u32 as u64) >> 32_i32;
                        rem = softfloat_shortShiftLeft128(rem.v64, rem.v0, 29_i32 as u8);
                        term = softfloat_mul128By32(sigB.v64, sigB.v0, q as u32);
                        rem = softfloat_sub128(rem.v64, rem.v0, term.v64, term.v0);
                        if rem.v64 & 0x8000000000000000 != 0 {
                            rem = softfloat_add128(rem.v64, rem.v0, sigB.v64, sigB.v0);
                        }
                        expDiff -= 29_i32 as i64;
                    }
                    q = ((q64 >> 32_i32) as u32 >> (!expDiff & 31_i32 as i64)) as u64;
                    rem = softfloat_shortShiftLeft128(
                        rem.v64,
                        rem.v0,
                        (expDiff + 30_i32 as i64) as u8,
                    );
                    term = softfloat_mul128By32(sigB.v64, sigB.v0, q as u32);
                    rem = softfloat_sub128(rem.v64, rem.v0, term.v64, term.v0);
                    if rem.v64 & 0x8000000000000000 != 0 {
                        altRem = softfloat_add128(rem.v64, rem.v0, sigB.v64, sigB.v0);
                        current_block = 10361468463296557780;
                    } else {
                        current_block = 5159818223158340697;
                    }
                }
                if current_block == 5159818223158340697 {
                    loop {
                        altRem = rem;
                        q = q.wrapping_add(1);
                        rem = softfloat_sub128(rem.v64, rem.v0, sigB.v64, sigB.v0);
                        if rem.v64 & 0x8000000000000000 != 0 {
                            break;
                        }
                    }
                }
                meanRem = softfloat_add128(rem.v64, rem.v0, altRem.v64, altRem.v0);
                if meanRem.v64 & 0x8000000000000000 != 0
                    || meanRem.v64 | meanRem.v0 == 0 && q & 1_i32 as u64 != 0
                {
                    rem = altRem;
                }
                signRem = signA;
                if rem.v64 & 0x8000000000000000 != 0 {
                    signRem = !signRem;
                    rem = softfloat_sub128(0_i32 as u64, 0_i32 as u64, rem.v64, rem.v0);
                }
                return softfloat_normRoundPackToF128(
                    signRem,
                    expB - 1_i32 as i64,
                    rem.v64,
                    rem.v0,
                );
            }
        }
    }
    match current_block {
        2938443861848483734 => {
            uiZ = softfloat_propagateNaNF128UI(uiA64, uiA0, uiB64, uiB0);
        }
        _ => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            uiZ.v64 = 0x7fff800000000000;
            uiZ.v0 = 0;
        }
    }
    uZ.ui = uiZ;
    uZ.f
}
