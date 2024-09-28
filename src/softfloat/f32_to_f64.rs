use super::*;

pub unsafe fn f32_to_f64(mut a: float32_t) -> float64_t {
    let mut current_block: u64;
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut frac: u64;
    let mut uiZ: u64 = 0;
    let mut normExpSig: exp16_sig32;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
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
            uiZ = 0x7ff8000000000000_u64;
        } else {
            uiZ = ((sign as u64) << 63_i32)
                .wrapping_add((0x7ff_i32 as u64) << 52_i32)
                .wrapping_add(0_i32 as u64);
        }
    } else {
        if exp == 0 {
            if frac == 0 {
                uiZ = ((sign as u64) << 63_i32)
                    .wrapping_add((0_i32 as u64) << 52_i32)
                    .wrapping_add(0_i32 as u64);
                current_block = 16828882742342091130;
            } else {
                normExpSig = softfloat_normSubnormalF32Sig(frac);
                exp = normExpSig.exp - 1_i32 as i64;
                frac = normExpSig.sig;
                current_block = 15904375183555213903;
            }
        } else {
            current_block = 15904375183555213903;
        }
        match current_block {
            16828882742342091130 => {}
            _ => {
                uiZ = ((sign as u64) << 63_i32)
                    .wrapping_add(((exp + 0x380_i32 as i64) as u64) << 52_i32)
                    .wrapping_add(frac << 29_i32);
            }
        }
    }
    uZ.ui = uiZ;
    uZ.f
}
