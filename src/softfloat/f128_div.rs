use super::*;

pub unsafe fn f128_div(mut a: float128_t, mut b: float128_t) -> float128_t {
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
    let mut signB: bool;
    let mut expB: i64;
    let mut sigB: uint128 = uint128 { v0: 0, v64: 0 };
    let mut signZ: bool;
    let mut normExpSig: exp32_sig128;
    let mut expZ: i64;
    let mut rem: uint128;
    let mut recip32: u64;
    let mut ix: i32;
    let mut q64: u64;
    let mut q: u64;
    let mut term: uint128;
    let mut qs: [u64; 3] = [0; 3];
    let mut sigZExtra: u64;
    let mut sigZ: uint128;
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    signA = uiA64 >> 63_i32 != 0;
    expA = (uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sigA.v64 = uiA64 & 0xffffffffffff_u64;
    sigA.v0 = uiA0;
    uB.f = b;
    uiB64 = uB.ui.v64;
    uiB0 = uB.ui.v0;
    signB = uiB64 >> 63_i32 != 0;
    expB = (uiB64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sigB.v64 = uiB64 & 0xffffffffffff_u64;
    sigB.v0 = uiB0;
    signZ = signA as i32 ^ signB as i32 != 0;
    if expA == 0x7fff_i32 as i64 {
        if sigA.v64 | sigA.v0 != 0 {
            current_block = 17755040026855196725;
        } else if expB == 0x7fff_i32 as i64 {
            if sigB.v64 | sigB.v0 != 0 {
                current_block = 17755040026855196725;
            } else {
                current_block = 5723914486740819442;
            }
        } else {
            current_block = 2778482485192577870;
        }
    } else {
        if expB == 0x7fff_i32 as i64 {
            if sigB.v64 | sigB.v0 != 0 {
                current_block = 17755040026855196725;
            } else {
                current_block = 1928768250582374294;
            }
        } else {
            if expB == 0 {
                if sigB.v64 | sigB.v0 == 0 {
                    if expA as u64 | sigA.v64 | sigA.v0 == 0 {
                        current_block = 5723914486740819442;
                    } else {
                        softfloat_raiseFlags(softfloat_flag_infinite as i32 as u8);
                        current_block = 2778482485192577870;
                    }
                } else {
                    normExpSig = softfloat_normSubnormalF128Sig(sigB.v64, sigB.v0);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 17281240262373992796;
                }
            } else {
                current_block = 17281240262373992796;
            }
            match current_block {
                2778482485192577870 => {}
                5723914486740819442 => {}
                _ => {
                    if expA == 0 {
                        if sigA.v64 | sigA.v0 == 0 {
                            current_block = 1928768250582374294;
                        } else {
                            normExpSig = softfloat_normSubnormalF128Sig(sigA.v64, sigA.v0);
                            expA = normExpSig.exp;
                            sigA = normExpSig.sig;
                            current_block = 7828949454673616476;
                        }
                    } else {
                        current_block = 7828949454673616476;
                    }
                    match current_block {
                        1928768250582374294 => {}
                        _ => {
                            expZ = expA - expB + 0x3ffe_i32 as i64;
                            sigA.v64 |= 0x1000000000000_u64;
                            sigB.v64 |= 0x1000000000000_u64;
                            rem = sigA;
                            if softfloat_lt128(sigA.v64, sigA.v0, sigB.v64, sigB.v0) {
                                expZ -= 1;
                                rem = softfloat_add128(sigA.v64, sigA.v0, sigA.v64, sigA.v0);
                            }
                            recip32 = 0x7fffffffffffffff_u64
                                .wrapping_div((sigB.v64 >> 17_i32) as u32 as u64)
                                as u32 as u64;
                            ix = 3_i32;
                            loop {
                                q64 = ((rem.v64 >> 19_i32) as u32 as u64).wrapping_mul(recip32);
                                q = q64.wrapping_add(0x80000000_u32 as u64) >> 32_i32;
                                ix -= 1;
                                if ix < 0_i32 {
                                    break;
                                }
                                rem = softfloat_shortShiftLeft128(rem.v64, rem.v0, 29_i32 as u8);
                                term = softfloat_mul128By32(sigB.v64, sigB.v0, q as u32);
                                rem = softfloat_sub128(rem.v64, rem.v0, term.v64, term.v0);
                                if rem.v64 & 0x8000000000000000_u64 != 0 {
                                    q = q.wrapping_sub(1);
                                    rem = softfloat_add128(rem.v64, rem.v0, sigB.v64, sigB.v0);
                                }
                                qs[ix as usize] = q;
                            }
                            if (q.wrapping_add(1_i32 as u64) & 7_i32 as u64) < 2_i32 as u64 {
                                rem = softfloat_shortShiftLeft128(rem.v64, rem.v0, 29_i32 as u8);
                                term = softfloat_mul128By32(sigB.v64, sigB.v0, q as u32);
                                rem = softfloat_sub128(rem.v64, rem.v0, term.v64, term.v0);
                                if rem.v64 & 0x8000000000000000_u64 != 0 {
                                    q = q.wrapping_sub(1);
                                    rem = softfloat_add128(rem.v64, rem.v0, sigB.v64, sigB.v0);
                                } else if softfloat_le128(sigB.v64, sigB.v0, rem.v64, rem.v0) {
                                    q = q.wrapping_add(1);
                                    rem = softfloat_sub128(rem.v64, rem.v0, sigB.v64, sigB.v0);
                                }
                                if rem.v64 | rem.v0 != 0 {
                                    q |= 1_i32 as u64;
                                }
                            }
                            sigZExtra = q << 60_i32;
                            term = softfloat_shortShiftLeft128(0_i32 as u64, qs[1], 54_i32 as u8);
                            sigZ = softfloat_add128(
                                qs[2] << 19_i32,
                                (qs[0] << 25_i32).wrapping_add(q >> 4_i32),
                                term.v64,
                                term.v0,
                            );
                            return softfloat_roundPackToF128(
                                signZ, expZ, sigZ.v64, sigZ.v0, sigZExtra,
                            );
                        }
                    }
                }
            }
        }
        match current_block {
            17755040026855196725 => {}
            2778482485192577870 => {}
            5723914486740819442 => {}
            _ => {
                uiZ.v64 = ((signZ as u64) << 63_i32)
                    .wrapping_add((0_i32 as u64) << 48_i32)
                    .wrapping_add(0_i32 as u64);
                current_block = 4402128107817972972;
            }
        }
    }
    match current_block {
        5723914486740819442 => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            uiZ.v64 = 0x7fff800000000000_u64;
            uiZ.v0 = 0_u64;
            current_block = 9670605402105114866;
        }
        17755040026855196725 => {
            uiZ = softfloat_propagateNaNF128UI(uiA64, uiA0, uiB64, uiB0);
            current_block = 9670605402105114866;
        }
        2778482485192577870 => {
            uiZ.v64 = ((signZ as u64) << 63_i32)
                .wrapping_add((0x7fff_i32 as u64) << 48_i32)
                .wrapping_add(0_i32 as u64);
            current_block = 4402128107817972972;
        }
        _ => {}
    }
    if current_block == 4402128107817972972 {
        uiZ.v0 = 0_i32 as u64;
    }
    uZ.ui = uiZ;
    uZ.f
}
