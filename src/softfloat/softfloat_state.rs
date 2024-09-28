use super::*;

// XXX this could become a Softfloat struct and all the functions here member functions
pub static mut softfloat_roundingMode: u8 = softfloat_round_near_even as i32 as u8;
pub static mut softfloat_detectTininess: u8 = softfloat_tininess_afterRounding as i32 as u8;
pub static mut softfloat_exceptionFlags: u8 = 0_i32 as u8;
