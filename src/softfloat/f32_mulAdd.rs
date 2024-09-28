use super::*;

pub unsafe fn f32_mulAdd(mut a: float32_t, mut b: float32_t, mut c: float32_t) -> float32_t {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: u64;
    let mut uB: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiB: u64;
    let mut uC: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiC: u64;
    uA.f = a;
    uiA = uA.ui as u64;
    uB.f = b;
    uiB = uB.ui as u64;
    uC.f = c;
    uiC = uC.ui as u64;
    softfloat_mulAddF32(uiA, uiB, uiC, 0_i32 as u8)
}
