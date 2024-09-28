use super::*;

pub unsafe fn f32_isSignalingNaN(mut a: float32_t) -> bool {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    uA.f = a;
    uA.ui & 0x7fc00000_i32 as u32 == 0x7f800000_i32 as u32 && uA.ui & 0x3fffff_i32 as u32 != 0
}
