use super::*;

pub unsafe fn f128_mul(mut a: float128_t, mut b: float128_t) -> float128_t {
    let mut current_block: u64;
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: u64;
    let mut uiA0: u64;
    let mut signA: bool;
    let mut expA: i64;
    let mut sigA: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uB: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiB64: u64;
    let mut uiB0: u64;
    let mut signB: bool;
    let mut expB: i64;
    let mut sigB: uint128 = uint128 { v0: 0, v64: 0 };
    let mut signZ: bool;
    let mut magBits: u64 = 0;
    let mut normExpSig: exp32_sig128;
    let mut expZ: i64;
    let mut sig256Z: [u64; 4] = [0; 4];
    let mut sigZExtra: u64;
    let mut sigZ: uint128;
    let mut sig128Extra: uint128_extra;
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    signA = uiA64 >> 63_i32 != 0;
    expA = (uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sigA.v64 = uiA64 & 0xffffffffffff;
    sigA.v0 = uiA0;
    uB.f = b;
    uiB64 = uB.ui.v64;
    uiB0 = uB.ui.v0;
    signB = uiB64 >> 63_i32 != 0;
    expB = (uiB64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    sigB.v64 = uiB64 & 0xffffffffffff;
    sigB.v0 = uiB0;
    signZ = signA as i32 ^ signB as i32 != 0;
    if expA == 0x7fff_i32 as i64 {
        if sigA.v64 | sigA.v0 != 0 || expB == 0x7fff_i32 as i64 && sigB.v64 | sigB.v0 != 0 {
            current_block = 15587166790441369992;
        } else {
            magBits = expB as u64 | sigB.v64 | sigB.v0;
            current_block = 4178002844255707336;
        }
    } else if expB == 0x7fff_i32 as i64 {
        if sigB.v64 | sigB.v0 != 0 {
            current_block = 15587166790441369992;
        } else {
            magBits = expA as u64 | sigA.v64 | sigA.v0;
            current_block = 4178002844255707336;
        }
    } else {
        if expA == 0 {
            if sigA.v64 | sigA.v0 == 0 {
                current_block = 7853677049044618404;
            } else {
                normExpSig = softfloat_normSubnormalF128Sig(sigA.v64, sigA.v0);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
                current_block = 14763689060501151050;
            }
        } else {
            current_block = 14763689060501151050;
        }
        if current_block == 14763689060501151050 {
            if expB == 0 {
                if sigB.v64 | sigB.v0 == 0 {
                    current_block = 7853677049044618404;
                } else {
                    normExpSig = softfloat_normSubnormalF128Sig(sigB.v64, sigB.v0);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 15897653523371991391;
                }
            } else {
                current_block = 15897653523371991391;
            }
            match current_block {
                7853677049044618404 => {}
                _ => {
                    expZ = expA + expB - 0x4000_i32 as i64;
                    sigA.v64 |= 0x1000000000000;
                    sigB = softfloat_shortShiftLeft128(sigB.v64, sigB.v0, 16_i32 as u8);
                    softfloat_mul128To256M(
                        sigA.v64,
                        sigA.v0,
                        sigB.v64,
                        sigB.v0,
                        sig256Z.as_mut_ptr(),
                    );
                    sigZExtra = sig256Z[1] | (sig256Z[0] != 0_i32 as u64) as i32 as u64;
                    sigZ = softfloat_add128(sig256Z[3], sig256Z[2], sigA.v64, sigA.v0);
                    if 0x2000000000000 <= sigZ.v64 {
                        expZ += 1;
                        sig128Extra = softfloat_shortShiftRightJam128Extra(
                            sigZ.v64,
                            sigZ.v0,
                            sigZExtra,
                            1_i32 as u8,
                        );
                        sigZ = sig128Extra.v;
                        sigZExtra = sig128Extra.extra;
                    }
                    return softfloat_roundPackToF128(signZ, expZ, sigZ.v64, sigZ.v0, sigZExtra);
                }
            }
        }
        uiZ.v64 = ((signZ as u64) << 63_i32)
            .wrapping_add((0_i32 as u64) << 48_i32)
            .wrapping_add(0_i32 as u64);
        current_block = 12065082218314902246;
    }
    match current_block {
        4178002844255707336 => {
            if magBits == 0 {
                softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
                uiZ.v64 = 0x7fff800000000000;
                uiZ.v0 = 0;
                current_block = 9304412649211998351;
            } else {
                uiZ.v64 = ((signZ as u64) << 63_i32)
                    .wrapping_add((0x7fff_i32 as u64) << 48_i32)
                    .wrapping_add(0_i32 as u64);
                current_block = 12065082218314902246;
            }
        }
        15587166790441369992 => {
            uiZ = softfloat_propagateNaNF128UI(uiA64, uiA0, uiB64, uiB0);
            current_block = 9304412649211998351;
        }
        _ => {}
    }
    if current_block == 12065082218314902246 {
        uiZ.v0 = 0_i32 as u64;
    }
    uZ.ui = uiZ;
    uZ.f
}
