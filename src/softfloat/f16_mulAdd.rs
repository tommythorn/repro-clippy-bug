use super::*;

pub unsafe fn f16_mulAdd(mut a: float16_t, mut b: float16_t, mut c: float16_t) -> float16_t {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: u64;
    let mut uB: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiB: u64;
    let mut uC: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiC: u64;
    uA.f = a;
    uiA = uA.ui as u64;
    uB.f = b;
    uiB = uB.ui as u64;
    uC.f = c;
    uiC = uC.ui as u64;
    softfloat_mulAddF16(uiA, uiB, uiC, 0_i32 as u8)
}
