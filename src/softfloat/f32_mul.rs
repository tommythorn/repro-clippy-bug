use super::*;

pub unsafe fn f32_mul(a: float32_t, b: float32_t) -> float32_t {
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
    let mut magBits: u64 = 0;
    let mut normExpSig: exp16_sig32;
    let mut expZ: i64;
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
        if sigA != 0 || expB == 0xff_i32 as i64 && sigB != 0 {
            current_block = 7211456467417213049;
        } else {
            magBits = expB as u64 | sigB;
            current_block = 3699674519062403871;
        }
    } else if expB == 0xff_i32 as i64 {
        if sigB != 0 {
            current_block = 7211456467417213049;
        } else {
            magBits = expA as u64 | sigA;
            current_block = 3699674519062403871;
        }
    } else {
        if expA == 0 {
            if sigA == 0 {
                current_block = 15659553351241700142;
            } else {
                normExpSig = softfloat_normSubnormalF32Sig(sigA);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
                current_block = 14401909646449704462;
            }
        } else {
            current_block = 14401909646449704462;
        }
        if current_block == 14401909646449704462 {
            if expB == 0 {
                if sigB == 0 {
                    current_block = 15659553351241700142;
                } else {
                    normExpSig = softfloat_normSubnormalF32Sig(sigB);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 14818589718467733107;
                }
            } else {
                current_block = 14818589718467733107;
            }
            match current_block {
                15659553351241700142 => {}
                _ => {
                    expZ = expA + expB - 0x7f_i32 as i64;
                    sigA = (sigA | 0x800000_i32 as u64) << 7_i32;
                    sigB = (sigB | 0x800000_i32 as u64) << 8_i32;
                    sigZ = softfloat_shortShiftRightJam64(sigA.wrapping_mul(sigB), 32_i32 as u8);
                    if sigZ < 0x40000000_i32 as u64 {
                        expZ -= 1;
                        sigZ <<= 1_i32;
                    }
                    return softfloat_roundPackToF32(signZ, expZ, sigZ);
                }
            }
        }
        uiZ = ((signZ as u32) << 31_i32)
            .wrapping_add((0_i32 as u32) << 23_i32)
            .wrapping_add(0_i32 as u32) as u64;
        current_block = 1831219958539456861;
    }
    match current_block {
        3699674519062403871 => {
            if magBits == 0 {
                softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
                uiZ = 0x7fc00000_i32 as u64;
            } else {
                uiZ = ((signZ as u32) << 31_i32)
                    .wrapping_add((0xff_i32 as u32) << 23_i32)
                    .wrapping_add(0_i32 as u32) as u64;
            }
        }
        7211456467417213049 => {
            uiZ = softfloat_propagateNaNF32UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ as u32;
    uZ.f
}
