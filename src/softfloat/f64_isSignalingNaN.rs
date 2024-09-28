use super::*;

pub unsafe fn f64_isSignalingNaN(mut a: float64_t) -> bool {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    uA.ui & 0x7ff8000000000000_u64 == 0x7ff0000000000000_u64 && uA.ui & 0x7ffffffffffff_u64 != 0
}
