use super::*;

pub unsafe fn f64_to_f128(mut a: float64_t) -> float128_t {
    let mut current_block: u64;
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut sign: bool;
    let mut exp: i64;
    let mut frac: u64;
    let mut commonNaN: commonNaN = commonNaN { _unused: 0 };
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut normExpSig: exp16_sig64;
    let mut frac128: uint128;
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
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
                current_block = 14656667696112911617;
            } else {
                normExpSig = softfloat_normSubnormalF64Sig(frac);
                exp = normExpSig.exp - 1_i32 as i64;
                frac = normExpSig.sig;
                current_block = 4808432441040389987;
            }
        } else {
            current_block = 4808432441040389987;
        }
        match current_block {
            14656667696112911617 => {}
            _ => {
                frac128 = softfloat_shortShiftLeft128(0_i32 as u64, frac, 60_i32 as u8);
                uiZ.v64 = ((sign as u64) << 63_i32)
                    .wrapping_add(((exp + 0x3c00_i32 as i64) as u64) << 48_i32)
                    .wrapping_add(frac128.v64);
                uiZ.v0 = frac128.v0;
            }
        }
    }
    uZ.ui = uiZ;
    uZ.f
}
