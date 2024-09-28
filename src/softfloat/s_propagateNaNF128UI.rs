use super::*;

pub unsafe fn softfloat_propagateNaNF128UI(
    mut uiA64: u64,
    mut uiA0: u64,
    mut uiB64: u64,
    mut uiB0: u64,
) -> uint128 {
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    if uiA64 & 0x7fff800000000000_u64 == 0x7fff000000000000_u64
        && (uiA0 != 0 || uiA64 & 0x7fffffffffff_u64 != 0)
        || uiB64 & 0x7fff800000000000_u64 == 0x7fff000000000000_u64
            && (uiB0 != 0 || uiB64 & 0x7fffffffffff_u64 != 0)
    {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
    }
    uiZ.v64 = 0x7fff800000000000_u64;
    uiZ.v0 = 0_u64;
    uiZ
}
