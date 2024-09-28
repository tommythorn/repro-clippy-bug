use super::*;

pub unsafe fn ui32_to_f32(mut a: u32) -> float32_t {
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    if a == 0 {
        uZ.ui = 0_i32 as u32;
        return uZ.f;
    }
    if a & 0x80000000_u32 != 0 {
        softfloat_roundPackToF32(
            false,
            0x9d_i32 as i64,
            (a >> 1_i32 | a & 1_i32 as u32) as u64,
        )
    } else {
        softfloat_normRoundPackToF32(false, 0x9c_i32 as i64, a as u64)
    }
}
