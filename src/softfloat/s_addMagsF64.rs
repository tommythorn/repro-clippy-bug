use super::*;

pub unsafe fn softfloat_addMagsF64(uiA: u64, uiB: u64, signZ: bool) -> float64_t {
    let mut current_block: u64;
    let mut expA: i64;
    let mut sigA: u64;
    let mut expB: i64;
    let mut sigB: u64;
    let mut expDiff: i64;
    let mut uiZ: u64 = 0;
    let mut expZ: i64 = 0;
    let mut sigZ: u64 = 0;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    expA = (uiA >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sigA = uiA & 0xfffffffffffff;
    expB = (uiB >> 52_i32) as i64 & 0x7ff_i32 as i64;
    sigB = uiB & 0xfffffffffffff;
    expDiff = expA - expB;
    if expDiff == 0 {
        if expA == 0 {
            uiZ = uiA.wrapping_add(sigB);
            current_block = 12833942176128510879;
        } else if expA == 0x7ff_i32 as i64 {
            if sigA | sigB != 0 {
                current_block = 15279871717949377988;
            } else {
                uiZ = uiA;
                current_block = 12833942176128510879;
            }
        } else {
            expZ = expA;
            sigZ = 0x20000000000000_u64.wrapping_add(sigA).wrapping_add(sigB);
            sigZ <<= 9_i32;
            current_block = 7746103178988627676;
        }
    } else {
        sigA <<= 9_i32;
        sigB <<= 9_i32;
        if expDiff < 0_i32 as i64 {
            if expB == 0x7ff_i32 as i64 {
                if sigB != 0 {
                    current_block = 15279871717949377988;
                } else {
                    uiZ = ((signZ as u64) << 63_i32)
                        .wrapping_add((0x7ff_i32 as u64) << 52_i32)
                        .wrapping_add(0_i32 as u64);
                    current_block = 12833942176128510879;
                }
            } else {
                expZ = expB;
                if expA != 0 {
                    sigA = sigA.wrapping_add(0x2000000000000000_u64);
                } else {
                    sigA <<= 1_i32;
                }
                sigA = softfloat_shiftRightJam64(sigA, -expDiff as u64);
                current_block = 9853141518545631134;
            }
        } else if expA == 0x7ff_i32 as i64 {
            if sigA != 0 {
                current_block = 15279871717949377988;
            } else {
                uiZ = uiA;
                current_block = 12833942176128510879;
            }
        } else {
            expZ = expA;
            if expB != 0 {
                sigB = sigB.wrapping_add(0x2000000000000000_u64);
            } else {
                sigB <<= 1_i32;
            }
            sigB = softfloat_shiftRightJam64(sigB, expDiff as u64);
            current_block = 9853141518545631134;
        }
        match current_block {
            12833942176128510879 => {}
            15279871717949377988 => {}
            _ => {
                sigZ = 0x2000000000000000_u64.wrapping_add(sigA).wrapping_add(sigB);
                if sigZ < 0x4000000000000000_u64 {
                    expZ -= 1;
                    sigZ <<= 1_i32;
                }
                current_block = 7746103178988627676;
            }
        }
    }
    match current_block {
        7746103178988627676 => return softfloat_roundPackToF64(signZ, expZ, sigZ),
        15279871717949377988 => {
            uiZ = softfloat_propagateNaNF64UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ;
    uZ.f
}
