use super::*;

pub unsafe fn f128_mulAdd(mut a: float128_t, mut b: float128_t, mut c: float128_t) -> float128_t {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: u64;
    let mut uiA0: u64;
    let mut uB: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiB64: u64;
    let mut uiB0: u64;
    let mut uC: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiC64: u64;
    let mut uiC0: u64;
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    uB.f = b;
    uiB64 = uB.ui.v64;
    uiB0 = uB.ui.v0;
    uC.f = c;
    uiC64 = uC.ui.v64;
    uiC0 = uC.ui.v0;
    softfloat_mulAddF128(uiA64, uiA0, uiB64, uiB0, uiC64, uiC0, 0_i32 as u8)
}
