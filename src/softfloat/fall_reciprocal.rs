use super::*;

unsafe fn extract64(mut val: u64, mut pos: i32, mut len: i32) -> u64 {
    if pos >= 0_i32 && len > 0_i32 && len <= 64_i32 - pos {
    } else {
        panic!("pos >= 0 && len > 0 && len <= 64 - pos");
    }
    if pos >= 0_i32 && len > 0_i32 && len <= 64_i32 - pos {
    } else {
        panic!("pos >= 0 && len > 0 && len <= 64 - pos",);
    };
    val >> pos & !0_u64 >> (64_i32 - len)
}

unsafe fn make_mask64(mut pos: i32, mut len: i32) -> u64 {
    if pos >= 0_i32 && len > 0_i32 && pos < 64_i32 && len <= 64_i32 {
    } else {
        panic!("pos >= 0 && len > 0 && pos < 64 && len <= 64",);
    }
    if pos >= 0_i32 && len > 0_i32 && pos < 64_i32 && len <= 64_i32 {
    } else {
        panic!("pos >= 0 && len > 0 && pos < 64 && len <= 64");
    };
    18446744073709551615_u64 >> (64_i32 - len) << pos
}

unsafe fn rsqrte7(mut val: u64, mut e: i32, mut s: i32, mut sub: bool) -> u64 {
    let mut exp: u64 = extract64(val, s, e);
    let mut sig: u64 = extract64(val, 0_i32, s);
    let mut sign: u64 = extract64(val, s + e, 1_i32);
    let p: i32 = 7_i32;
    static mut table: [u8; 128] = [
        52, 51, 50, 48, 47, 46, 44, 43, 42, 41, 40, 39, 38, 36, 35, 34, 33, 32, 31, 30, 30, 29, 28,
        27, 26, 25, 24, 23, 23, 22, 21, 20, 19, 19, 18, 17, 16, 16, 15, 14, 14, 13, 12, 12, 11, 10,
        10, 9, 9, 8, 7, 7, 6, 6, 5, 4, 4, 3, 3, 2, 2, 1, 1, 0, 127, 125, 123, 121, 119, 118, 116,
        114, 113, 111, 109, 108, 106, 105, 103, 102, 100, 99, 97, 96, 95, 93, 92, 91, 90, 88, 87,
        86, 85, 84, 83, 82, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71, 70, 70, 69, 68, 67, 66, 65, 64,
        63, 63, 62, 61, 60, 59, 59, 58, 57, 56, 56, 55, 54, 53,
    ];
    if sub {
        while extract64(sig, s - 1_i32, 1_i32) == 0 {
            exp = exp.wrapping_sub(1);
            sig <<= 1;
        }
        sig = sig << 1_i32 & make_mask64(0_i32, s);
    }
    let mut idx: i32 = ((exp & 1_i32 as u64) << (p - 1_i32) | sig >> (s - p + 1_i32)) as i32;
    let mut out_sig: u64 = (table[idx as usize] as u64) << (s - p);
    let mut out_exp: u64 =
        (3_i32 as u64 * make_mask64(0_i32, e - 1_i32)).wrapping_add(!exp) / 2_i32 as u64;
    sign << (s + e) | out_exp << s | out_sig
}

pub unsafe fn f16_rsqrte7(mut in_0: float16_t) -> float16_t {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = in_0;
    let mut ret: u32 = f16_classify(in_0) as u32;
    let mut sub: bool = false;
    let mut current_block_9: u64;
    match ret {
        2 => {
            current_block_9 = 1701664287637484839;
        }
        4 => {
            current_block_9 = 1701664287637484839;
        }
        1 | 256 => {
            current_block_9 = 6993838161958972860;
        }
        512 => {
            current_block_9 = 17264445587133170540;
        }
        8 => {
            uA.ui = 0xfc00_i32 as u16;
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_infinite as i32) as u8;
            current_block_9 = 4166486009154926805;
        }
        16 => {
            uA.ui = 0x7c00_i32 as u16;
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_infinite as i32) as u8;
            current_block_9 = 4166486009154926805;
        }
        128 => {
            uA.ui = 0_i32 as u16;
            current_block_9 = 4166486009154926805;
        }
        32 => {
            sub = 1_i32 != 0;
            current_block_9 = 15144072073100954468;
        }
        _ => {
            current_block_9 = 15144072073100954468;
        }
    }
    match current_block_9 {
        1701664287637484839 => {
            current_block_9 = 6993838161958972860;
        }
        15144072073100954468 => {
            uA.ui = rsqrte7(uA.ui as u64, 5_i32, 10_i32, sub) as u16;
            current_block_9 = 4166486009154926805;
        }
        _ => {}
    }
    if current_block_9 == 6993838161958972860 {
        softfloat_exceptionFlags =
            (softfloat_exceptionFlags as i32 | softfloat_flag_invalid as i32) as u8;
        current_block_9 = 17264445587133170540;
    }
    if current_block_9 == 17264445587133170540 {
        uA.ui = 0x7e00_i32 as u16;
    }
    uA.f
}

pub unsafe fn f32_rsqrte7(mut in_0: float32_t) -> float32_t {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    uA.f = in_0;
    let mut ret: u32 = f32_classify(in_0) as u32;
    let mut sub: bool = false;
    let mut current_block_9: u64;
    match ret {
        2 => {
            current_block_9 = 8923074298878698145;
        }
        4 => {
            current_block_9 = 8923074298878698145;
        }
        1 | 256 => {
            current_block_9 = 16089598228750784139;
        }
        512 => {
            current_block_9 = 8627194089830818738;
        }
        8 => {
            uA.ui = 0xff800000_u32;
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_infinite as i32) as u8;
            current_block_9 = 4166486009154926805;
        }
        16 => {
            uA.ui = 0x7f800000_i32 as u32;
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_infinite as i32) as u8;
            current_block_9 = 4166486009154926805;
        }
        128 => {
            uA.ui = 0_i32 as u32;
            current_block_9 = 4166486009154926805;
        }
        32 => {
            sub = 1_i32 != 0;
            current_block_9 = 4075303062396226477;
        }
        _ => {
            current_block_9 = 4075303062396226477;
        }
    }
    match current_block_9 {
        8923074298878698145 => {
            current_block_9 = 16089598228750784139;
        }
        4075303062396226477 => {
            uA.ui = rsqrte7(uA.ui as u64, 8_i32, 23_i32, sub) as u32;
            current_block_9 = 4166486009154926805;
        }
        _ => {}
    }
    if current_block_9 == 16089598228750784139 {
        softfloat_exceptionFlags =
            (softfloat_exceptionFlags as i32 | softfloat_flag_invalid as i32) as u8;
        current_block_9 = 8627194089830818738;
    }
    if current_block_9 == 8627194089830818738 {
        uA.ui = 0x7fc00000_i32 as u32;
    }
    uA.f
}

pub unsafe fn f64_rsqrte7(mut in_0: float64_t) -> float64_t {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = in_0;
    let mut ret: u32 = f64_classify(in_0) as u32;
    let mut sub: bool = false;
    let mut current_block_9: u64;
    match ret {
        2 => {
            current_block_9 = 11878696678032717057;
        }
        4 => {
            current_block_9 = 11878696678032717057;
        }
        1 | 256 => {
            current_block_9 = 2627216711007172089;
        }
        512 => {
            current_block_9 = 14146705567854054825;
        }
        8 => {
            uA.ui = 0xfff0000000000000_u64;
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_infinite as i32) as u8;
            current_block_9 = 4166486009154926805;
        }
        16 => {
            uA.ui = 0x7ff0000000000000_u64;
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_infinite as i32) as u8;
            current_block_9 = 4166486009154926805;
        }
        128 => {
            uA.ui = 0_i32 as u64;
            current_block_9 = 4166486009154926805;
        }
        32 => {
            sub = 1_i32 != 0;
            current_block_9 = 17861496924281778896;
        }
        _ => {
            current_block_9 = 17861496924281778896;
        }
    }
    match current_block_9 {
        11878696678032717057 => {
            current_block_9 = 2627216711007172089;
        }
        17861496924281778896 => {
            uA.ui = rsqrte7(uA.ui, 11_i32, 52_i32, sub);
            current_block_9 = 4166486009154926805;
        }
        _ => {}
    }
    if current_block_9 == 2627216711007172089 {
        softfloat_exceptionFlags =
            (softfloat_exceptionFlags as i32 | softfloat_flag_invalid as i32) as u8;
        current_block_9 = 14146705567854054825;
    }
    if current_block_9 == 14146705567854054825 {
        uA.ui = 0x7ff8000000000000_u64;
    }
    uA.f
}

unsafe fn recip7(
    mut val: u64,
    mut e: i32,
    mut s: i32,
    mut rm: i32,
    mut sub: bool,
    mut round_abnormal: *mut bool,
) -> u64 {
    let mut exp: u64 = extract64(val, s, e);
    let mut sig: u64 = extract64(val, 0_i32, s);
    let mut sign: u64 = extract64(val, s + e, 1_i32);
    let p: i32 = 7_i32;
    static mut table: [u8; 128] = [
        127, 125, 123, 121, 119, 117, 116, 114, 112, 110, 109, 107, 105, 104, 102, 100, 99, 97, 96,
        94, 93, 91, 90, 88, 87, 85, 84, 83, 81, 80, 79, 77, 76, 75, 74, 72, 71, 70, 69, 68, 66, 65,
        64, 63, 62, 61, 60, 59, 58, 57, 56, 55, 54, 53, 52, 51, 50, 49, 48, 47, 46, 45, 44, 43, 42,
        41, 40, 40, 39, 38, 37, 36, 35, 35, 34, 33, 32, 31, 31, 30, 29, 28, 28, 27, 26, 25, 25, 24,
        23, 23, 22, 21, 21, 20, 19, 19, 18, 17, 17, 16, 15, 15, 14, 14, 13, 12, 12, 11, 11, 10, 9,
        9, 8, 8, 7, 7, 6, 5, 5, 4, 4, 3, 3, 2, 2, 1, 1, 0,
    ];
    if sub {
        while extract64(sig, s - 1_i32, 1_i32) == 0_i32 as u64 {
            exp = exp.wrapping_sub(1);
            sig <<= 1_i32;
        }
        sig = sig << 1_i32 & make_mask64(0_i32, s);
        if exp != 0_i32 as u64 && exp != 18446744073709551615_u64 {
            *round_abnormal = 1_i32 != 0;
            if rm == 1_i32 || rm == 2_i32 && sign == 0 || rm == 3_i32 && sign != 0 {
                return (sign << (s + e) | make_mask64(s, e)).wrapping_sub(1_i32 as u64);
            } else {
                return sign << (s + e) | make_mask64(s, e);
            }
        }
    }
    let mut idx: i32 = (sig >> (s - p)) as i32;
    let mut out_sig: u64 = (table[idx as usize] as u64) << (s - p);
    let mut out_exp: u64 = (2_i32 as u64 * make_mask64(0_i32, e - 1_i32)).wrapping_add(!exp);
    if out_exp == 0_i32 as u64 || out_exp == 18446744073709551615_u64 {
        out_sig = out_sig >> 1_i32 | make_mask64(s - 1_i32, 1_i32);
        if out_exp == 18446744073709551615_u64 {
            out_sig >>= 1_i32;
            out_exp = 0_i32 as u64;
        }
    }
    sign << (s + e) | out_exp << s | out_sig
}

pub unsafe fn f16_recip7(mut in_0: float16_t) -> float16_t {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = in_0;
    let mut ret: u32 = f16_classify(in_0) as u32;
    let mut sub: bool = false;
    let mut round_abnormal: bool = false;
    let mut current_block_12: u64;
    match ret {
        1 => {
            uA.ui = 0x8000_i32 as u16;
            current_block_12 = 12039483399334584727;
        }
        128 => {
            uA.ui = 0_i32 as u16;
            current_block_12 = 12039483399334584727;
        }
        8 => {
            uA.ui = 0xfc00_i32 as u16;
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_infinite as i32) as u8;
            current_block_12 = 12039483399334584727;
        }
        16 => {
            uA.ui = 0x7c00_i32 as u16;
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_infinite as i32) as u8;
            current_block_12 = 12039483399334584727;
        }
        256 => {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_invalid as i32) as u8;
            current_block_12 = 16950291826939847965;
        }
        512 => {
            current_block_12 = 16950291826939847965;
        }
        4 | 32 => {
            sub = 1_i32 != 0;
            current_block_12 = 17610290921369817802;
        }
        _ => {
            current_block_12 = 17610290921369817802;
        }
    }
    match current_block_12 {
        17610290921369817802 => {
            uA.ui = recip7(
                uA.ui as u64,
                5_i32,
                10_i32,
                softfloat_roundingMode as i32,
                sub,
                &mut round_abnormal,
            ) as u16;
            if round_abnormal {
                softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                    | (softfloat_flag_inexact as i32 | softfloat_flag_overflow as i32))
                    as u8;
            }
        }
        16950291826939847965 => {
            uA.ui = 0x7e00_i32 as u16;
        }
        _ => {}
    }
    uA.f
}

pub unsafe fn f32_recip7(mut in_0: float32_t) -> float32_t {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    uA.f = in_0;
    let mut ret: u32 = f32_classify(in_0) as u32;
    let mut sub: bool = false;
    let mut round_abnormal: bool = false;
    let mut current_block_12: u64;
    match ret {
        1 => {
            uA.ui = 0x80000000_u32;
            current_block_12 = 12039483399334584727;
        }
        128 => {
            uA.ui = 0_i32 as u32;
            current_block_12 = 12039483399334584727;
        }
        8 => {
            uA.ui = 0xff800000_u32;
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_infinite as i32) as u8;
            current_block_12 = 12039483399334584727;
        }
        16 => {
            uA.ui = 0x7f800000_i32 as u32;
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_infinite as i32) as u8;
            current_block_12 = 12039483399334584727;
        }
        256 => {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_invalid as i32) as u8;
            current_block_12 = 17889897109798831544;
        }
        512 => {
            current_block_12 = 17889897109798831544;
        }
        4 | 32 => {
            sub = 1_i32 != 0;
            current_block_12 = 16021920888602068802;
        }
        _ => {
            current_block_12 = 16021920888602068802;
        }
    }
    match current_block_12 {
        16021920888602068802 => {
            uA.ui = recip7(
                uA.ui as u64,
                8_i32,
                23_i32,
                softfloat_roundingMode as i32,
                sub,
                &mut round_abnormal,
            ) as u32;
            if round_abnormal {
                softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                    | (softfloat_flag_inexact as i32 | softfloat_flag_overflow as i32))
                    as u8;
            }
        }
        17889897109798831544 => {
            uA.ui = 0x7fc00000_i32 as u32;
        }
        _ => {}
    }
    uA.f
}

pub unsafe fn f64_recip7(mut in_0: float64_t) -> float64_t {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = in_0;
    let mut ret: u32 = f64_classify(in_0) as u32;
    let mut sub: bool = false;
    let mut round_abnormal: bool = false;
    let mut current_block_12: u64;
    match ret {
        1 => {
            uA.ui = 0x8000000000000000_u64;
            current_block_12 = 12039483399334584727;
        }
        128 => {
            uA.ui = 0_i32 as u64;
            current_block_12 = 12039483399334584727;
        }
        8 => {
            uA.ui = 0xfff0000000000000_u64;
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_infinite as i32) as u8;
            current_block_12 = 12039483399334584727;
        }
        16 => {
            uA.ui = 0x7ff0000000000000_i64 as u64;
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_infinite as i32) as u8;
            current_block_12 = 12039483399334584727;
        }
        256 => {
            softfloat_exceptionFlags =
                (softfloat_exceptionFlags as i32 | softfloat_flag_invalid as i32) as u8;
            current_block_12 = 13014635939034406365;
        }
        512 => {
            current_block_12 = 13014635939034406365;
        }
        4 | 32 => {
            sub = 1_i32 != 0;
            current_block_12 = 7181068393185102741;
        }
        _ => {
            current_block_12 = 7181068393185102741;
        }
    }
    match current_block_12 {
        7181068393185102741 => {
            uA.ui = recip7(
                uA.ui,
                11_i32,
                52_i32,
                softfloat_roundingMode as i32,
                sub,
                &mut round_abnormal,
            );
            if round_abnormal {
                softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                    | (softfloat_flag_inexact as i32 | softfloat_flag_overflow as i32))
                    as u8;
            }
        }
        13014635939034406365 => {
            uA.ui = 0x7ff8000000000000_u64;
        }
        _ => {}
    }
    uA.f
}
