use super::*;

pub unsafe fn softfloat_raiseFlags(mut flags: u8) {
    softfloat_exceptionFlags = (softfloat_exceptionFlags as i32 | flags as i32) as u8;
}
