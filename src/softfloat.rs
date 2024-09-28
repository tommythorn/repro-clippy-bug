// ============================================================================
//
// This C source file is part of the SoftFloat IEEE Floating-Point Arithmetic
// Package, Release 3d, by John R. Hauser.
//
// Copyright 2011, 2012, 2013, 2014, 2015, 2016, 2017 The Regents of the
// University of California.  All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions, and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions, and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// 3. Neither the name of the University nor the names of its contributors may
// be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS "AS IS", AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE, ARE
// DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// =============================================================================

// The mechanical output of C2Rust is expected to have a lot of
// ... less than ideal code that Clippy will complain about.

#![allow(
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::unnecessary_mut_passed,
    non_snake_case,
    non_upper_case_globals,
    unused_mut
)]

pub mod f128_add;
pub mod f128_classify;
pub mod f128_div;
pub mod f128_eq;
pub mod f128_eq_signaling;
pub mod f128_isSignalingNaN;
pub mod f128_le;
pub mod f128_le_quiet;
pub mod f128_lt;
pub mod f128_lt_quiet;
pub mod f128_mul;
pub mod f128_mulAdd;
pub mod f128_rem;
pub mod f128_roundToInt;
pub mod f128_sqrt;
pub mod f128_sub;
pub mod f128_to_f16;
pub mod f128_to_f32;
pub mod f128_to_f64;
pub mod f128_to_i32;
pub mod f128_to_i32_r_minMag;
pub mod f128_to_i64;
pub mod f128_to_i64_r_minMag;
pub mod f128_to_ui32;
pub mod f128_to_ui32_r_minMag;
pub mod f128_to_ui64;
pub mod f128_to_ui64_r_minMag;
pub mod f16_add;
pub mod f16_classify;
pub mod f16_div;
pub mod f16_eq;
pub mod f16_eq_signaling;
pub mod f16_isSignalingNaN;
pub mod f16_le;
pub mod f16_le_quiet;
pub mod f16_lt;
pub mod f16_lt_quiet;
pub mod f16_mul;
pub mod f16_mulAdd;
pub mod f16_rem;
pub mod f16_roundToInt;
pub mod f16_sqrt;
pub mod f16_sub;
pub mod f16_to_f128;
pub mod f16_to_f32;
pub mod f16_to_f64;
pub mod f16_to_i16;
pub mod f16_to_i32;
pub mod f16_to_i32_r_minMag;
pub mod f16_to_i64;
pub mod f16_to_i64_r_minMag;
pub mod f16_to_i8;
pub mod f16_to_ui16;
pub mod f16_to_ui32;
pub mod f16_to_ui32_r_minMag;
pub mod f16_to_ui64;
pub mod f16_to_ui64_r_minMag;
pub mod f16_to_ui8;
pub mod f32_add;
pub mod f32_classify;
pub mod f32_div;
pub mod f32_eq;
pub mod f32_eq_signaling;
pub mod f32_isSignalingNaN;
pub mod f32_le;
pub mod f32_le_quiet;
pub mod f32_lt;
pub mod f32_lt_quiet;
pub mod f32_mul;
pub mod f32_mulAdd;
pub mod f32_rem;
pub mod f32_roundToInt;
pub mod f32_sqrt;
pub mod f32_sub;
pub mod f32_to_f128;
pub mod f32_to_f16;
pub mod f32_to_f64;
pub mod f32_to_i16;
pub mod f32_to_i32;
pub mod f32_to_i32_r_minMag;
pub mod f32_to_i64;
pub mod f32_to_i64_r_minMag;
pub mod f32_to_ui16;
pub mod f32_to_ui32;
pub mod f32_to_ui32_r_minMag;
pub mod f32_to_ui64;
pub mod f32_to_ui64_r_minMag;
pub mod f64_add;
pub mod f64_classify;
pub mod f64_div;
pub mod f64_eq;
pub mod f64_eq_signaling;
pub mod f64_isSignalingNaN;
pub mod f64_le;
pub mod f64_le_quiet;
pub mod f64_lt;
pub mod f64_lt_quiet;
pub mod f64_mul;
pub mod f64_mulAdd;
pub mod f64_rem;
pub mod f64_roundToInt;
pub mod f64_sqrt;
pub mod f64_sub;
pub mod f64_to_f128;
pub mod f64_to_f16;
pub mod f64_to_f32;
pub mod f64_to_i32;
pub mod f64_to_i32_mod_r_minMag;
pub mod f64_to_i32_r_minMag;
pub mod f64_to_i64;
pub mod f64_to_i64_r_minMag;
pub mod f64_to_ui32;
pub mod f64_to_ui32_r_minMag;
pub mod f64_to_ui64;
pub mod f64_to_ui64_r_minMag;
pub mod fall_maxmin;
pub mod fall_reciprocal;
pub mod i32_to_f128;
pub mod i32_to_f16;
pub mod i32_to_f32;
pub mod i32_to_f64;
pub mod i64_to_f128;
pub mod i64_to_f16;
pub mod i64_to_f32;
pub mod i64_to_f64;
pub mod s_add128;
pub mod s_add256M;
pub mod s_addCarryM;
pub mod s_addComplCarryM;
pub mod s_addM;
pub mod s_addMagsF128;
pub mod s_addMagsF16;
pub mod s_addMagsF32;
pub mod s_addMagsF64;
pub mod s_approxRecip32_1;
pub mod s_approxRecipSqrt32_1;
pub mod s_approxRecipSqrt_1Ks;
pub mod s_approxRecip_1Ks;
pub mod s_compare128M;
pub mod s_compare96M;
pub mod s_countLeadingZeros16;
pub mod s_countLeadingZeros32;
pub mod s_countLeadingZeros64;
pub mod s_countLeadingZeros8;
pub mod s_eq128;
pub mod s_le128;
pub mod s_lt128;
pub mod s_mul128By32;
pub mod s_mul128MTo256M;
pub mod s_mul128To256M;
pub mod s_mul64ByShifted32To128;
pub mod s_mul64To128;
pub mod s_mul64To128M;
pub mod s_mulAddF128;
pub mod s_mulAddF16;
pub mod s_mulAddF32;
pub mod s_mulAddF64;
pub mod s_negXM;
pub mod s_normRoundPackToF128;
pub mod s_normRoundPackToF16;
pub mod s_normRoundPackToF32;
pub mod s_normRoundPackToF64;
pub mod s_normSubnormalF128Sig;
pub mod s_normSubnormalF16Sig;
pub mod s_normSubnormalF32Sig;
pub mod s_normSubnormalF64Sig;
pub mod s_propagateNaNF128UI;
pub mod s_propagateNaNF16UI;
pub mod s_propagateNaNF32UI;
pub mod s_propagateNaNF64UI;
pub mod s_remStepMBy32;
pub mod s_roundMToI64;
pub mod s_roundMToUI64;
pub mod s_roundPackMToI64;
pub mod s_roundPackMToUI64;
pub mod s_roundPackToF128;
pub mod s_roundPackToF16;
pub mod s_roundPackToF32;
pub mod s_roundPackToF64;
pub mod s_roundPackToI32;
pub mod s_roundPackToI64;
pub mod s_roundPackToUI32;
pub mod s_roundPackToUI64;
pub mod s_roundToI32;
pub mod s_roundToI64;
pub mod s_roundToUI32;
pub mod s_roundToUI64;
pub mod s_shiftRightJam128;
pub mod s_shiftRightJam128Extra;
pub mod s_shiftRightJam256M;
pub mod s_shiftRightJam32;
pub mod s_shiftRightJam64;
pub mod s_shiftRightJam64Extra;
pub mod s_shortShiftLeft128;
pub mod s_shortShiftLeft64To96M;
pub mod s_shortShiftRight128;
pub mod s_shortShiftRightExtendM;
pub mod s_shortShiftRightJam128;
pub mod s_shortShiftRightJam128Extra;
pub mod s_shortShiftRightJam64;
pub mod s_shortShiftRightJam64Extra;
pub mod s_shortShiftRightM;
pub mod s_sub128;
pub mod s_sub1XM;
pub mod s_sub256M;
pub mod s_subM;
pub mod s_subMagsF128;
pub mod s_subMagsF16;
pub mod s_subMagsF32;
pub mod s_subMagsF64;
pub mod softfloat_raiseFlags;
pub mod softfloat_state;
pub mod ui32_to_f128;
pub mod ui32_to_f16;
pub mod ui32_to_f32;
pub mod ui32_to_f64;
pub mod ui64_to_f128;
pub mod ui64_to_f16;
pub mod ui64_to_f32;
pub mod ui64_to_f64;

pub use f128_add::*;
pub use f128_classify::*;
pub use f128_div::*;
pub use f128_eq::*;
pub use f128_eq_signaling::*;
pub use f128_isSignalingNaN::*;
pub use f128_le::*;
pub use f128_le_quiet::*;
pub use f128_lt::*;
pub use f128_lt_quiet::*;
pub use f128_mul::*;
pub use f128_mulAdd::*;
pub use f128_rem::*;
pub use f128_roundToInt::*;
pub use f128_sqrt::*;
pub use f128_sub::*;
pub use f128_to_f16::*;
pub use f128_to_f32::*;
pub use f128_to_f64::*;
pub use f128_to_i32::*;
pub use f128_to_i32_r_minMag::*;
pub use f128_to_i64::*;
pub use f128_to_i64_r_minMag::*;
pub use f128_to_ui32::*;
pub use f128_to_ui32_r_minMag::*;
pub use f128_to_ui64::*;
pub use f128_to_ui64_r_minMag::*;
pub use f16_add::*;
pub use f16_classify::*;
pub use f16_div::*;
pub use f16_eq::*;
pub use f16_eq_signaling::*;
pub use f16_isSignalingNaN::*;
pub use f16_le::*;
pub use f16_le_quiet::*;
pub use f16_lt::*;
pub use f16_lt_quiet::*;
pub use f16_mul::*;
pub use f16_mulAdd::*;
pub use f16_rem::*;
pub use f16_roundToInt::*;
pub use f16_sqrt::*;
pub use f16_sub::*;
pub use f16_to_f128::*;
pub use f16_to_f32::*;
pub use f16_to_f64::*;
pub use f16_to_i16::*;
pub use f16_to_i32::*;
pub use f16_to_i32_r_minMag::*;
pub use f16_to_i64::*;
pub use f16_to_i64_r_minMag::*;
pub use f16_to_i8::*;
pub use f16_to_ui16::*;
pub use f16_to_ui32::*;
pub use f16_to_ui32_r_minMag::*;
pub use f16_to_ui64::*;
pub use f16_to_ui64_r_minMag::*;
pub use f16_to_ui8::*;
pub use f32_add::*;
pub use f32_classify::*;
pub use f32_div::*;
pub use f32_eq::*;
pub use f32_eq_signaling::*;
pub use f32_isSignalingNaN::*;
pub use f32_le::*;
pub use f32_le_quiet::*;
pub use f32_lt::*;
pub use f32_lt_quiet::*;
pub use f32_mul::*;
pub use f32_mulAdd::*;
pub use f32_rem::*;
pub use f32_roundToInt::*;
pub use f32_sqrt::*;
pub use f32_sub::*;
pub use f32_to_f128::*;
pub use f32_to_f16::*;
pub use f32_to_f64::*;
pub use f32_to_i16::*;
pub use f32_to_i32::*;
pub use f32_to_i32_r_minMag::*;
pub use f32_to_i64::*;
pub use f32_to_i64_r_minMag::*;
pub use f32_to_ui16::*;
pub use f32_to_ui32::*;
pub use f32_to_ui32_r_minMag::*;
pub use f32_to_ui64::*;
pub use f32_to_ui64_r_minMag::*;
pub use f64_add::*;
pub use f64_classify::*;
pub use f64_div::*;
pub use f64_eq::*;
pub use f64_eq_signaling::*;
pub use f64_isSignalingNaN::*;
pub use f64_le::*;
pub use f64_le_quiet::*;
pub use f64_lt::*;
pub use f64_lt_quiet::*;
pub use f64_mul::*;
pub use f64_mulAdd::*;
pub use f64_rem::*;
pub use f64_roundToInt::*;
pub use f64_sqrt::*;
pub use f64_sub::*;
pub use f64_to_f128::*;
pub use f64_to_f16::*;
pub use f64_to_f32::*;
pub use f64_to_i32::*;
pub use f64_to_i32_mod_r_minMag::*;
pub use f64_to_i32_r_minMag::*;
pub use f64_to_i64::*;
pub use f64_to_i64_r_minMag::*;
pub use f64_to_ui32::*;
pub use f64_to_ui32_r_minMag::*;
pub use f64_to_ui64::*;
pub use f64_to_ui64_r_minMag::*;
pub use fall_maxmin::*;
pub use fall_reciprocal::*;
pub use i32_to_f128::*;
pub use i32_to_f16::*;
pub use i32_to_f32::*;
pub use i32_to_f64::*;
pub use i64_to_f128::*;
pub use i64_to_f16::*;
pub use i64_to_f32::*;
pub use i64_to_f64::*;
pub use s_add128::*;
pub use s_add256M::*;
pub use s_addCarryM::*;
pub use s_addComplCarryM::*;
pub use s_addM::*;
pub use s_addMagsF128::*;
pub use s_addMagsF16::*;
pub use s_addMagsF32::*;
pub use s_addMagsF64::*;
pub use s_approxRecip32_1::*;
pub use s_approxRecipSqrt32_1::*;
pub use s_approxRecipSqrt_1Ks::*;
pub use s_approxRecip_1Ks::*;
pub use s_compare128M::*;
pub use s_compare96M::*;
pub use s_countLeadingZeros16::*;
pub use s_countLeadingZeros32::*;
pub use s_countLeadingZeros64::*;
pub use s_countLeadingZeros8::*;
pub use s_eq128::*;
pub use s_le128::*;
pub use s_lt128::*;
pub use s_mul128By32::*;
pub use s_mul128MTo256M::*;
pub use s_mul128To256M::*;
pub use s_mul64ByShifted32To128::*;
pub use s_mul64To128::*;
pub use s_mul64To128M::*;
pub use s_mulAddF128::*;
pub use s_mulAddF16::*;
pub use s_mulAddF32::*;
pub use s_mulAddF64::*;
pub use s_negXM::*;
pub use s_normRoundPackToF128::*;
pub use s_normRoundPackToF16::*;
pub use s_normRoundPackToF32::*;
pub use s_normRoundPackToF64::*;
pub use s_normSubnormalF128Sig::*;
pub use s_normSubnormalF16Sig::*;
pub use s_normSubnormalF32Sig::*;
pub use s_normSubnormalF64Sig::*;
pub use s_propagateNaNF128UI::*;
pub use s_propagateNaNF16UI::*;
pub use s_propagateNaNF32UI::*;
pub use s_propagateNaNF64UI::*;
pub use s_remStepMBy32::*;
pub use s_roundMToI64::*;
pub use s_roundMToUI64::*;
pub use s_roundPackMToI64::*;
pub use s_roundPackMToUI64::*;
pub use s_roundPackToF128::*;
pub use s_roundPackToF16::*;
pub use s_roundPackToF32::*;
pub use s_roundPackToF64::*;
pub use s_roundPackToI32::*;
pub use s_roundPackToI64::*;
pub use s_roundPackToUI32::*;
pub use s_roundPackToUI64::*;
pub use s_roundToI32::*;
pub use s_roundToI64::*;
pub use s_roundToUI32::*;
pub use s_roundToUI64::*;
pub use s_shiftRightJam128::*;
pub use s_shiftRightJam128Extra::*;
pub use s_shiftRightJam256M::*;
pub use s_shiftRightJam32::*;
pub use s_shiftRightJam64::*;
pub use s_shiftRightJam64Extra::*;
pub use s_shortShiftLeft128::*;
pub use s_shortShiftLeft64To96M::*;
pub use s_shortShiftRight128::*;
pub use s_shortShiftRightExtendM::*;
pub use s_shortShiftRightJam128::*;
pub use s_shortShiftRightJam128Extra::*;
pub use s_shortShiftRightJam64::*;
pub use s_shortShiftRightJam64Extra::*;
pub use s_shortShiftRightM::*;
pub use s_sub128::*;
pub use s_sub1XM::*;
pub use s_sub256M::*;
pub use s_subM::*;
pub use s_subMagsF128::*;
pub use s_subMagsF16::*;
pub use s_subMagsF32::*;
pub use s_subMagsF64::*;
pub use softfloat_raiseFlags::*;
pub use softfloat_state::*;
pub use ui32_to_f128::*;
pub use ui32_to_f16::*;
pub use ui32_to_f32::*;
pub use ui32_to_f64::*;
pub use ui64_to_f128::*;
pub use ui64_to_f16::*;
pub use ui64_to_f32::*;
pub use ui64_to_f64::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct commonNaN {
    pub _unused: i8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp32_sig128 {
    pub exp: i64,
    pub sig: uint128,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp16_sig32 {
    pub exp: i64,
    pub sig: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp16_sig64 {
    pub exp: i64,
    pub sig: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp8_sig16 {
    pub exp: i8,
    pub sig: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct float16_t {
    pub v: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct float16x2_t {
    pub v: [float16_t; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct float128_t {
    pub v: [u64; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct float32_t {
    pub v: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct float64_t {
    pub v: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint128 {
    pub v0: u64,
    pub v64: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint128_extra {
    pub extra: u64,
    pub v: uint128,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint64_extra {
    pub extra: u64,
    pub v: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui16_f16 {
    pub ui: u16,
    pub f: float16_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: u32,
    pub f: float32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_i32 {
    pub ui: u32,
    pub i: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: u64,
    pub f: float64_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_i64 {
    pub ui: u64,
    pub i: i64,
}

pub const softfloat_flag_inexact: u32 = 1;
pub const softfloat_flag_infinite: u32 = 8;
pub const softfloat_flag_invalid: u32 = 16;
pub const softfloat_flag_overflow: u32 = 4;
pub const softfloat_flag_underflow: u32 = 2;
pub const softfloat_mulAdd_divProd: u32 = 4;
pub const softfloat_mulAdd_subC: u32 = 1;
pub const softfloat_mulAdd_subProd: u32 = 2;
pub const softfloat_round_max: u32 = 3;
pub const softfloat_round_min: u32 = 2;
pub const softfloat_round_minMag: u32 = 1;
pub const softfloat_round_near_even: u32 = 0;
pub const softfloat_round_near_maxMag: u32 = 4;
pub const softfloat_round_odd: u32 = 5;
pub const softfloat_tininess_beforeRounding: u32 = 0;
pub const softfloat_tininess_afterRounding: u32 = 1;

unsafe fn softfloat_commonNaNToF128UI(_aPtr: *const commonNaN) -> uint128 {
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    uiZ.v64 = 0x7fff800000000000_u64;
    uiZ.v0 = 0_u64;
    uiZ
}
