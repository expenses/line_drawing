#![cfg(test)]

extern crate rand;

use self::rand::Rng;
use self::rand::distributions::range::SampleRange;
use ::*;

use std::ops::Neg;

const NUM_TESTS: u16 = 10000;
const RANGE: isize = 500;
const RANGE_FLOAT: f32 = 500.0;

pub fn reverse_slice<T: Clone>(points: &[T]) -> Vec<T> {
    points.iter().rev().cloned().collect()
}

fn random_point<T>(rng: &mut rand::ThreadRng, range: T) -> Point<T>
where
    T: SampleRange + PartialOrd + Neg<Output = T> + Copy,
{
    (rng.gen_range(-range, range), rng.gen_range(-range, range))
}

fn random_voxel<T>(rng: &mut rand::ThreadRng, range: T) -> Voxel<T>
where
    T: SampleRange + PartialOrd + Neg<Output = T> + Copy,
{
    (
        rng.gen_range(-range, range),
        rng.gen_range(-range, range),
        rng.gen_range(-range, range),
    )
}

#[test]
fn supercover_symmetrical() {
    let supercover = |a, b| Supercover::new(a, b).collect::<Vec<_>>();

    let mut rng = rand::thread_rng();

    for _ in 0..NUM_TESTS {
        let start = random_point(&mut rng, RANGE);
        let end = random_point(&mut rng, RANGE);

        assert_eq!(
            supercover(start, end),
            reverse_slice(&supercover(end, start))
        );
    }
}

#[test]
#[should_panic]
fn bresenham_not_symmetrical() {
    let bresenham = |a, b| Bresenham::new(a, b).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_TESTS {
        let start = random_point(&mut rng, RANGE);
        let end = random_point(&mut rng, RANGE);

        assert_eq!(bresenham(start, end), reverse_slice(&bresenham(end, start)));
    }
}

#[test]
#[should_panic]
fn bresenham_3d_not_symmetrical() {
    let bresenham_3d = |a, b| Bresenham3d::new(a, b).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_TESTS {
        let start = random_voxel(&mut rng, RANGE);
        let end = random_voxel(&mut rng, RANGE);

        assert_eq!(
            bresenham_3d(start, end),
            reverse_slice(&bresenham_3d(end, start))
        );
    }
}

#[test]
#[should_panic]
fn walk_voxels_symmetrical() {
    let walk_voxels = |a, b| WalkVoxels::<_, i16>::new(a, b).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_TESTS {
        let start = random_voxel(&mut rng, RANGE_FLOAT);
        let end = random_voxel(&mut rng, RANGE_FLOAT);

        assert_eq!(
            walk_voxels(start, end),
            reverse_slice(&walk_voxels(end, start))
        );
    }
}
