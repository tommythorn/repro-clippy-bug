use super::*;

pub unsafe fn ui64_to_f32(a: u64) -> float32_t {
    let mut shiftDist: i8;
    let mut u: ui32_f32 = ui32_f32 { ui: 0 };
    let mut sig: u64;
    shiftDist = (softfloat_countLeadingZeros64(a) as i32 - 40_i32) as i8;
    if 0_i32 <= shiftDist as i32 {
        u.ui = (if a != 0 {
            (((0_i32 as u32) << 31_i32)
                .wrapping_add(((0x95_i32 - shiftDist as i32) as u32) << 23_i32) as u64)
                .wrapping_add(a << shiftDist as i32)
        } else {
            0_i32 as u64
        }) as u32;
        u.f
    } else {
        shiftDist = (shiftDist as i32 + 7_i32) as i8;
        sig = if (shiftDist as i32) < 0_i32 {
            softfloat_shortShiftRightJam64(a, -(shiftDist as i32) as u8)
        } else {
            a << shiftDist as i32
        };
        softfloat_roundPackToF32(false, (0x9c_i32 - shiftDist as i32) as i64, sig)
    }
}
