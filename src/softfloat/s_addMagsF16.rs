use super::*;

pub unsafe fn softfloat_addMagsF16(mut uiA: u64, mut uiB: u64) -> float16_t {
    let mut current_block: u64;
    let mut expA: i8;
    let mut sigA: u64;
    let mut expB: i8;
    let mut sigB: u64;
    let mut expDiff: i8;
    let mut uiZ: u64 = 0;
    let mut signZ: bool = false;
    let mut expZ: i8 = 0;
    let mut sigZ: u64 = 0;
    let mut sigX: u64 = 0;
    let mut sigY: u64 = 0;
    let mut shiftDist: i8 = 0;
    let mut sig32Z: u64;
    let mut roundingMode: i8;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    expA = ((uiA >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    sigA = uiA & 0x3ff_i32 as u64;
    expB = ((uiB >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    sigB = uiB & 0x3ff_i32 as u64;
    expDiff = (expA as i32 - expB as i32) as i8;
    if expDiff == 0 {
        if expA == 0 {
            uiZ = uiA.wrapping_add(sigB);
            current_block = 16487309867376737528;
        } else if expA as i32 == 0x1f_i32 {
            if sigA | sigB != 0 {
                current_block = 4012118441784044925;
            } else {
                uiZ = uiA;
                current_block = 16487309867376737528;
            }
        } else {
            signZ = uiA as u16 as i32 >> 15_i32 != 0;
            expZ = expA;
            sigZ = (0x800_i32 as u64).wrapping_add(sigA).wrapping_add(sigB);
            if sigZ & 1_i32 as u64 == 0 && (expZ as i32) < 0x1e_i32 {
                sigZ >>= 1_i32;
                current_block = 7364418751118325979;
            } else {
                sigZ <<= 3_i32;
                current_block = 11763295167351361500;
            }
        }
    } else {
        signZ = uiA as u16 as i32 >> 15_i32 != 0;
        if (expDiff as i32) < 0_i32 {
            if expB as i32 == 0x1f_i32 {
                if sigB != 0 {
                    current_block = 4012118441784044925;
                } else {
                    uiZ = (((signZ as u16 as i32) << 15_i32)
                        + ((0x1f_i32 as u16 as i32) << 10_i32)
                        + 0_i32) as u64;
                    current_block = 16487309867376737528;
                }
            } else if expDiff as i32 <= -13_i32 {
                uiZ = ((((signZ as u16 as i32) << 15_i32) + ((expB as u16 as i32) << 10_i32))
                    as u64)
                    .wrapping_add(sigB);
                if expA as u64 | sigA != 0 {
                    current_block = 13073962406288452309;
                } else {
                    current_block = 16487309867376737528;
                }
            } else {
                expZ = expB;
                sigX = sigB | 0x400_i32 as u64;
                sigY = sigA.wrapping_add(if expA as i32 != 0 {
                    0x400_i32 as u64
                } else {
                    sigA
                });
                shiftDist = (19_i32 + expDiff as i32) as i8;
                current_block = 5529461102203738653;
            }
        } else {
            uiZ = uiA;
            if expA as i32 == 0x1f_i32 {
                if sigA != 0 {
                    current_block = 4012118441784044925;
                } else {
                    current_block = 16487309867376737528;
                }
            } else if 13_i32 <= expDiff as i32 {
                if expB as u64 | sigB != 0 {
                    current_block = 13073962406288452309;
                } else {
                    current_block = 16487309867376737528;
                }
            } else {
                expZ = expA;
                sigX = sigA | 0x400_i32 as u64;
                sigY = sigB.wrapping_add(if expB as i32 != 0 {
                    0x400_i32 as u64
                } else {
                    sigB
                });
                shiftDist = (19_i32 - expDiff as i32) as i8;
                current_block = 5529461102203738653;
            }
        }
        match current_block {
            16487309867376737528 => {}
            4012118441784044925 => {}
            _ => match current_block {
                13073962406288452309 => {
                    roundingMode = softfloat_roundingMode as i8;
                    if roundingMode as i32 != softfloat_round_near_even as i32 {
                        if roundingMode as i32
                            == (if uiZ as u16 as i32 >> 15_i32 != 0 {
                                softfloat_round_min as i32
                            } else {
                                softfloat_round_max as i32
                            })
                        {
                            uiZ = uiZ.wrapping_add(1);
                            if (uiZ << 1_i32) as u16 as i32 == 0xf800_i32 {
                                softfloat_raiseFlags(
                                    (softfloat_flag_overflow as i32 | softfloat_flag_inexact as i32)
                                        as u8,
                                );
                            }
                        } else if roundingMode as i32 == softfloat_round_odd as i32 {
                            uiZ |= 1_i32 as u64;
                        }
                    }
                    softfloat_exceptionFlags =
                        (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
                    current_block = 16487309867376737528;
                }
                _ => {
                    sig32Z = (sigX << 19_i32).wrapping_add(sigY << shiftDist as i32);
                    if sig32Z < 0x40000000_i32 as u64 {
                        expZ -= 1;
                        sig32Z <<= 1_i32;
                    }
                    sigZ = sig32Z >> 16_i32;
                    if sig32Z & 0xffff_i32 as u64 != 0 {
                        sigZ |= 1_i32 as u64;
                        current_block = 11763295167351361500;
                    } else if sigZ & 0xf_i32 as u64 == 0 && (expZ as i32) < 0x1e_i32 {
                        sigZ >>= 4_i32;
                        current_block = 7364418751118325979;
                    } else {
                        current_block = 11763295167351361500;
                    }
                }
            },
        }
    }
    match current_block {
        7364418751118325979 => {
            uiZ = ((((signZ as u16 as i32) << 15_i32) + ((expZ as u16 as i32) << 10_i32)) as u64)
                .wrapping_add(sigZ);
        }
        11763295167351361500 => {
            return softfloat_roundPackToF16(signZ, expZ as i64, sigZ);
        }
        4012118441784044925 => {
            uiZ = softfloat_propagateNaNF16UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ as u16;
    uZ.f
}
