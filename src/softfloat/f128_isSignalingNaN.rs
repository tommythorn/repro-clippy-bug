use super::*;

pub unsafe fn f128_isSignalingNaN(mut a: float128_t) -> bool {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    uA.f = a;
    uA.ui.v64 & 0x7fff800000000000_u64 == 0x7fff000000000000_u64
        && (uA.ui.v0 != 0 || uA.ui.v64 & 0x7fffffffffff_u64 != 0)
}
