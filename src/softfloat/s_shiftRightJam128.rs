use super::*;

pub unsafe fn softfloat_shiftRightJam128(mut a64: u64, mut a0: u64, mut dist: u64) -> uint128 {
    let mut u8NegDist: u8;
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    if dist < 64_i32 as u64 {
        u8NegDist = dist.wrapping_neg() as u8;
        z.v64 = a64 >> dist;
        z.v0 = a64 << (u8NegDist as i32 & 63_i32)
            | a0 >> dist
            | (a0 << (u8NegDist as i32 & 63_i32) != 0_i32 as u64) as i32 as u64;
    } else {
        z.v64 = 0_i32 as u64;
        z.v0 = if dist < 127_i32 as u64 {
            a64 >> (dist & 63_i32 as u64)
                | (a64 & ((1_i32 as u64) << (dist & 63_i32 as u64)).wrapping_sub(1_i32 as u64) | a0
                    != 0_i32 as u64) as i32 as u64
        } else {
            (a64 | a0 != 0_i32 as u64) as i32 as u64
        };
    }
    z
}
