//! A collection of line-drawing algorithms for use in graphics and video games.
//!
//! Currently implemented:
//!
//! * [`bresenham`] through [`bresenham-rs`].
//! * The [mid-point line algorithm].
//! * [Xiaolin Wu's line algorithm].
//! * [`WalkGrid`] and [`Supercover`] implemented from [this article by Red Blob Games][article].
//!
//! [`bresenham`]: fn.bresenham.html
//! [`bresenham-rs`]: https://crates.io/crates/bresenham
//! [mid-point line algorithm]: http://www.mat.univie.ac.at/~kriegl/Skripten/CG/node25.html
//! [Xiaolin Wu's line algorithm]: https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
//! [`WalkGrid`]: struct.WalkGrid.html
//! [`Supercover`]: struct.Supercover.html
//! [article]: http://www.redblobgames.com/grids/line-drawing.html

extern crate bresenham;

mod midpoint;
mod xiaolin_wu;
mod grid_walking;

pub use midpoint::{Midpoint, midpoint, sorted_midpoint};
pub use xiaolin_wu::{xiaolin_wu, sorted_xiaolin_wu};
pub use grid_walking::{WalkGrid, Supercover, walk_grid, supercover, sorted_walk_grid};

type Point<T> = (T, T);

// Sort two points and return whether they were reordered or not

fn sort_y<T: PartialOrd>(a: Point<T>, b: Point<T>) -> (Point<T>, Point<T>, bool) {
    if a.1 > b.1 {
        (b, a, true)
    } else {
        (a, b, false)
    }
}

fn sort_x<T: PartialOrd>(a: Point<T>, b: Point<T>) -> (Point<T>, Point<T>, bool) {
    if a.0 > b.0 {
        (b, a, true)
    } else {
        (a, b, false)
    }
}

// Reverse an slice of points into a vec
fn reverse<T: Clone>(points: &[T]) -> Vec<T> {
    points.iter().rev().cloned().collect()
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

/// Like [`bresenham`] but sorts the points before hand to ensure that the line is symmetrical.
/// [`bresenham`]: fn.bresenham.html
pub fn sorted_bresenham(start: Point<isize>, end: Point<isize>) -> Vec<Point<isize>> {
    let (start, end, reordered) = sort_y(start, end);
    let points = bresenham(start, end);

    if !reordered {
        points
    } else {
        reverse(&points)
    }
}

#[test]
fn bresenham_tests() {
    // Bresenham is not symetrical by default
    assert_ne!(bresenham((0, 0), (5, 3)), reverse(&bresenham((5, 3), (0, 0))));

    // But should be if using the sorted version
    assert_eq!(
        sorted_bresenham((0, 0), (5, 3)),
        reverse(&sorted_bresenham((5, 3), (0, 0)))
    );
}