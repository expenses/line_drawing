#![feature(test)]

extern crate line_drawing;
extern crate test;

use line_drawing::*;
use test::Bencher;

const START: (isize, isize) = (678, 1000);
const END: (isize, isize) = (0, 0);

const START_FLOAT: (f32, f32) = (START.0 as f32, START.1 as f32);
const END_FLOAT: (f32, f32) = (END.0 as f32, END.1 as f32);

#[bench]
fn bench_walk_grid(bencher: &mut Bencher) {
    bencher.iter(|| walk_grid(START, END));
}

#[bench]
fn bench_supercover(bencher: &mut Bencher) {
    bencher.iter(|| supercover(START, END));
}

#[bench]
fn bench_bresenham(bencher: &mut Bencher) {
    bencher.iter(|| bresenham(START, END));
}

#[bench]
fn bench_midpoint(bencher: &mut Bencher) {
    bencher.iter(|| midpoint(START_FLOAT, END_FLOAT, false));
}

#[bench]
fn bench_midpoint_orthagonal(bencher: &mut Bencher) {
    bencher.iter(|| midpoint(START_FLOAT, END_FLOAT, false));
}

#[bench]
fn bench_xiaolin_wu(bencher: &mut Bencher) {
    bencher.iter(|| xiaolin_wu(START_FLOAT, END_FLOAT));
}

#[bench]
fn bench_sorted_walk_grid(bencher: &mut Bencher) {
    bencher.iter(|| sorted_walk_grid(START, END));
}

#[bench]
fn bench_sorted_bresenham(bencher: &mut Bencher) {
    bencher.iter(|| sorted_bresenham(START, END));
}

#[bench]
fn bench_sorted_midpoint(bencher: &mut Bencher) {
    bencher.iter(|| sorted_midpoint(START_FLOAT, END_FLOAT, false));
}

#[bench]
fn bench_sorted_midpoint_orthogonal(bencher: &mut Bencher) {
    bencher.iter(|| sorted_midpoint(START_FLOAT, END_FLOAT, true));
}

#[bench]
fn bench_sorted_xiaolin_wu(bencher: &mut Bencher) {
    bencher.iter(|| sorted_xiaolin_wu(START_FLOAT, END_FLOAT));
}