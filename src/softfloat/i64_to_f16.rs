use super::*;

pub unsafe fn i64_to_f16(a: i64) -> float16_t {
    let mut sign: bool;
    let mut absA: u64;
    let mut shiftDist: i8;
    let mut u: ui16_f16 = ui16_f16 { ui: 0 };
    let mut sig: u64;
    sign = a < 0_i32 as i64;
    absA = if sign as i32 != 0 {
        (a as u64).wrapping_neg()
    } else {
        a as u64
    };
    shiftDist = (softfloat_countLeadingZeros64(absA) as i32 - 53_i32) as i8;
    if 0_i32 <= shiftDist as i32 {
        u.ui = (if a != 0 {
            ((((sign as u16 as i32) << 15_i32)
                + (((0x18_i32 - shiftDist as i32) as u16 as i32) << 10_i32)) as u64)
                .wrapping_add(absA << shiftDist as i32)
        } else {
            0_i32 as u64
        }) as u16;
        u.f
    } else {
        shiftDist = (shiftDist as i32 + 4_i32) as i8;
        sig = if (shiftDist as i32) < 0_i32 {
            softfloat_shortShiftRightJam64(absA, -(shiftDist as i32) as u8)
        } else {
            absA << shiftDist as i32
        };
        softfloat_roundPackToF16(sign, (0x1c_i32 - shiftDist as i32) as i64, sig)
    }
}
