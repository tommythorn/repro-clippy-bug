use super::*;

pub unsafe fn f32_to_f16(mut a: float32_t) -> float16_t {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut frac: u64;
    let mut uiZ: u64;
    let mut frac16: u64;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as u64;
    sign = uiA as u32 >> 31_i32 != 0;
    exp = (uiA >> 23_i32) as i64 & 0xff_i32 as i64;
    frac = uiA & 0x7fffff_i32 as u64;
    if exp == 0xff_i32 as i64 {
        if frac != 0 {
            if uiA & 0x400000_i32 as u64 == 0 {
                softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
            }
            uiZ = 0x7e00_i32 as u64;
        } else {
            uiZ = (((sign as u16 as i32) << 15_i32) + ((0x1f_i32 as u16 as i32) << 10_i32) + 0_i32)
                as u64;
        }
    } else {
        frac16 = frac >> 9_i32 | (frac & 0x1ff_i32 as u64 != 0_i32 as u64) as i32 as u64;
        if exp as u64 | frac16 == 0 {
            uiZ = (((sign as u16 as i32) << 15_i32) + ((0_i32 as u16 as i32) << 10_i32) + 0_i32)
                as u64;
        } else {
            return softfloat_roundPackToF16(
                sign,
                exp - 0x71_i32 as i64,
                frac16 | 0x4000_i32 as u64,
            );
        }
    }
    uZ.ui = uiZ as u16;
    uZ.f
}
