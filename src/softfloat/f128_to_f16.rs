use super::*;

pub unsafe fn f128_to_f16(a: float128_t) -> float16_t {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: u64;
    let mut uiA0: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut frac64: u64;
    let mut uiZ: u64;
    let mut frac16: u64;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    sign = uiA64 >> 63_i32 != 0;
    exp = (uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64;
    frac64 = uiA64 & 0xffffffffffff_u64 | (uiA0 != 0_i32 as u64) as i32 as u64;
    if exp == 0x7fff_i32 as i64 {
        if frac64 != 0 {
            if uiA64 & 0x800000000000_u64 == 0 {
                softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            }
            uiZ = 0x7e00_i32 as u64;
        } else {
            uiZ = (((sign as u16 as i32) << 15_i32) + ((0x1f_i32 as u16 as i32) << 10_i32) + 0_i32)
                as u64;
        }
    } else {
        frac16 = softfloat_shortShiftRightJam64(frac64, 34_i32 as u8);
        if exp as u64 | frac16 == 0 {
            uiZ = (((sign as u16 as i32) << 15_i32) + ((0_i32 as u16 as i32) << 10_i32) + 0_i32)
                as u64;
        } else {
            exp -= 0x3ff1_i32 as i64;
            if (::core::mem::size_of::<i64>() as u64) < ::core::mem::size_of::<i64>() as u64
                && exp < -0x40_i32 as i64
            {
                exp = -0x40_i32 as i64;
            }
            return softfloat_roundPackToF16(sign, exp, frac16 | 0x4000_i32 as u64);
        }
    }
    uZ.ui = uiZ as u16;
    uZ.f
}
