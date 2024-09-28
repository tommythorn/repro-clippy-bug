use super::*;

unsafe fn softfloat_add128(mut a64: u64, mut a0: u64, mut b64: u64, mut b0: u64) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v0 = a0.wrapping_add(b0);
    z.v64 = a64
        .wrapping_add(b64)
        .wrapping_add((z.v0 < a0) as i32 as u64);
    z
}
pub unsafe fn softfloat_roundPackToF128(
    mut sign: bool,
    mut exp: i64,
    mut sig64: u64,
    mut sig0: u64,
    mut sigExtra: u64,
) -> float128_t {
    let mut current_block: u64;
    let mut roundingMode: u8;
    let mut roundNearEven: bool;
    let mut doIncrement: bool;
    let mut isTiny: bool;
    let mut sig128Extra: uint128_extra;
    let mut uiZ64: u64 = 0;
    let mut uiZ0: u64 = 0;
    let mut sig128: uint128;
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    roundingMode = softfloat_roundingMode;
    roundNearEven = roundingMode as i32 == softfloat_round_near_even as i32;
    doIncrement = 0x8000000000000000_u64 <= sigExtra;
    if !roundNearEven && roundingMode as i32 != softfloat_round_near_maxMag as i32 {
        doIncrement = roundingMode as i32
            == (if sign as i32 != 0 {
                softfloat_round_min as i32
            } else {
                softfloat_round_max as i32
            })
            && sigExtra != 0;
    }
    if 0x7ffd_i32 as u32 <= exp as u32 {
        if exp < 0_i32 as i64 {
            isTiny = softfloat_detectTininess as i32 == softfloat_tininess_beforeRounding as i32
                || exp < -1_i32 as i64
                || !doIncrement
                || softfloat_lt128(sig64, sig0, 0x1ffffffffffff_u64, 0xffffffffffffffff_u64) as i32
                    != 0;
            sig128Extra = softfloat_shiftRightJam128Extra(sig64, sig0, sigExtra, -exp as u64);
            sig64 = sig128Extra.v.v64;
            sig0 = sig128Extra.v.v0;
            sigExtra = sig128Extra.extra;
            exp = 0_i32 as i64;
            if isTiny as i32 != 0 && sigExtra != 0 {
                softfloat_raiseFlags(softfloat_flag_underflow as i32 as u8);
            }
            doIncrement = 0x8000000000000000_u64 <= sigExtra;
            if !roundNearEven && roundingMode as i32 != softfloat_round_near_maxMag as i32 {
                doIncrement = roundingMode as i32
                    == (if sign as i32 != 0 {
                        softfloat_round_min as i32
                    } else {
                        softfloat_round_max as i32
                    })
                    && sigExtra != 0;
            }
            current_block = 18386322304582297246;
        } else if (0x7ffd_i32 as i64) < exp
            || exp == 0x7ffd_i32 as i64
                && softfloat_eq128(sig64, sig0, 0x1ffffffffffff_u64, 0xffffffffffffffff_u64) as i32
                    != 0
                && doIncrement as i32 != 0
        {
            softfloat_raiseFlags(
                (softfloat_flag_overflow as i32 | softfloat_flag_inexact as i32) as u8,
            );
            if roundNearEven as i32 != 0
                || roundingMode as i32 == softfloat_round_near_maxMag as i32
                || roundingMode as i32
                    == (if sign as i32 != 0 {
                        softfloat_round_min as i32
                    } else {
                        softfloat_round_max as i32
                    })
            {
                uiZ64 = ((sign as u64) << 63_i32)
                    .wrapping_add((0x7fff_i32 as u64) << 48_i32)
                    .wrapping_add(0_i32 as u64);
                uiZ0 = 0_i32 as u64;
            } else {
                uiZ64 = ((sign as u64) << 63_i32)
                    .wrapping_add((0x7ffe_i32 as u64) << 48_i32)
                    .wrapping_add(0xffffffffffff_u64);
                uiZ0 = 0xffffffffffffffff_u64;
            }
            current_block = 7418879765302796085;
        } else {
            current_block = 18386322304582297246;
        }
    } else {
        current_block = 18386322304582297246;
    }
    if current_block == 18386322304582297246 {
        if sigExtra != 0 {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_inexact as i32) as u8;
            if roundingMode as i32 == softfloat_round_odd as i32 {
                sig0 |= 1_i32 as u64;
                current_block = 2710716663382087210;
            } else {
                current_block = 5689316957504528238;
            }
        } else {
            current_block = 5689316957504528238;
        }
        if current_block == 5689316957504528238 {
            if doIncrement {
                sig128 = softfloat_add128(sig64, sig0, 0_i32 as u64, 1_i32 as u64);
                sig64 = sig128.v64;
                sig0 = sig128.v0
                    & !(((sigExtra & 0x7fffffffffffffff_u64 == 0) as i32 & roundNearEven as i32)
                        as u64);
            } else if sig64 | sig0 == 0 {
                exp = 0_i32 as i64;
            }
        }
        uiZ64 = ((sign as u64) << 63_i32)
            .wrapping_add((exp as u64) << 48_i32)
            .wrapping_add(sig64);
        uiZ0 = sig0;
    }
    uZ.ui.v64 = uiZ64;
    uZ.ui.v0 = uiZ0;
    uZ.f
}
