//! Benchmarks for `Integer` operations (addition, multiplication, etc.)
#![feature(test)]

extern crate test;

use std::hint::black_box;

use arbitrary_precision::Integer;
use test::bench::Bencher;

#[bench]
fn integer_add(bencher: &mut Bencher) {
    bencher.iter(|| {
        let a = black_box(Integer::from(u128::MAX));
        let b = black_box(Integer::from(u128::MAX));
        a + b
    });
}

#[bench]
fn integer_mul(bencher: &mut Bencher) {
    bencher.iter(|| {
        let a = black_box(Integer::from(u128::MAX));
        let b = black_box(Integer::from(u128::MAX));
        a * b
    });
}

#[bench]
fn integer_pow(bencher: &mut Bencher) {
    bencher.iter(|| {
        let a = black_box(Integer::from(u128::MAX));
        let e = 32;
        a.pow(e)
    });
}
