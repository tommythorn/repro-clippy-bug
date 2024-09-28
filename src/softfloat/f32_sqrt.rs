use super::*;

pub unsafe fn f32_sqrt(mut a: float32_t) -> float32_t {
    let mut current_block: u64;
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: u64;
    let mut signA: bool;
    let mut expA: i64;
    let mut sigA: u64;
    let mut uiZ: u64 = 0;
    let mut normExpSig: exp16_sig32;
    let mut expZ: i64;
    let mut sigZ: u64;
    let mut shiftedSigZ: u64;
    let mut negRem: u32;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as u64;
    signA = uiA as u32 >> 31_i32 != 0;
    expA = (uiA >> 23_i32) as i64 & 0xff_i32 as i64;
    sigA = uiA & 0x7fffff_i32 as u64;
    if expA == 0xff_i32 as i64 {
        if sigA != 0 {
            uiZ = softfloat_propagateNaNF32UI(uiA, 0_i32 as u64);
            current_block = 1831219958539456861;
        } else {
            if !signA {
                return a;
            }
            current_block = 2396609706071017823;
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
                normExpSig = softfloat_normSubnormalF32Sig(sigA);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
            }
            expZ = ((expA - 0x7f_i32 as i64) >> 1_i32) + 0x7e_i32 as i64;
            expA &= 1_i32 as i64;
            sigA = (sigA | 0x800000_i32 as u64) << 8_i32;
            sigZ =
                (sigA * softfloat_approxRecipSqrt32_1(expA as u32, sigA as u32) as u64) >> 32_i32;
            if expA != 0 {
                sigZ >>= 1_i32;
            }
            sigZ = sigZ.wrapping_add(2_i32 as u64);
            if (sigZ & 0x3f_i32 as u64) < 2_i32 as u64 {
                shiftedSigZ = sigZ >> 2_i32;
                negRem = (shiftedSigZ * shiftedSigZ) as u32;
                sigZ &= !3_i32 as u64;
                if negRem & 0x80000000_u32 != 0 {
                    sigZ |= 1_i32 as u64;
                } else if negRem != 0 {
                    sigZ = sigZ.wrapping_sub(1);
                }
            }
            return softfloat_roundPackToF32(false, expZ, sigZ);
        }
        current_block = 2396609706071017823;
    }
    if current_block == 2396609706071017823 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        uiZ = 0x7fc00000_i32 as u64;
    }
    uZ.ui = uiZ as u32;
    uZ.f
}
