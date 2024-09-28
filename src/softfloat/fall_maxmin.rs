use super::*;

pub unsafe fn f16_max(mut a: float16_t, mut b: float16_t) -> float16_t {
    let mut greater: bool =
        f16_lt_quiet(b, a) as i32 != 0 || f16_eq(b, a) as i32 != 0 && b.v as i32 >> 15_i32 != 0;
    if !(a.v as i32) & 0x7c00_i32 == 0_i32
        && a.v as i32 & 0x3ff_i32 != 0
        && (!(b.v as i32) & 0x7c00_i32 == 0_i32 && b.v as i32 & 0x3ff_i32 != 0)
    {
        let mut ui: ui16_f16 = ui16_f16 { ui: 0 };
        ui.ui = 0x7e00_i32 as u16;
        ui.f
    } else if greater as i32 != 0
        || !(b.v as i32) & 0x7c00_i32 == 0_i32 && b.v as i32 & 0x3ff_i32 != 0
    {
        a
    } else {
        b
    }
}
pub unsafe fn f32_max(mut a: float32_t, mut b: float32_t) -> float32_t {
    let mut greater: bool =
        f32_lt_quiet(b, a) as i32 != 0 || f32_eq(b, a) as i32 != 0 && b.v >> 31_i32 != 0;
    if !a.v & 0x7f800000_i32 as u32 == 0_i32 as u32
        && a.v & 0x7fffff_i32 as u32 != 0
        && (!b.v & 0x7f800000_i32 as u32 == 0_i32 as u32 && b.v & 0x7fffff_i32 as u32 != 0)
    {
        let mut ui: ui32_f32 = ui32_f32 { ui: 0 };
        ui.ui = 0x7fc00000_i32 as u32;
        ui.f
    } else if greater as i32 != 0
        || !b.v & 0x7f800000_i32 as u32 == 0_i32 as u32 && b.v & 0x7fffff_i32 as u32 != 0
    {
        a
    } else {
        b
    }
}
pub unsafe fn f64_max(mut a: float64_t, mut b: float64_t) -> float64_t {
    let mut greater: bool =
        f64_lt_quiet(b, a) as i32 != 0 || f64_eq(b, a) as i32 != 0 && b.v >> 63_i32 != 0;
    if !a.v & 0x7ff0000000000000_u64 == 0_i32 as u64
        && a.v & 0xfffffffffffff != 0
        && (!b.v & 0x7ff0000000000000_u64 == 0_i32 as u64 && b.v & 0xfffffffffffff != 0)
    {
        let mut ui: ui64_f64 = ui64_f64 { ui: 0 };
        ui.ui = 0x7ff8000000000000_u64;
        ui.f
    } else if greater as i32 != 0
        || !b.v & 0x7ff0000000000000_u64 == 0_i32 as u64 && b.v & 0xfffffffffffff != 0
    {
        a
    } else {
        b
    }
}
pub unsafe fn f16_min(mut a: float16_t, mut b: float16_t) -> float16_t {
    let mut less: bool =
        f16_lt_quiet(a, b) as i32 != 0 || f16_eq(a, b) as i32 != 0 && a.v as i32 >> 15_i32 != 0;
    if !(a.v as i32) & 0x7c00_i32 == 0_i32
        && a.v as i32 & 0x3ff_i32 != 0
        && (!(b.v as i32) & 0x7c00_i32 == 0_i32 && b.v as i32 & 0x3ff_i32 != 0)
    {
        let mut ui: ui16_f16 = ui16_f16 { ui: 0 };
        ui.ui = 0x7e00_i32 as u16;
        ui.f
    } else if less as i32 != 0 || !(b.v as i32) & 0x7c00_i32 == 0_i32 && b.v as i32 & 0x3ff_i32 != 0
    {
        a
    } else {
        b
    }
}
pub unsafe fn f32_min(mut a: float32_t, mut b: float32_t) -> float32_t {
    let mut less: bool =
        f32_lt_quiet(a, b) as i32 != 0 || f32_eq(a, b) as i32 != 0 && a.v >> 31_i32 != 0;
    if !a.v & 0x7f800000_i32 as u32 == 0_i32 as u32
        && a.v & 0x7fffff_i32 as u32 != 0
        && (!b.v & 0x7f800000_i32 as u32 == 0_i32 as u32 && b.v & 0x7fffff_i32 as u32 != 0)
    {
        let mut ui: ui32_f32 = ui32_f32 { ui: 0 };
        ui.ui = 0x7fc00000_i32 as u32;
        ui.f
    } else if less as i32 != 0
        || !b.v & 0x7f800000_i32 as u32 == 0_i32 as u32 && b.v & 0x7fffff_i32 as u32 != 0
    {
        a
    } else {
        b
    }
}
pub unsafe fn f64_min(mut a: float64_t, mut b: float64_t) -> float64_t {
    let mut less: bool =
        f64_lt_quiet(a, b) as i32 != 0 || f64_eq(a, b) as i32 != 0 && a.v >> 63_i32 != 0;
    if !a.v & 0x7ff0000000000000_u64 == 0_i32 as u64
        && a.v & 0xfffffffffffff != 0
        && (!b.v & 0x7ff0000000000000_u64 == 0_i32 as u64 && b.v & 0xfffffffffffff != 0)
    {
        let mut ui: ui64_f64 = ui64_f64 { ui: 0 };
        ui.ui = 0x7ff8000000000000_u64;
        ui.f
    } else if less as i32 != 0
        || !b.v & 0x7ff0000000000000_u64 == 0_i32 as u64 && b.v & 0xfffffffffffff != 0
    {
        a
    } else {
        b
    }
}
pub unsafe fn f16_maximum(mut a: float16_t, mut b: float16_t) -> float16_t {
    let mut greater: bool =
        f16_lt_quiet(b, a) as i32 != 0 || f16_eq(b, a) as i32 != 0 && b.v as i32 >> 15_i32 != 0;
    if !(a.v as i32) & 0x7c00_i32 == 0_i32 && a.v as i32 & 0x3ff_i32 != 0
        || !(b.v as i32) & 0x7c00_i32 == 0_i32 && b.v as i32 & 0x3ff_i32 != 0
    {
        let mut ui: ui16_f16 = ui16_f16 { ui: 0 };
        ui.ui = 0x7e00_i32 as u16;
        ui.f
    } else if greater as i32 != 0
        || !(b.v as i32) & 0x7c00_i32 == 0_i32 && b.v as i32 & 0x3ff_i32 != 0
    {
        a
    } else {
        b
    }
}
pub unsafe fn f32_maximum(mut a: float32_t, mut b: float32_t) -> float32_t {
    let mut greater: bool =
        f32_lt_quiet(b, a) as i32 != 0 || f32_eq(b, a) as i32 != 0 && b.v >> 31_i32 != 0;
    if !a.v & 0x7f800000_i32 as u32 == 0_i32 as u32 && a.v & 0x7fffff_i32 as u32 != 0
        || !b.v & 0x7f800000_i32 as u32 == 0_i32 as u32 && b.v & 0x7fffff_i32 as u32 != 0
    {
        let mut ui: ui32_f32 = ui32_f32 { ui: 0 };
        ui.ui = 0x7fc00000_i32 as u32;
        ui.f
    } else if greater as i32 != 0
        || !b.v & 0x7f800000_i32 as u32 == 0_i32 as u32 && b.v & 0x7fffff_i32 as u32 != 0
    {
        a
    } else {
        b
    }
}
pub unsafe fn f64_maximum(mut a: float64_t, mut b: float64_t) -> float64_t {
    let mut greater: bool =
        f64_lt_quiet(b, a) as i32 != 0 || f64_eq(b, a) as i32 != 0 && b.v >> 63_i32 != 0;
    if !a.v & 0x7ff0000000000000_u64 == 0_i32 as u64 && a.v & 0xfffffffffffff != 0
        || !b.v & 0x7ff0000000000000_u64 == 0_i32 as u64 && b.v & 0xfffffffffffff != 0
    {
        let mut ui: ui64_f64 = ui64_f64 { ui: 0 };
        ui.ui = 0x7ff8000000000000_u64;
        ui.f
    } else if greater as i32 != 0
        || !b.v & 0x7ff0000000000000_u64 == 0_i32 as u64 && b.v & 0xfffffffffffff != 0
    {
        a
    } else {
        b
    }
}
pub unsafe fn f16_minimum(mut a: float16_t, mut b: float16_t) -> float16_t {
    let mut less: bool =
        f16_lt_quiet(a, b) as i32 != 0 || f16_eq(a, b) as i32 != 0 && a.v as i32 >> 15_i32 != 0;
    if !(a.v as i32) & 0x7c00_i32 == 0_i32 && a.v as i32 & 0x3ff_i32 != 0
        || !(b.v as i32) & 0x7c00_i32 == 0_i32 && b.v as i32 & 0x3ff_i32 != 0
    {
        let mut ui: ui16_f16 = ui16_f16 { ui: 0 };
        ui.ui = 0x7e00_i32 as u16;
        ui.f
    } else if less as i32 != 0 || !(b.v as i32) & 0x7c00_i32 == 0_i32 && b.v as i32 & 0x3ff_i32 != 0
    {
        a
    } else {
        b
    }
}
pub unsafe fn f32_minimum(mut a: float32_t, mut b: float32_t) -> float32_t {
    let mut less: bool =
        f32_lt_quiet(a, b) as i32 != 0 || f32_eq(a, b) as i32 != 0 && a.v >> 31_i32 != 0;
    if !a.v & 0x7f800000_i32 as u32 == 0_i32 as u32 && a.v & 0x7fffff_i32 as u32 != 0
        || !b.v & 0x7f800000_i32 as u32 == 0_i32 as u32 && b.v & 0x7fffff_i32 as u32 != 0
    {
        let mut ui: ui32_f32 = ui32_f32 { ui: 0 };
        ui.ui = 0x7fc00000_i32 as u32;
        ui.f
    } else if less as i32 != 0
        || !b.v & 0x7f800000_i32 as u32 == 0_i32 as u32 && b.v & 0x7fffff_i32 as u32 != 0
    {
        a
    } else {
        b
    }
}
pub unsafe fn f64_minimum(mut a: float64_t, mut b: float64_t) -> float64_t {
    let mut less: bool =
        f64_lt_quiet(a, b) as i32 != 0 || f64_eq(a, b) as i32 != 0 && a.v >> 63_i32 != 0;
    if !a.v & 0x7ff0000000000000_u64 == 0_i32 as u64 && a.v & 0xfffffffffffff != 0
        || !b.v & 0x7ff0000000000000_u64 == 0_i32 as u64 && b.v & 0xfffffffffffff != 0
    {
        let mut ui: ui64_f64 = ui64_f64 { ui: 0 };
        ui.ui = 0x7ff8000000000000_u64;
        ui.f
    } else if less as i32 != 0
        || !b.v & 0x7ff0000000000000_u64 == 0_i32 as u64 && b.v & 0xfffffffffffff != 0
    {
        a
    } else {
        b
    }
}
