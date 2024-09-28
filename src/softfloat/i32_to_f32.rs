use super::*;

pub unsafe fn i32_to_f32(mut a: i32) -> float32_t {
    let mut sign: bool;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    let mut absA: u64;
    sign = a < 0_i32;
    if a & 0x7fffffff_i32 == 0 {
        uZ.ui = if sign as i32 != 0 {
            ((1_i32 as u32) << 31_i32)
                .wrapping_add((0x9e_i32 as u32) << 23_i32)
                .wrapping_add(0_i32 as u32)
        } else {
            0_i32 as u32
        };
        return uZ.f;
    }
    absA = if sign as i32 != 0 {
        (a as u64).wrapping_neg()
    } else {
        a as u64
    };
    softfloat_normRoundPackToF32(sign, 0x9c_i32 as i64, absA)
}
