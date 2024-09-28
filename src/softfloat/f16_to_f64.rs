use super::*;

pub unsafe fn f16_to_f64(mut a: float16_t) -> float64_t {
    let mut current_block: u64;
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: u64;
    let mut sign: bool;
    let mut exp: i8;
    let mut frac: u64;
    let mut uiZ: u64 = 0;
    let mut normExpSig: exp8_sig16;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as u64;
    sign = uiA as u16 as i32 >> 15_i32 != 0;
    exp = ((uiA >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    frac = uiA & 0x3ff_i32 as u64;
    if exp as i32 == 0x1f_i32 {
        if frac != 0 {
            if uiA & 0x200_i32 as u64 == 0 {
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
                current_block = 94606511845669733;
            } else {
                normExpSig = softfloat_normSubnormalF16Sig(frac);
                exp = (normExpSig.exp as i32 - 1_i32) as i8;
                frac = normExpSig.sig;
                current_block = 15904375183555213903;
            }
        } else {
            current_block = 15904375183555213903;
        }
        match current_block {
            94606511845669733 => {}
            _ => {
                uiZ = ((sign as u64) << 63_i32)
                    .wrapping_add(((exp as i32 + 0x3f0_i32) as u64) << 52_i32)
                    .wrapping_add(frac << 42_i32);
            }
        }
    }
    uZ.ui = uiZ;
    uZ.f
}
