use super::*;

pub unsafe fn f64_sqrt(mut a: float64_t) -> float64_t {
    let mut current_block: u64;
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut signA: bool;
    let mut expA: i64;
    let mut sigA: u64;
    let mut uiZ: u64 = 0;
    let mut normExpSig: exp16_sig64;
    let mut expZ: i64;
    let mut sig32A: u32;
    let mut recipSqrt32: u32;
    let mut sig32Z: u32;
    let mut rem: u64;
    let mut q: u32;
    let mut sigZ: u64;
    let mut shiftedSigZ: u64;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    uiA = uA.ui;
    signA = uiA >> 63_i32 != 0;
    expA = (uiA >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sigA = uiA & 0xfffffffffffff;
    if expA == 0x7ff_i32 as i64 {
        if sigA != 0 {
            uiZ = softfloat_propagateNaNF64UI(uiA, 0_i32 as u64);
            current_block = 16012456306191887598;
        } else {
            if !signA {
                return a;
            }
            current_block = 261048879173830877;
        }
    } else {
        if signA {
            if expA as u64 | sigA == 0 {
                return a;
            }
        } else {
            if expA == 0 {
                if sigA == 0 {
                    return a;
                }
                normExpSig = softfloat_normSubnormalF64Sig(sigA);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
            }
            expZ = ((expA - 0x3ff_i32 as i64) >> 1_i32) + 0x3fe_i32 as i64;
            expA &= 1_i32 as i64;
            sigA |= 0x10000000000000_u64;
            sig32A = (sigA >> 21_i32) as u32;
            recipSqrt32 = softfloat_approxRecipSqrt32_1(expA as u32, sig32A);
            sig32Z = ((sig32A as u64 * recipSqrt32 as u64) >> 32_i32) as u32;
            if expA != 0 {
                sigA <<= 8_i32;
                sig32Z >>= 1_i32;
            } else {
                sigA <<= 9_i32;
            }
            rem = sigA.wrapping_sub(sig32Z as u64 * sig32Z as u64);
            q = (((rem >> 2_i32) as u32 as u64 * recipSqrt32 as u64) >> 32_i32) as u32;
            sigZ = ((sig32Z as u64) << 32_i32 | (1_i32 << 5_i32) as u64)
                .wrapping_add((q as u64) << 3_i32);
            if (sigZ & 0x1ff_i32 as u64) < 0x22_i32 as u64 {
                sigZ &= !(0x3f_i32 as u64);
                shiftedSigZ = sigZ >> 6_i32;
                rem = (sigA << 52_i32).wrapping_sub(shiftedSigZ * shiftedSigZ);
                if rem & 0x8000000000000000_u64 != 0 {
                    sigZ = sigZ.wrapping_sub(1);
                } else if rem != 0 {
                    sigZ |= 1_i32 as u64;
                }
            }
            return softfloat_roundPackToF64(false, expZ, sigZ);
        }
        current_block = 261048879173830877;
    }
    if current_block == 261048879173830877 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        uiZ = 0x7ff8000000000000_u64;
    }
    uZ.ui = uiZ;
    uZ.f
}
