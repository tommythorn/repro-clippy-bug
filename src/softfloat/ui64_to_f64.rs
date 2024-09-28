use super::*;

pub unsafe fn ui64_to_f64(a: u64) -> float64_t {
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    if a == 0 {
        uZ.ui = 0;
        return uZ.f;
    }
    if a & 0x8000000000000000 != 0 {
        softfloat_roundPackToF64(false, 0x43d, softfloat_shortShiftRightJam64(a, 1))
    } else {
        softfloat_normRoundPackToF64(false, 0x43c, a)
    }
}
