use super::*;

pub unsafe fn f64_mul(mut a: float64_t, mut b: float64_t) -> float64_t {
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
    let mut magBits: u64 = 0;
    let mut normExpSig: exp16_sig64;
    let mut expZ: i64;
    let mut sig128Z: uint128;
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
        if sigA != 0 || expB == 0x7ff_i32 as i64 && sigB != 0 {
            current_block = 728971354303024830;
        } else {
            magBits = expB as u64 | sigB;
            current_block = 2602379012213406804;
        }
    } else if expB == 0x7ff_i32 as i64 {
        if sigB != 0 {
            current_block = 728971354303024830;
        } else {
            magBits = expA as u64 | sigA;
            current_block = 2602379012213406804;
        }
    } else {
        if expA == 0 {
            if sigA == 0 {
                current_block = 3492211876320587303;
            } else {
                normExpSig = softfloat_normSubnormalF64Sig(sigA);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
                current_block = 11307063007268554308;
            }
        } else {
            current_block = 11307063007268554308;
        }
        if current_block == 11307063007268554308 {
            if expB == 0 {
                if sigB == 0 {
                    current_block = 3492211876320587303;
                } else {
                    normExpSig = softfloat_normSubnormalF64Sig(sigB);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 15925075030174552612;
                }
            } else {
                current_block = 15925075030174552612;
            }
            match current_block {
                3492211876320587303 => {}
                _ => {
                    expZ = expA + expB - 0x3ff_i32 as i64;
                    sigA = (sigA | 0x10000000000000_u64) << 10_i32;
                    sigB = (sigB | 0x10000000000000_u64) << 11_i32;
                    sig128Z = softfloat_mul64To128(sigA, sigB);
                    sigZ = sig128Z.v64 | (sig128Z.v0 != 0_i32 as u64) as i32 as u64;
                    if sigZ < 0x4000000000000000_u64 {
                        expZ -= 1;
                        sigZ <<= 1_i32;
                    }
                    return softfloat_roundPackToF64(signZ, expZ, sigZ);
                }
            }
        }
        uiZ = ((signZ as u64) << 63_i32)
            .wrapping_add((0_i32 as u64) << 52_i32)
            .wrapping_add(0_i32 as u64);
        current_block = 8553045381780209475;
    }
    match current_block {
        2602379012213406804 => {
            if magBits == 0 {
                softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
                uiZ = 0x7ff8000000000000_u64;
            } else {
                uiZ = ((signZ as u64) << 63_i32)
                    .wrapping_add((0x7ff_i32 as u64) << 52_i32)
                    .wrapping_add(0_i32 as u64);
            }
        }
        728971354303024830 => {
            uiZ = softfloat_propagateNaNF64UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ;
    uZ.f
}
