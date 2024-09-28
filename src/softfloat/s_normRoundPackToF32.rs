use super::*;

pub unsafe fn softfloat_normRoundPackToF32(sign: bool, mut exp: i64, sig: u64) -> float32_t {
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    let shiftDist = (softfloat_countLeadingZeros32(sig as u32) as i32 - 1) as i8;
    exp -= shiftDist as i64;
    if 7_i32 <= shiftDist as i32 && (exp as u32) < 0xfd {
        uZ.ui = (((sign as u32) << 31).wrapping_add(((if sig != 0 { exp } else { 0 }) as u32) << 23)
            as u64)
            .wrapping_add(sig << (shiftDist as i32 - 7)) as u32;
        uZ.f
    } else {
        softfloat_roundPackToF32(sign, exp, sig << shiftDist as i32)
    }
}
