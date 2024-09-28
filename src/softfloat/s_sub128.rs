use super::*;

pub fn softfloat_sub128(mut a64: u64, mut a0: u64, mut b64: u64, mut b0: u64) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v0 = a0.wrapping_sub(b0);
    z.v64 = a64.wrapping_sub(b64).wrapping_sub((a0 < b0) as i32 as u64);
    z
}
