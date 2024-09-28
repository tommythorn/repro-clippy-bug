use super::*;

pub unsafe fn f64_mulAdd(mut a: float64_t, mut b: float64_t, mut c: float64_t) -> float64_t {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut uB: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiB: u64;
    let mut uC: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiC: u64;
    uA.f = a;
    uiA = uA.ui;
    uB.f = b;
    uiB = uB.ui;
    uC.f = c;
    uiC = uC.ui;
    softfloat_mulAddF64(uiA, uiB, uiC, 0_i32 as u8)
}
