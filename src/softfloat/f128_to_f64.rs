use super::*;

pub unsafe fn f128_to_f64(mut a: float128_t) -> float64_t {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: u64;
    let mut uiA0: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut frac64: u64;
    let mut frac0: u64;
    let mut uiZ: u64;
    let mut frac128: uint128;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    sign = uiA64 >> 63_i32 != 0;
    exp = (uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    frac64 = uiA64 & 0xffffffffffff_u64;
    frac0 = uiA0;
    if exp == 0x7fff_i32 as i64 {
        if frac64 | frac0 != 0 {
            if uiA64 & 0x800000000000_u64 == 0 {
                softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            }
            uiZ = 0x7ff8000000000000_u64;
        } else {
            uiZ = ((sign as u64) << 63_i32)
                .wrapping_add((0x7ff_i32 as u64) << 52_i32)
                .wrapping_add(0_i32 as u64);
        }
    } else {
        frac128 = softfloat_shortShiftLeft128(frac64, frac0, 14_i32 as u8);
        frac64 = frac128.v64 | (frac128.v0 != 0_i32 as u64) as i32 as u64;
        if exp as u64 | frac64 == 0 {
            uiZ = ((sign as u64) << 63_i32)
                .wrapping_add((0_i32 as u64) << 52_i32)
                .wrapping_add(0_i32 as u64);
        } else {
            exp -= 0x3c01_i32 as i64;
            if (::core::mem::size_of::<i64>() as u64) < ::core::mem::size_of::<i64>() as u64
                && exp < -0x1000_i32 as i64
            {
                exp = -0x1000_i32 as i64;
            }
            return softfloat_roundPackToF64(sign, exp, frac64 | 0x4000000000000000_u64);
        }
    }
    uZ.ui = uiZ;
    uZ.f
}
