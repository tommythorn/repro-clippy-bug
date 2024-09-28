use super::*;

pub unsafe fn f32_div(mut a: float32_t, mut b: float32_t) -> float32_t {
    let mut current_block: u64;
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: u64;
    let mut signA: bool;
    let mut expA: i64;
    let mut sigA: u64;
    let mut uB: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiB: u64;
    let mut signB: bool;
    let mut expB: i64;
    let mut sigB: u64;
    let mut signZ: bool;
    let mut normExpSig: exp16_sig32;
    let mut expZ: i64;
    let mut sig64A: u64;
    let mut sigZ: u64;
    let mut uiZ: u64 = 0;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as u64;
    signA = uiA as u32 >> 31_i32 != 0;
    expA = (uiA >> 23_i32) as i64 & 0xff_i32 as i64;
    sigA = uiA & 0x7fffff_i32 as u64;
    uB.f = b;
    uiB = uB.ui as u64;
    signB = uiB as u32 >> 31_i32 != 0;
    expB = (uiB >> 23_i32) as i64 & 0xff_i32 as i64;
    sigB = uiB & 0x7fffff_i32 as u64;
    signZ = signA as i32 ^ signB as i32 != 0;
    if expA == 0xff_i32 as i64 {
        if sigA != 0 {
            current_block = 17970437205363635078;
        } else if expB == 0xff_i32 as i64 {
            if sigB != 0 {
                current_block = 17970437205363635078;
            } else {
                current_block = 1360432764087822320;
            }
        } else {
            current_block = 3389896622712769313;
        }
    } else {
        if expB == 0xff_i32 as i64 {
            if sigB != 0 {
                current_block = 17970437205363635078;
            } else {
                current_block = 12179215777628164084;
            }
        } else {
            if expB == 0 {
                if sigB == 0 {
                    if expA as u64 | sigA == 0 {
                        current_block = 1360432764087822320;
                    } else {
                        softfloat_raiseFlags(softfloat_flag_infinite as i32 as u8);
                        current_block = 3389896622712769313;
                    }
                } else {
                    normExpSig = softfloat_normSubnormalF32Sig(sigB);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 7056779235015430508;
                }
            } else {
                current_block = 7056779235015430508;
            }
            match current_block {
                3389896622712769313 => {}
                1360432764087822320 => {}
                _ => {
                    if expA == 0 {
                        if sigA == 0 {
                            current_block = 12179215777628164084;
                        } else {
                            normExpSig = softfloat_normSubnormalF32Sig(sigA);
                            expA = normExpSig.exp;
                            sigA = normExpSig.sig;
                            current_block = 11459959175219260272;
                        }
                    } else {
                        current_block = 11459959175219260272;
                    }
                    match current_block {
                        12179215777628164084 => {}
                        _ => {
                            expZ = expA - expB + 0x7e_i32 as i64;
                            sigA |= 0x800000_i32 as u64;
                            sigB |= 0x800000_i32 as u64;
                            if sigA < sigB {
                                expZ -= 1;
                                sig64A = sigA << 31_i32;
                            } else {
                                sig64A = sigA << 30_i32;
                            }
                            sigZ = sig64A.wrapping_div(sigB);
                            if sigZ & 0x3f_i32 as u64 == 0 {
                                sigZ |= (sigB.wrapping_mul(sigZ) != sig64A) as i32 as u64;
                            }
                            return softfloat_roundPackToF32(signZ, expZ, sigZ);
                        }
                    }
                }
            }
        }
        match current_block {
            17970437205363635078 => {}
            3389896622712769313 => {}
            1360432764087822320 => {}
            _ => {
                uiZ = ((signZ as u32) << 31_i32)
                    .wrapping_add((0_i32 as u32) << 23_i32)
                    .wrapping_add(0_i32 as u32) as u64;
                current_block = 16324846280944331837;
            }
        }
    }
    match current_block {
        1360432764087822320 => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            uiZ = 0x7fc00000_i32 as u64;
        }
        17970437205363635078 => {
            uiZ = softfloat_propagateNaNF32UI(uiA, uiB);
        }
        3389896622712769313 => {
            uiZ = ((signZ as u32) << 31_i32)
                .wrapping_add((0xff_i32 as u32) << 23_i32)
                .wrapping_add(0_i32 as u32) as u64;
        }
        _ => {}
    }
    uZ.ui = uiZ as u32;
    uZ.f
}
