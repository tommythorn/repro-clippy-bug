use super::*;

pub unsafe fn f128_sub(mut a: float128_t, mut b: float128_t) -> float128_t {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: u64;
    let mut uiA0: u64;
    let mut signA: bool;
    let mut uB: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiB64: u64;
    let mut uiB0: u64;
    let mut signB: bool;
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    signA = uiA64 >> 63_i32 != 0;
    uB.f = b;
    uiB64 = uB.ui.v64;
    uiB0 = uB.ui.v0;
    signB = uiB64 >> 63_i32 != 0;
    if signA as i32 == signB as i32 {
        softfloat_subMagsF128(uiA64, uiA0, uiB64, uiB0, signA)
    } else {
        softfloat_addMagsF128(uiA64, uiA0, uiB64, uiB0, signA)
    }
}
