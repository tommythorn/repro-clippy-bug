use super::*;

pub unsafe fn f16_add(mut a: float16_t, mut b: float16_t) -> float16_t {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: u64;
    let mut uB: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiB: u64;
    uA.f = a;
    uiA = uA.ui as u64;
    uB.f = b;
    uiB = uB.ui as u64;
    if (uiA ^ uiB) as u16 as i32 >> 15_i32 != 0 {
        softfloat_subMagsF16(uiA, uiB)
    } else {
        softfloat_addMagsF16(uiA, uiB)
    }
}
