use super::*;

pub unsafe fn softfloat_addMagsF32(mut uiA: u64, mut uiB: u64) -> float32_t {
    let mut current_block: u64;
    let mut expA: i64;
    let mut sigA: u64;
    let mut expB: i64;
    let mut sigB: u64;
    let mut expDiff: i64;
    let mut uiZ: u64 = 0;
    let mut signZ: bool = false;
    let mut expZ: i64 = 0;
    let mut sigZ: u64 = 0;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    expA = (uiA >> 23_i32) as i64 & 0xff_i32 as i64;
    sigA = uiA & 0x7fffff_i32 as u64;
    expB = (uiB >> 23_i32) as i64 & 0xff_i32 as i64;
    sigB = uiB & 0x7fffff_i32 as u64;
    expDiff = expA - expB;
    if expDiff == 0 {
        if expA == 0 {
            uiZ = uiA.wrapping_add(sigB);
            current_block = 13829036845730652584;
        } else if expA == 0xff_i32 as i64 {
            if sigA | sigB != 0 {
                current_block = 11208688405839173004;
            } else {
                uiZ = uiA;
                current_block = 13829036845730652584;
            }
        } else {
            signZ = uiA as u32 >> 31_i32 != 0;
            expZ = expA;
            sigZ = (0x1000000_i32 as u64).wrapping_add(sigA).wrapping_add(sigB);
            if sigZ & 1_i32 as u64 == 0 && expZ < 0xfe_i32 as i64 {
                uiZ = (((signZ as u32) << 31_i32).wrapping_add((expZ as u32) << 23_i32) as u64)
                    .wrapping_add(sigZ >> 1_i32);
                current_block = 13829036845730652584;
            } else {
                sigZ <<= 6_i32;
                current_block = 7828949454673616476;
            }
        }
    } else {
        signZ = uiA as u32 >> 31_i32 != 0;
        sigA <<= 6_i32;
        sigB <<= 6_i32;
        if expDiff < 0_i32 as i64 {
            if expB == 0xff_i32 as i64 {
                if sigB != 0 {
                    current_block = 11208688405839173004;
                } else {
                    uiZ = ((signZ as u32) << 31_i32)
                        .wrapping_add((0xff_i32 as u32) << 23_i32)
                        .wrapping_add(0_i32 as u32) as u64;
                    current_block = 13829036845730652584;
                }
            } else {
                expZ = expB;
                sigA = sigA.wrapping_add(if expA != 0 {
                    0x20000000_i32 as u64
                } else {
                    sigA
                });
                sigA = softfloat_shiftRightJam32(sigA as u32, -expDiff as u64) as u64;
                current_block = 2569451025026770673;
            }
        } else if expA == 0xff_i32 as i64 {
            if sigA != 0 {
                current_block = 11208688405839173004;
            } else {
                uiZ = uiA;
                current_block = 13829036845730652584;
            }
        } else {
            expZ = expA;
            sigB = sigB.wrapping_add(if expB != 0 {
                0x20000000_i32 as u64
            } else {
                sigB
            });
            sigB = softfloat_shiftRightJam32(sigB as u32, expDiff as u64) as u64;
            current_block = 2569451025026770673;
        }
        match current_block {
            13829036845730652584 => {}
            11208688405839173004 => {}
            _ => {
                sigZ = (0x20000000_i32 as u64)
                    .wrapping_add(sigA)
                    .wrapping_add(sigB);
                if sigZ < 0x40000000_i32 as u64 {
                    expZ -= 1;
                    sigZ <<= 1_i32;
                }
                current_block = 7828949454673616476;
            }
        }
    }
    match current_block {
        7828949454673616476 => return softfloat_roundPackToF32(signZ, expZ, sigZ),
        11208688405839173004 => {
            uiZ = softfloat_propagateNaNF32UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ as u32;
    uZ.f
}
