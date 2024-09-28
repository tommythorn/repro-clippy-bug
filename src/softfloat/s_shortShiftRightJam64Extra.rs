use super::*;

pub unsafe fn softfloat_shortShiftRightJam64Extra(a: u64, extra: u64, dist: u8) -> uint64_extra {
    let mut z: uint64_extra = uint64_extra { extra: 0, v: 0 };
    z.v = a >> dist;
    z.extra = a << (-(dist as i32) & 63) | (extra != 0) as i32 as u64;
    z
}
