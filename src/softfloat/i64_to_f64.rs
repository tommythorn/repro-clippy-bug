use super::*;

pub unsafe fn i64_to_f64(mut a: i64) -> float64_t {
    let mut sign: bool;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    let mut absA: u64;
    sign = a < 0_i32 as i64;
    if a as u64 & 0x7fffffffffffffff_u64 == 0 {
        uZ.ui = if sign as i32 != 0 {
            ((1_i32 as u64) << 63_i32)
                .wrapping_add((0x43e_i32 as u64) << 52_i32)
                .wrapping_add(0_i32 as u64)
        } else {
            0_i32 as u64
        };
        return uZ.f;
    }
    absA = if sign as i32 != 0 {
        (a as u64).wrapping_neg()
    } else {
        a as u64
    };
    softfloat_normRoundPackToF64(sign, 0x43c_i32 as i64, absA)
}
