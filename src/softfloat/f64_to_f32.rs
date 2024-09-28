use super::*;

pub unsafe fn f64_to_f32(a: float64_t) -> float32_t {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut frac: u64;
    let mut uiZ: u64;
    let mut frac32: u64;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
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
            uiZ = 0x7fc00000_i32 as u64;
        } else {
            uiZ = ((sign as u32) << 31_i32)
                .wrapping_add((0xff_i32 as u32) << 23_i32)
                .wrapping_add(0_i32 as u32) as u64;
        }
    } else {
        frac32 = softfloat_shortShiftRightJam64(frac, 22_i32 as u8);
        if exp as u64 | frac32 == 0 {
            uiZ = ((sign as u32) << 31_i32)
                .wrapping_add((0_i32 as u32) << 23_i32)
                .wrapping_add(0_i32 as u32) as u64;
        } else {
            return softfloat_roundPackToF32(
                sign,
                exp - 0x381_i32 as i64,
                frac32 | 0x40000000_i32 as u64,
            );
        }
    }
    uZ.ui = uiZ as u32;
    uZ.f
}
