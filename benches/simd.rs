#![feature(stdsimd)]
#![feature(test)]

extern crate simdasm;
extern crate test;

use std::simd;
use test::Bencher;

#[bench]
fn bitand_u64x1_sum1(bench: &mut Bencher) {
    let v1 = vec![
        9213812,
        12039812093,
        1203981023,
        12039812,
        198230123,
        238012,
        982137,
        9018237,
    ];
    let v2 = vec![
        929012313812,
        12030912832093,
        12039810923823,
        12039812192323,
        1110198230123,
        29821338012,
        98213701923,
        90182370989899,
    ];
    bench.iter(|| simdasm::bitand_u64x1_sum1(&v1[..], &v2[..]));
}

#[bench]
fn bitand_u64x4_sum1(bench: &mut Bencher) {
    let v1 = vec![
        simd::u64x4::new(9213812, 12039812093, 1203981023, 12039812),
        simd::u64x4::new(198230123, 238012, 982137, 9018237),
    ];
    let v2 = vec![
        simd::u64x4::new(929012313812, 12030912832093, 12039810923823, 12039812192323),
        simd::u64x4::new(1110198230123, 29821338012, 98213701923, 90182370989899),
    ];
    bench.iter(|| simdasm::bitand_u64x4_sum1(&v1[..], &v2[..]));
}
