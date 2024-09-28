use super::*;

pub unsafe fn f32_sub(mut a: float32_t, mut b: float32_t) -> float32_t {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: u64;
    let mut uB: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiB: u64;
    uA.f = a;
    uiA = uA.ui as u64;
    uB.f = b;
    uiB = uB.ui as u64;
    if (uiA ^ uiB) as u32 >> 31_i32 != 0 {
        softfloat_addMagsF32(uiA, uiB)
    } else {
        softfloat_subMagsF32(uiA, uiB)
    }
}
