use super::*;

pub unsafe fn f16_to_f128(mut a: float16_t) -> float128_t {
    let mut current_block: u64;
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: u64;
    let mut sign: bool;
    let mut exp: i8;
    let mut frac: u64;
    let mut commonNaN: commonNaN = commonNaN { _unused: 0 };
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut normExpSig: exp8_sig16;
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
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
                current_block = 2054165464947643463;
            } else {
                normExpSig = softfloat_normSubnormalF16Sig(frac);
                exp = (normExpSig.exp as i32 - 1_i32) as i8;
                frac = normExpSig.sig;
                current_block = 2838571290723028321;
            }
        } else {
            current_block = 2838571290723028321;
        }
        match current_block {
            2054165464947643463 => {}
            _ => {
                uiZ.v64 = ((sign as u64) << 63_i32)
                    .wrapping_add(((exp as i32 + 0x3ff0_i32) as u64) << 48_i32)
                    .wrapping_add(frac << 38_i32);
                uiZ.v0 = 0_i32 as u64;
            }
        }
    }
    uZ.ui = uiZ;
    uZ.f
}
