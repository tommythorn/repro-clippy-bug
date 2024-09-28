use super::*;

pub fn softfloat_shiftRightJam64Extra(a: u64, extra: u64, dist: u64) -> uint64_extra {
    let mut z: uint64_extra = uint64_extra { extra: 0, v: 0 };
    if dist < 64 {
        z.v = a >> dist;
        z.extra = a << (dist.wrapping_neg() & 63);
    } else {
        z.v = 0;
        z.extra = if dist == 64 { a } else { (a != 0) as u64 };
    }
    z.extra |= (extra != 0) as u64;
    z
}
