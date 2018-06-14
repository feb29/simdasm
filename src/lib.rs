#![allow(non_camel_case_types)]
#![feature(stdsimd)]

use std::simd::{u32x16, u32x4, u32x8, u64x4, u64x8, u8x32, u8x64};

type u8x1 = u8;
type u32x1 = u32;
type u64x1 = u64;

pub fn bitand_u8x1(u1: u8x1, u2: u8x1) -> u8x1 {
    u1 & u2
}
pub fn bitand_u8x32(u1: u8x32, u2: u8x32) -> u8x32 {
    u1 & u2
}
pub fn bitand_u8x64(u1: u8x64, u2: u8x64) -> u8x64 {
    u1 & u2
}

pub fn bitand_u32x1(u1: u32x1, u2: u32x1) -> u32x1 {
    u1 & u2
}
pub fn bitand_u32x8(u1: u32x8, u2: u32x8) -> u32x8 {
    u1 & u2
}
pub fn bitand_u32x16(u1: u32x16, u2: u32x16) -> u32x16 {
    u1 & u2
}

pub fn bitand_u64x1(u1: u64x1, u2: u64x1) -> u64x1 {
    u1 & u2
}
pub fn bitand_u64x4(u1: u64x4, u2: u64x4) -> u64x4 {
    u1 & u2
}
pub fn bitand_u64x8(u1: u64x8, u2: u64x8) -> u64x8 {
    u1 & u2
}

macro_rules! bitand_count1 {
    ($v1:expr, $v2:expr, $lanes:expr) => {{
        let mut ones = 0;
        let new = $v1 & $v2;
        unsafe {
            for i in 0..$lanes {
                ones += new.extract_unchecked(i).count_ones();
            }
        }
        ones
    }};
}

pub fn bitand_u8x1_count1(u1: u8x1, u2: u8x1) -> u32 {
    (u1 & u2).count_ones()
}
pub fn bitand_u8x32_count1(u1: u8x32, u2: u8x32) -> u32 {
    bitand_count1!(u1, u2, 32)
}
pub fn bitand_u8x64_count1(u1: u8x64, u2: u8x64) -> u32 {
    bitand_count1!(u1, u2, 64)
}

pub fn bitand_u32x1_count1(u1: u32x1, u2: u32x1) -> u32 {
    (u1 & u2).count_ones()
}
pub fn bitand_u32x8_count1(u1: u32x8, u2: u32x8) -> u32 {
    bitand_count1!(u1, u2, 8)
}
pub fn bitand_u32x16_count1(u1: u32x16, u2: u32x16) -> u32 {
    bitand_count1!(u1, u2, 16)
}

pub fn bitand_u64x1_count1(u1: u64x1, u2: u64x1) -> u32 {
    (u1 & u2).count_ones()
}
pub fn bitand_u64x4_count1(u1: u64x4, u2: u64x4) -> u32 {
    bitand_count1!(u1, u2, 4)
}
pub fn bitand_u64x8_count1(u1: u64x8, u2: u64x8) -> u32 {
    bitand_count1!(u1, u2, 8)
}

pub fn bitand_u64x1_sum1(v1: &[u64x1], v2: &[u64x1]) -> u32 {
    let mut ones = 0;
    for (&lhs, &rhs) in v1.iter().zip(v2.iter()) {
        ones += (lhs & rhs).count_ones();
    }
    ones
}
pub fn bitand_u64x4_sum1(v1: &[u64x4], v2: &[u64x4]) -> u32 {
    let mut ones = 0;
    for i in 0..v1.len() {
        ones += bitand_count1!(v1[i], v2[i], 4);
    }
    ones
}
pub fn bitand_u64x8_sum1(v1: &[u64x8], v2: &[u64x8]) -> u32 {
    let mut ones = 0;
    for i in 0..v1.len() {
        ones += bitand_count1!(v1[i], v2[i], 8);
    }
    ones
}
