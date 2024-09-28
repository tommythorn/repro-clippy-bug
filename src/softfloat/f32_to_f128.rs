use super::*;

pub unsafe fn f32_to_f128(mut a: float32_t) -> float128_t {
    let mut current_block: u64;
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut frac: u64;
    let mut commonNaN: commonNaN = commonNaN { _unused: 0 };
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut normExpSig: exp16_sig32;
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
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
            uiZ = softfloat_commonNaNToF128UI(&mut commonNaN);
        } else {
            uiZ.v64 = ((sign as u64) << 63_i32)
                .wrapping_add((0x7fff_i32 as u64) << 48_i32)
                .wrapping_add(0_i32 as u64);
            uiZ.v0 = 0_i32 as u64;
        }
    } else {
        if exp == 0 {
            if frac == 0 {
                uiZ.v64 = ((sign as u64) << 63_i32)
                    .wrapping_add((0_i32 as u64) << 48_i32)
                    .wrapping_add(0_i32 as u64);
                uiZ.v0 = 0_i32 as u64;
                current_block = 7472828778719368086;
            } else {
                normExpSig = softfloat_normSubnormalF32Sig(frac);
                exp = normExpSig.exp - 1_i32 as i64;
                frac = normExpSig.sig;
                current_block = 2838571290723028321;
            }
        } else {
            current_block = 2838571290723028321;
        }
        match current_block {
            7472828778719368086 => {}
            _ => {
                uiZ.v64 = ((sign as u64) << 63_i32)
                    .wrapping_add(((exp + 0x3f80_i32 as i64) as u64) << 48_i32)
                    .wrapping_add(frac << 25_i32);
                uiZ.v0 = 0_i32 as u64;
            }
        }
    }
    uZ.ui = uiZ;
    uZ.f
}
