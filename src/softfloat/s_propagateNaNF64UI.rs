use super::*;

pub unsafe fn softfloat_propagateNaNF64UI(mut uiA: u64, mut uiB: u64) -> u64 {
    if uiA & 0x7ff8000000000000_u64 == 0x7ff0000000000000_u64 && uiA & 0x7ffffffffffff_u64 != 0
        || uiB & 0x7ff8000000000000_u64 == 0x7ff0000000000000_u64 && uiB & 0x7ffffffffffff_u64 != 0
    {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
    }
    0x7ff8000000000000_u64
}
