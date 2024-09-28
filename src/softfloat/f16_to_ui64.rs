use super::*;

pub unsafe fn f16_to_ui64(mut a: float16_t, mut roundingMode: u8, mut exact: bool) -> u64 {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: u64;
    let mut sign: bool;
    let mut exp: i8;
    let mut frac: u64;
    let mut sig32: u64;
    let mut shiftDist: i8;
    uA.f = a;
    uiA = uA.ui as u64;
    sign = uiA as u16 as i32 >> 15_i32 != 0;
    exp = ((uiA >> 10_i32) as i8 as i32 & 0x1f_i32) as i8;
    frac = uiA & 0x3ff_i32 as u64;
    if exp as i32 == 0x1f_i32 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
        return if frac != 0 {
            0xffffffffffffffff_u64
        } else if sign as i32 != 0 {
            0_i32 as u64
        } else {
            0xffffffffffffffff_u64
        };
    }
    sig32 = frac;
    if exp != 0 {
        sig32 |= 0x400_i32 as u64;
        shiftDist = (exp as i32 - 0x19_i32) as i8;
        if 0_i32 <= shiftDist as i32 && !sign {
            return sig32 << shiftDist as i32;
        }
        shiftDist = (exp as i32 - 0xd_i32) as i8;
        if 0_i32 < shiftDist as i32 {
            sig32 <<= shiftDist as i32;
        }
    }
    softfloat_roundToUI32(sign, sig32, roundingMode, exact)
}
