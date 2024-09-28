use super::*;

pub unsafe fn f64_sub(mut a: float64_t, mut b: float64_t) -> float64_t {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: u64;
    let mut signA: bool;
    let mut uB: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiB: u64;
    let mut signB: bool;
    uA.f = a;
    uiA = uA.ui;
    signA = uiA >> 63_i32 != 0;
    uB.f = b;
    uiB = uB.ui;
    signB = uiB >> 63_i32 != 0;
    if signA as i32 == signB as i32 {
        softfloat_subMagsF64(uiA, uiB, signA)
    } else {
        softfloat_addMagsF64(uiA, uiB, signA)
    }
}
