use super::*;

pub unsafe fn softfloat_normRoundPackToF16(
    mut sign: bool,
    mut exp: i64,
    mut sig: u64,
) -> float16_t {
    let mut shiftDist: i8;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    shiftDist = (softfloat_countLeadingZeros16(sig as u16) as i32 - 1_i32) as i8;
    exp -= shiftDist as i64;
    if 4_i32 <= shiftDist as i32 && (exp as u32) < 0x1d_i32 as u32 {
        uZ.ui = ((((sign as u16 as i32) << 15_i32)
            + (((if sig != 0 { exp } else { 0_i32 as i64 }) as u16 as i32) << 10_i32))
            as u64)
            .wrapping_add(sig << (shiftDist as i32 - 4_i32)) as u16;
        uZ.f
    } else {
        softfloat_roundPackToF16(sign, exp, sig << shiftDist as i32)
    }
}
