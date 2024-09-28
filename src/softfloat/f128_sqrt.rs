use super::*;

pub unsafe fn f128_sqrt(mut a: float128_t) -> float128_t {
    let mut current_block: u64;
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: u64;
    let mut uiA0: u64;
    let mut signA: bool;
    let mut expA: i64;
    let mut sigA: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut normExpSig: exp32_sig128;
    let mut expZ: i64;
    let mut sig32A: u64;
    let mut recipSqrt32: u64;
    let mut sig32Z: u64;
    let mut rem: uint128;
    let mut qs: [u32; 3] = [0; 3];
    let mut q: u64;
    let mut x64: u64;
    let mut sig64Z: u64;
    let mut y: uint128;
    let mut term: uint128;
    let mut sigZExtra: u64;
    let mut sigZ: uint128;
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
    if expA == 0x7fff_i32 as i64 {
        if sigA.v64 | sigA.v0 != 0 {
            uiZ = softfloat_propagateNaNF128UI(uiA64, uiA0, 0_i32 as u64, 0_i32 as u64);
            current_block = 12979388566066332591;
        } else {
            if !signA {
                return a;
            }
            current_block = 10078400233350910629;
        }
    } else {
        if signA {
            if expA as u64 | sigA.v64 | sigA.v0 == 0 {
                return a;
            }
        } else {
            if expA == 0 {
                if sigA.v64 | sigA.v0 == 0 {
                    return a;
                }
                normExpSig = softfloat_normSubnormalF128Sig(sigA.v64, sigA.v0);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
            }
            expZ = ((expA - 0x3fff_i32 as i64) >> 1_i32) + 0x3ffe_i32 as i64;
            expA &= 1_i32 as i64;
            sigA.v64 |= 0x1000000000000_u64;
            sig32A = sigA.v64 >> 17_i32;
            recipSqrt32 = softfloat_approxRecipSqrt32_1(expA as u32, sig32A as u32) as u64;
            sig32Z = sig32A.wrapping_mul(recipSqrt32) >> 32_i32;
            if expA != 0 {
                sig32Z >>= 1_i32;
                rem = softfloat_shortShiftLeft128(sigA.v64, sigA.v0, 12_i32 as u8);
            } else {
                rem = softfloat_shortShiftLeft128(sigA.v64, sigA.v0, 13_i32 as u8);
            }
            qs[2] = sig32Z as u32;
            rem.v64 = rem.v64.wrapping_sub(sig32Z.wrapping_mul(sig32Z));
            q = ((rem.v64 >> 2_i32) as u32 as u64 * recipSqrt32) >> 32_i32;
            x64 = sig32Z << 32_i32;
            sig64Z = x64.wrapping_add(q << 3_i32);
            y = softfloat_shortShiftLeft128(rem.v64, rem.v0, 29_i32 as u8);
            loop {
                term = softfloat_mul64ByShifted32To128(x64.wrapping_add(sig64Z), q as u32);
                rem = softfloat_sub128(y.v64, y.v0, term.v64, term.v0);
                if rem.v64 & 0x8000000000000000_u64 == 0 {
                    break;
                }
                q = q.wrapping_sub(1);
                sig64Z = sig64Z.wrapping_sub((1_i32 << 3_i32) as u64);
            }
            qs[1] = q as u32;
            q = (rem.v64 >> 2_i32).wrapping_mul(recipSqrt32) >> 32_i32;
            y = softfloat_shortShiftLeft128(rem.v64, rem.v0, 29_i32 as u8);
            sig64Z <<= 1_i32;
            loop {
                term = softfloat_shortShiftLeft128(0_i32 as u64, sig64Z, 32_i32 as u8);
                term = softfloat_add128(term.v64, term.v0, 0_i32 as u64, q << 6_i32);
                term = softfloat_mul128By32(term.v64, term.v0, q as u32);
                rem = softfloat_sub128(y.v64, y.v0, term.v64, term.v0);
                if rem.v64 & 0x8000000000000000_u64 == 0 {
                    break;
                }
                q = q.wrapping_sub(1);
            }
            qs[0] = q as u32;
            q = ((rem.v64 >> 2_i32).wrapping_mul(recipSqrt32) >> 32_i32).wrapping_add(2_i32 as u64);
            sigZExtra = q << 59_i32;
            term = softfloat_shortShiftLeft128(0_i32 as u64, qs[1] as u64, 53_i32 as u8);
            sigZ = softfloat_add128(
                (qs[2] as u64) << 18_i32,
                ((qs[0] as u64) << 24_i32).wrapping_add(q >> 5_i32),
                term.v64,
                term.v0,
            );
            if q & 0xf_i32 as u64 <= 2_i32 as u64 {
                q &= !3_i32 as u64;
                sigZExtra = q << 59_i32;
                y = softfloat_shortShiftLeft128(sigZ.v64, sigZ.v0, 6_i32 as u8);
                y.v0 |= sigZExtra >> 58_i32;
                term = softfloat_sub128(y.v64, y.v0, 0_i32 as u64, q);
                y = softfloat_mul64ByShifted32To128(term.v0, q as u32);
                term = softfloat_mul64ByShifted32To128(term.v64, q as u32);
                term = softfloat_add128(term.v64, term.v0, 0_i32 as u64, y.v64);
                rem = softfloat_shortShiftLeft128(rem.v64, rem.v0, 20_i32 as u8);
                term = softfloat_sub128(term.v64, term.v0, rem.v64, rem.v0);
                if term.v64 & 0x8000000000000000_u64 != 0 {
                    sigZExtra |= 1_i32 as u64;
                } else if term.v64 | term.v0 | y.v0 != 0 {
                    if sigZExtra != 0 {
                        sigZExtra = sigZExtra.wrapping_sub(1);
                    } else {
                        sigZ = softfloat_sub128(sigZ.v64, sigZ.v0, 0_i32 as u64, 1_i32 as u64);
                        sigZExtra = !0_i32 as u64;
                    }
                }
            }
            return softfloat_roundPackToF128(false, expZ, sigZ.v64, sigZ.v0, sigZExtra);
        }
        current_block = 10078400233350910629;
    }
    if current_block == 10078400233350910629 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        uiZ.v64 = 0x7fff800000000000_u64;
        uiZ.v0 = 0_u64;
    }
    uZ.ui = uiZ;
    uZ.f
}
