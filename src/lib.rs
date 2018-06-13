#![feature(stdsimd)]

use std::simd;

pub fn bitand_u8x1(u1: u8, u2: u8) -> u8 {
    u1 & u2
}

pub fn bitand_u8x32(u1: simd::u8x32, u2: simd::u8x32) -> simd::u8x32 {
    u1 & u2
}

pub fn bitand_u8x64(u1: simd::u8x64, u2: simd::u8x64) -> simd::u8x64 {
    u1 & u2
}

pub fn bitand_u64x1(u1: u64, u2: u64) -> u64 {
    u1 & u2
}

pub fn bitand_u64x4(u1: simd::u64x4, u2: simd::u64x4) -> simd::u64x4 {
    u1 & u2
}

pub fn bitand_u64x8(u1: simd::u64x8, u2: simd::u64x8) -> simd::u64x8 {
    u1 & u2
}

pub fn and_u64x1_popcnt(v1: &[u64], v2: &[u64]) -> u32 {
    let mut ones = 0;
    for (&lhs, &rhs) in v1.iter().zip(v2.iter()) {
        ones += (lhs & rhs).count_ones();
    }
    ones
}

pub fn and_u64x4(v1: &[simd::u64x4], v2: &[simd::u64x4]) -> Vec<simd::u64x4> {
    let mut vec = Vec::with_capacity(v1.len());
    for i in 0..v1.len() {
        let n = v1[i] & v2[i];
        vec.push(n);
        // unsafe {
        //     for i in 0..4 {
        //         ones += n.extract_unchecked(i).count_ones();
        //     }
        // }
    }
    vec
}

pub fn and_u64x4_popcnt(v1: &[simd::u64x4], v2: &[simd::u64x4]) -> u32 {
    let mut ones = 0;
    for i in 0..v1.len() {
        let n = v1[i] & v2[i];
        unsafe {
            for i in 0..4 {
                ones += n.extract_unchecked(i).count_ones();
            }
        }
    }
    ones
}

pub fn and_u64x8(v1: &[simd::u64x8], v2: &[simd::u64x8]) -> Vec<simd::u64x8> {
    let mut vec = Vec::with_capacity(v1.len());
    for i in 0..v1.len() {
        let n = v1[i] & v2[i];
        vec.push(n);
    }
    vec
}

pub fn and_u64x8_popcnt(v1: &[simd::u64x8], v2: &[simd::u64x8]) -> u32 {
    let mut ones = 0;
    for i in 0..v1.len() {
        let n = v1[i] & v2[i];
        unsafe {
            for i in 0..8 {
                ones += n.extract_unchecked(i).count_ones();
            }
        }
    }
    ones
}
