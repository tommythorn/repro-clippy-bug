use super::*;

pub unsafe fn f64_to_f16(a: float64_t) -> float16_t {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut frac: u64;
    let mut uiZ: u64;
    let mut frac16: u64;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    uiA = uA.ui;
    sign = uiA >> 63_i32 != 0;
    exp = (uiA >> 52_i32) as i64 & 0x7ff_i32 as i64;
    frac = uiA & 0xfffffffffffff;
    if exp == 0x7ff_i32 as i64 {
        if frac != 0 {
            if uiA & 0x8000000000000_u64 == 0 {
                softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            }
            uiZ = 0x7e00_i32 as u64;
        } else {
            uiZ = (((sign as u16 as i32) << 15_i32) + ((0x1f_i32 as u16 as i32) << 10_i32) + 0_i32)
                as u64;
        }
    } else {
        frac16 = softfloat_shortShiftRightJam64(frac, 38_i32 as u8);
        if exp as u64 | frac16 == 0 {
            uiZ = (((sign as u16 as i32) << 15_i32) + ((0_i32 as u16 as i32) << 10_i32) + 0_i32)
                as u64;
        } else {
            return softfloat_roundPackToF16(
                sign,
                exp - 0x3f1_i32 as i64,
                frac16 | 0x4000_i32 as u64,
            );
        }
    }
    uZ.ui = uiZ as u16;
    uZ.f
}
