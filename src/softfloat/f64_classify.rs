use super::*;

pub unsafe fn f64_classify(mut a: float64_t) -> u64 {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    uA.f = a;
    uiA = uA.ui;
    let mut infOrNaN: u64 =
        ((uiA >> 52_i32) as i64 & 0x7ff_i32 as i64 == 0x7ff_i32 as i64) as i32 as u64;
    let mut subnormalOrZero: u64 =
        ((uiA >> 52_i32) as i64 & 0x7ff_i32 as i64 == 0_i32 as i64) as i32 as u64;
    let mut sign: bool = uiA >> 63_i32 != 0;
    let mut fracZero: bool = uiA & 0xfffffffffffff == 0_i32 as u64;
    let mut isNaN: bool =
        !uiA & 0x7ff0000000000000_u64 == 0_i32 as u64 && uiA & 0xfffffffffffff != 0;
    let mut isSNaN: bool =
        uiA & 0x7ff8000000000000_u64 == 0x7ff0000000000000_u64 && uiA & 0x7ffffffffffff_u64 != 0;
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
