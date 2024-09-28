use super::*;

pub unsafe fn f16_sqrt(mut a: float16_t) -> float16_t {
    let mut current_block: u64;
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: u64;
    let mut signA: bool;
    let mut expA: i8;
    let mut sigA: u64;
    let mut uiZ: u64 = 0;
    let mut normExpSig: exp8_sig16;
    let mut expZ: i8;
    let mut index: i32;
    let mut r0: u64;
    let mut ESqrR0: u64;
    let mut sigma0: u16;
    let mut recipSqrt16: u64;
    let mut sigZ: u64;
    let mut shiftedSigZ: u64;
    let mut negRem: u16;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as u64;
    signA = uiA as u16 as i32 >> 15_i32 != 0;
    expA = ((uiA >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    sigA = uiA & 0x3ff_i32 as u64;
    if expA as i32 == 0x1f_i32 {
        if sigA != 0 {
            uiZ = softfloat_propagateNaNF16UI(uiA, 0_i32 as u64);
            current_block = 16843452866975859139;
        } else {
            if !signA {
                return a;
            }
            current_block = 1885321959212250148;
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
                normExpSig = softfloat_normSubnormalF16Sig(sigA);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
            }
            expZ = (((expA as i32 - 0xf_i32) >> 1_i32) + 0xe_i32) as i8;
            expA = (expA as i32 & 1_i32) as i8;
            sigA |= 0x400_i32 as u64;
            index = (sigA >> 6_i32 & 0xe_i32 as u64).wrapping_add(expA as u64) as i32;
            r0 = (softfloat_approxRecipSqrt_1k0s[index as usize] as u64).wrapping_sub(
                (softfloat_approxRecipSqrt_1k1s[index as usize] as u64)
                    .wrapping_mul(sigA & 0x7f_i32 as u64)
                    >> 11_i32,
            );
            ESqrR0 = r0.wrapping_mul(r0) >> 1_i32;
            if expA != 0 {
                ESqrR0 >>= 1_i32;
            }
            sigma0 = !(ESqrR0.wrapping_mul(sigA) >> 16_i32) as u16;
            recipSqrt16 = r0.wrapping_add((r0 * sigma0 as u64) >> 25_i32);
            if recipSqrt16 & 0x8000_i32 as u64 == 0 {
                recipSqrt16 = 0x8000_i32 as u64;
            }
            sigZ = (sigA << 5_i32).wrapping_mul(recipSqrt16) >> 16_i32;
            if expA != 0 {
                sigZ >>= 1_i32;
            }
            sigZ = sigZ.wrapping_add(1);
            if sigZ & 7_i32 as u64 == 0 {
                shiftedSigZ = sigZ >> 1_i32;
                negRem = (shiftedSigZ * shiftedSigZ) as u16;
                sigZ &= !1_i32 as u64;
                if negRem as i32 & 0x8000 != 0 {
                    sigZ |= 1_i32 as u64;
                } else if negRem != 0 {
                    sigZ = sigZ.wrapping_sub(1);
                }
            }
            return softfloat_roundPackToF16(false, expZ as i64, sigZ);
        }
        current_block = 1885321959212250148;
    }
    if current_block == 1885321959212250148 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        uiZ = 0x7e00_i32 as u64;
    }
    uZ.ui = uiZ as u16;
    uZ.f
}
