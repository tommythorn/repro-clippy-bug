use super::*;

pub unsafe fn softfloat_propagateNaNF16UI(mut uiA: u64, mut uiB: u64) -> u64 {
    if uiA & 0x7e00_i32 as u64 == 0x7c00_i32 as u64 && uiA & 0x1ff_i32 as u64 != 0
        || uiB & 0x7e00_i32 as u64 == 0x7c00_i32 as u64 && uiB & 0x1ff_i32 as u64 != 0
    {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as u8);
    }
    0x7e00_i32 as u64
}
