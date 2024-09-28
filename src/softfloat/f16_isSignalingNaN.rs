use super::*;

pub unsafe fn f16_isSignalingNaN(mut a: float16_t) -> bool {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    uA.ui as i32 & 0x7e00_i32 == 0x7c00_i32 && uA.ui as i32 & 0x1ff_i32 != 0
}
