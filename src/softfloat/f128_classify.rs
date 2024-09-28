use super::*;

pub unsafe fn f128_classify(mut a: float128_t) -> u64 {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: u64;
    let mut uiA0: u64;
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    let mut infOrNaN: u64 =
        ((uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64 == 0x7fff_i32 as i64) as i32 as u64;
    let mut subnormalOrZero: u64 =
        ((uiA64 >> 48_i32) as i64 & 0x7fff_i32 as i64 == 0_i32 as i64) as i32 as u64;
    let mut sign: bool = uiA64 >> 63_i32 != 0;
    let mut fracZero: bool = uiA64 & 0xffffffffffff_u64 == 0_i32 as u64 && uiA0 == 0_i32 as u64;
    let mut isNaN: bool = !uiA64 & 0x7fff000000000000_u64 == 0_i32 as u64
        && (uiA0 != 0 || uiA64 & 0xffffffffffff_u64 != 0);
    let mut isSNaN: bool = uiA64 & 0x7fff800000000000_u64 == 0x7fff000000000000_u64
        && (uiA0 != 0 || uiA64 & 0x7fffffffffff_u64 != 0);
    (((sign as i32 != 0 && infOrNaN != 0 && fracZero as i32 != 0) as i32) << 0_i32
        | ((sign as i32 != 0 && infOrNaN == 0 && subnormalOrZero == 0) as i32) << 1_i32
        | ((sign as i32 != 0 && subnormalOrZero != 0 && !fracZero) as i32) << 2_i32
        | ((sign as i32 != 0 && subnormalOrZero != 0 && fracZero as i32 != 0) as i32) << 3_i32
        | ((!sign && infOrNaN != 0 && fracZero as i32 != 0) as i32) << 7_i32
        | ((!sign && infOrNaN == 0 && subnormalOrZero == 0) as i32) << 6_i32
        | ((!sign && subnormalOrZero != 0 && !fracZero) as i32) << 5_i32
        | ((!sign && subnormalOrZero != 0 && fracZero as i32 != 0) as i32) << 4_i32
        | ((isNaN as i32 != 0 && isSNaN as i32 != 0) as i32) << 8_i32
        | ((isNaN as i32 != 0 && !isSNaN) as i32) << 9_i32) as u64
}
