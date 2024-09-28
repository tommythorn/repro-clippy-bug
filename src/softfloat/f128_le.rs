use super::*;

pub unsafe fn f128_le(mut a: float128_t, mut b: float128_t) -> bool {
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
    let mut signA: bool;
    let mut signB: bool;
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    uB.f = b;
    uiB64 = uB.ui.v64;
    uiB0 = uB.ui.v0;
    if !uiA64 & 0x7fff000000000000_u64 == 0_i32 as u64
        && (uiA0 != 0 || uiA64 & 0xffffffffffff_u64 != 0)
        || !uiB64 & 0x7fff000000000000_u64 == 0_i32 as u64
            && (uiB0 != 0 || uiB64 & 0xffffffffffff_u64 != 0)
    {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        return false;
    }
    signA = uiA64 >> 63_i32 != 0;
    signB = uiB64 >> 63_i32 != 0;
    (if signA as i32 != signB as i32 {
        (signA as i32 != 0 || (uiA64 | uiB64) & 0x7fffffffffffffff_u64 | uiA0 | uiB0 == 0) as i32
    } else {
        (uiA64 == uiB64 && uiA0 == uiB0
            || signA as i32 ^ softfloat_lt128(uiA64, uiA0, uiB64, uiB0) as i32 != 0) as i32
    } != 0)
}
