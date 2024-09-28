use super::*;

pub unsafe fn ui64_to_f16(a: u64) -> float16_t {
    let mut shiftDist: i8;
    let mut u: ui16_f16 = ui16_f16 { ui: 0 };
    let mut sig: u64;
    shiftDist = (softfloat_countLeadingZeros64(a) as i32 - 53_i32) as i8;
    if 0_i32 <= shiftDist as i32 {
        u.ui = (if a != 0 {
            ((((0_i32 as u16 as i32) << 15_i32)
                + (((0x18_i32 - shiftDist as i32) as u16 as i32) << 10_i32)) as u64)
                .wrapping_add(a << shiftDist as i32)
        } else {
            0_i32 as u64
        }) as u16;
        u.f
    } else {
        shiftDist = (shiftDist as i32 + 4_i32) as i8;
        sig = if (shiftDist as i32) < 0_i32 {
            softfloat_shortShiftRightJam64(a, -(shiftDist as i32) as u8)
        } else {
            a << shiftDist as i32
        };
        softfloat_roundPackToF16(false, (0x1c_i32 - shiftDist as i32) as i64, sig)
    }
}
