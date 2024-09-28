pub unsafe fn softfloat_lt128(mut a64: u64, mut a0: u64, mut b64: u64, mut b0: u64) -> bool {
    a64 < b64 || a64 == b64 && a0 < b0
}
