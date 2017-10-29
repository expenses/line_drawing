#![feature(test)]

extern crate line_drawing;
extern crate bresenham;
extern crate test;

use line_drawing::*;
use test::Bencher;

const START: (isize, isize) = (678, 1000);
const END: (isize, isize) = (0, 0);

const START_FLOAT: (f32, f32) = (START.0 as f32, START.1 as f32);
const END_FLOAT: (f32, f32) = (END.0 as f32, END.1 as f32);

const START_VOXEL: (isize, isize, isize) = (START.0, START.1, 0);
const END_VOXEL: (isize, isize, isize) = (END.0, END.1, 0);

const START_VOXEL_FLOAT: (f32, f32, f32) = (START_FLOAT.0, START_FLOAT.1, 0.0);
const END_VOXEL_FLOAT: (f32, f32, f32) = (END_FLOAT.0, END_FLOAT.1, 0.0);

#[bench]
fn bench_bresenham(bencher: &mut Bencher) {
    bencher.iter(|| black_box(Bresenham::new(START, END)));
}

#[bench]
fn bench_bresenham_crate(bencher: &mut Bencher) {
    bencher.iter(|| black_box(bresenham::Bresenham::new(START, END)));
}

#[bench]
fn bench_walk_grid(bencher: &mut Bencher) {
    bencher.iter(|| black_box(WalkGrid::new(START, END)));
}

#[bench]
fn bench_supercover(bencher: &mut Bencher) {
    bencher.iter(|| black_box(Supercover::new(START, END)));
}

#[bench]
fn bench_midpoint(bencher: &mut Bencher) {
    bencher.iter(|| black_box(Midpoint::<_, isize>::new(START_FLOAT, END_FLOAT)));
}

#[bench]
fn bench_xiaolin_wu(bencher: &mut Bencher) {
    bencher.iter(|| black_box(XiaolinWu::<_, isize>::new(START_FLOAT, END_FLOAT)));
}

#[bench]
fn bench_bresenham_3d(bencher: &mut Bencher) {
    bencher.iter(|| black_box(Bresenham3d::new(START_VOXEL, END_VOXEL)));
}

#[bench]
fn bench_walk_voxels(bencher: &mut Bencher) {
    bencher.iter(|| black_box(WalkVoxels::<_, isize>::new(START_VOXEL_FLOAT, END_VOXEL_FLOAT)));
}

#[bench]
fn bench_steps_bresenham(bencher: &mut Bencher) {
    bencher.iter(|| black_box(Bresenham::new(START, END).steps()));
}

#[bench]
fn bench_bresenham_circle(bencher: &mut Bencher) {
    bencher.iter(|| black_box(BresenhamCircle::new(0, 0, 300)));
}

#[inline]
fn black_box<T: Iterator>(iter: T) {
    for item in iter {
        test::black_box(item);
    }
}