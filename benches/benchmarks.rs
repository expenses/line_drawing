#![feature(test)]

extern crate line_drawing;
extern crate bresenham;
extern crate test;

use line_drawing::*;
use bresenham::Bresenham;
use test::Bencher;

const START: (isize, isize) = (678, 1000);
const END: (isize, isize) = (0, 0);

const START_FLOAT: (f32, f32) = (START.0 as f32, START.1 as f32);
const END_FLOAT: (f32, f32) = (END.0 as f32, END.1 as f32);

#[bench]
fn bench_walk_grid(bencher: &mut Bencher) {
    bencher.iter(|| WalkGrid::new(START, END).count());
}

#[bench]
fn bench_supercover(bencher: &mut Bencher) {
    bencher.iter(|| Supercover::new(START, END).count());
}

#[bench]
fn bench_bresenham(bencher: &mut Bencher) {
    bencher.iter(|| Bresenham::new(START, END).count());
}

#[bench]
fn bench_midpoint(bencher: &mut Bencher) {
    bencher.iter(|| Midpoint::new(START_FLOAT, END_FLOAT).count());
}

#[bench]
fn bench_xiaolin_wu(bencher: &mut Bencher) {
    bencher.iter(|| XiaolinWu::new(START_FLOAT, END_FLOAT).count());
}