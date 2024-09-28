use super::*;

pub unsafe fn softfloat_shiftRightJam128Extra(
    mut a64: u64,
    mut a0: u64,
    mut extra: u64,
    mut dist: u64,
) -> uint128_extra {
    let mut u8NegDist: u8;
    let mut z: uint128_extra = uint128_extra {
        extra: 0,
        v: uint128 { v0: 0, v64: 0 },
    };
    u8NegDist = dist.wrapping_neg() as u8;
    if dist < 64_i32 as u64 {
        z.v.v64 = a64 >> dist;
        z.v.v0 = a64 << (u8NegDist as i32 & 63_i32) | a0 >> dist;
        z.extra = a0 << (u8NegDist as i32 & 63_i32);
    } else {
        z.v.v64 = 0_i32 as u64;
        if dist == 64_i32 as u64 {
            z.v.v0 = a64;
            z.extra = a0;
        } else {
            extra |= a0;
            if dist < 128_i32 as u64 {
                z.v.v0 = a64 >> (dist & 63_i32 as u64);
                z.extra = a64 << (u8NegDist as i32 & 63_i32);
            } else {
                z.v.v0 = 0_i32 as u64;
                z.extra = if dist == 128_i32 as u64 {
                    a64
                } else {
                    (a64 != 0_i32 as u64) as i32 as u64
                };
            }
        }
    }
    z.extra |= (extra != 0_i32 as u64) as i32 as u64;
    z
}
