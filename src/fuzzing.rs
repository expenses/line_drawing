#![cfg(test)]

extern crate rand;

use self::rand::Rng;
use *;

const MAX_TESTS: u16 = 10000;
const RANGE: isize = 5;

fn random_points(rng: &mut rand::ThreadRng) -> (Point<isize>, Point<isize>) {
    (
        (rng.gen_range(-RANGE, RANGE), rng.gen_range(-RANGE, RANGE)),
        (rng.gen_range(-RANGE, RANGE), rng.gen_range(-RANGE, RANGE))
    )
}

#[test]
fn supercover_symmetrical() {
    let mut rng = rand::thread_rng();

    for _ in 0 .. MAX_TESTS {
        let (a, b) = random_points(&mut rng);

        assert_eq!(
            supercover(a, b),
            reverse(&supercover(b, a))
        );
    }
}

#[test]
#[should_panic]
fn bresenham_not_symmetrical() {
    let mut rng = rand::thread_rng();

    for _ in 0 .. MAX_TESTS {
        let (a, b) = random_points(&mut rng); 

        assert_eq!(bresenham(a, b), reverse(&bresenham(a, b)));
    }
}