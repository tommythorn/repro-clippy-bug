use super::*;

pub unsafe fn f16_mul(mut a: float16_t, mut b: float16_t) -> float16_t {
    let mut current_block: u64;
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: u64;
    let mut signA: bool;
    let mut expA: i8;
    let mut sigA: u64;
    let mut uB: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiB: u64;
    let mut signB: bool;
    let mut expB: i8;
    let mut sigB: u64;
    let mut signZ: bool;
    let mut magBits: u64 = 0;
    let mut normExpSig: exp8_sig16;
    let mut expZ: i8;
    let mut sig32Z: u64;
    let mut sigZ: u64;
    let mut uiZ: u64 = 0;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as u64;
    signA = uiA as u16 as i32 >> 15_i32 != 0;
    expA = ((uiA >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    sigA = uiA & 0x3ff_i32 as u64;
    uB.f = b;
    uiB = uB.ui as u64;
    signB = uiB as u16 as i32 >> 15_i32 != 0;
    expB = ((uiB >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    sigB = uiB & 0x3ff_i32 as u64;
    signZ = signA as i32 ^ signB as i32 != 0;
    if expA as i32 == 0x1f_i32 {
        if sigA != 0 || expB as i32 == 0x1f_i32 && sigB != 0 {
            current_block = 8129404864280543644;
        } else {
            magBits = expB as u64 | sigB;
            current_block = 621899588027633843;
        }
    } else if expB as i32 == 0x1f_i32 {
        if sigB != 0 {
            current_block = 8129404864280543644;
        } else {
            magBits = expA as u64 | sigA;
            current_block = 621899588027633843;
        }
    } else {
        if expA == 0 {
            if sigA == 0 {
                current_block = 14535906770034819430;
            } else {
                normExpSig = softfloat_normSubnormalF16Sig(sigA);
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
                    current_block = 14535906770034819430;
                } else {
                    normExpSig = softfloat_normSubnormalF16Sig(sigB);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 15925075030174552612;
                }
            } else {
                current_block = 15925075030174552612;
            }
            match current_block {
                14535906770034819430 => {}
                _ => {
                    expZ = (expA as i32 + expB as i32 - 0xf_i32) as i8;
                    sigA = (sigA | 0x400_i32 as u64) << 4_i32;
                    sigB = (sigB | 0x400_i32 as u64) << 5_i32;
                    sig32Z = sigA.wrapping_mul(sigB);
                    sigZ = sig32Z >> 16_i32;
                    if sig32Z & 0xffff_i32 as u64 != 0 {
                        sigZ |= 1_i32 as u64;
                    }
                    if sigZ < 0x4000_i32 as u64 {
                        expZ -= 1;
                        sigZ <<= 1_i32;
                    }
                    return softfloat_roundPackToF16(signZ, expZ as i64, sigZ);
                }
            }
        }
        uiZ =
            (((signZ as u16 as i32) << 15_i32) + ((0_i32 as u16 as i32) << 10_i32) + 0_i32) as u64;
        current_block = 14854530145617030033;
    }
    match current_block {
        621899588027633843 => {
            if magBits == 0 {
                softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
                uiZ = 0x7e00_i32 as u64;
            } else {
                uiZ = (((signZ as u16 as i32) << 15_i32)
                    + ((0x1f_i32 as u16 as i32) << 10_i32)
                    + 0_i32) as u64;
            }
        }
        8129404864280543644 => {
            uiZ = softfloat_propagateNaNF16UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ as u16;
    uZ.f
}
