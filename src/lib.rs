//! A collection of line-drawing algorithms for use in graphics and video games.
//!
//! Currently implemented:
//!
//! * [`bresenham`] through [`bresenham-rs`].
//! * The [mid-point line algorithm].
//! * [Xiaolin Wu's line algorithm].
//! * [`WalkGrid`] and [`Supercover`] implemented from [this article by Red Blob Games][article].
//! * [`Bresenham3d`] - A 3-Dimensional implementation of bresenham.
//! * [`WalkVoxels`] - A similar 3-Dimensional algorithm that only takes orthogonal steps.
//!
//! [`bresenham`]: fn.bresenham.html
//! [`bresenham-rs`]: https://crates.io/crates/bresenham
//! [mid-point line algorithm]: http://www.mat.univie.ac.at/~kriegl/Skripten/CG/node25.html
//! [Xiaolin Wu's line algorithm]: https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
//! [`WalkGrid`]: struct.WalkGrid.html
//! [`Supercover`]: struct.Supercover.html
//! [article]: http://www.redblobgames.com/grids/line-drawing.html
//! [`Bresenham3d`]: struct.Bresenham3d.html
//! [`WalkVoxels`]: struct.WalkVoxels.html

extern crate bresenham;

pub mod steps;
pub mod octant;

mod midpoint;
mod xiaolin_wu;
mod grid_walking;
mod fuzzing;
mod bresenham_3d;
mod walk_voxels;

pub use midpoint::*;
pub use xiaolin_wu::*;
pub use grid_walking::*;
pub use bresenham_3d::*;
pub use walk_voxels::*;

use std::collections::VecDeque;

/// A point in 2D space.
pub type Point<T> = (T, T);
/// An point in 3D space.
pub type Voxel<T> = (T, T, T);

// Sort two points and return whether they were reordered or not

#[inline]
fn sort_y<T: PartialOrd>(a: Point<T>, b: Point<T>) -> (Point<T>, Point<T>, bool) {
    if a.1 > b.1 {
        (b, a, true)
    } else {
        (a, b, false)
    }
}

#[inline]
fn sort_x<T: PartialOrd>(a: Point<T>, b: Point<T>) -> (Point<T>, Point<T>, bool) {
    if a.0 > b.0 {
        (b, a, true)
    } else {
        (a, b, false)
    }
}

#[inline]
fn collect_vec_deque<T, I>(iter: I, reordered: bool) -> VecDeque<T> where I: Iterator<Item = T> {
    let mut vec = VecDeque::new();

    for item in iter {
        if reordered {
            vec.push_front(item);
        } else {
            vec.push_back(item);
        }
    }

    vec
}

/// A simple wrapper around [`bresenham-rs`] that includes the end point.
///
/// If all you need is this function then just using [`bresenham-rs`] would probably be easier.
/// See [`sorted_bresenham`] for a sorted version.
///
/// Example: 
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::bresenham; 
///
/// fn main() {
///     for (x, y) in bresenham((0, 0), (5, 6)) {
///         print!("({}, {}), ", x, y);
///     }
/// }
/// ```
///
/// ```text
/// (0, 0), (0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6),
/// ```
///
/// [`bresenham-rs`]: https://crates.io/crates/bresenham
/// [`sorted_bresenham`]: fn.sorted_bresenham.html
pub fn bresenham(start: Point<isize>, end: Point<isize>) -> Vec<Point<isize>> {
    // Use the bresenham iterator to collect up the points
    let mut points: Vec<_> = bresenham::Bresenham::new(start, end).collect();
    // Add the last point
    points.push(end);
    points
}

/// Sorts the points before hand to ensure that the line is symmetrical and collects into a
/// [`VecDeque`].
/// [`VecDeque`]: https://doc.rust-lang.org/nightly/collections/vec_deque/struct.VecDeque.html
pub fn bresenham_sorted(start: Point<isize>, end: Point<isize>) -> VecDeque<Point<isize>> {
    let (start, end, reordered) = sort_y(start, end);
    let mut points = collect_vec_deque(bresenham::Bresenham::new(start, end), reordered);

    if reordered {
        points.push_front(end);
    } else {
        points.push_back(end);
    }

    points
}