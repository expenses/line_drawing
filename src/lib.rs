//! A collection of line-drawing algorithms for use in graphics and video games.
//!
//! Currently implemented:
//!
//! * [`bresenham`] through [`bresenham-rs`].
//! * [`walk_grid`] and [`supercover`] implemented from [this article by Red Blob Games][article].
//!
//! [`bresenham`]: fn.bresenham.html
//! [`bresenham-rs`]: https://crates.io/crates/bresenham
//! [`walk_grid`]: fn.walk_grid.html
//! [`supercover`]: fn.supercover.html
//! [article]: http://www.redblobgames.com/grids/line-drawing.html

extern crate bresenham;

type Point = (isize, isize);

// Sort two points and return whether they were reordered or not
fn sort(a: Point, b: Point) -> (Point, Point, bool) {
    if a.1 > b.1 {
        (b, a, true)
    } else {
        (a, b, false)
    }
}

// Reverse an slice of points into a vec
fn reverse(points: &[Point]) -> Vec<Point> {
    points.iter().rev().cloned().collect()
}

/// Walk along a grid, taking only orthagonal steps.
///
/// See [this section][section] of the [article] for an interactive demonstration.
/// 
/// Note that this algorithm isn't symetrical; if you swap `start` and `end`, the reversed line
/// might not be the same. See [`sorted_walk_grid`] for a version that sorts the points so that
/// the line will be the same.
///
/// Example: 
///
/// ```rust
/// extern crate line_drawing;
/// use line_drawing::walk_grid; 
///
/// fn main() {
///     for (x, y) in walk_grid((0, 0), (5, 3)) {
///         println!("{}, {}", x, y);
///     }
/// }
/// ```
/// Should print out:
///
/// ```text
/// 0, 0
/// 1, 0
/// 1, 1
/// 2, 1
/// 2, 2
/// 3, 2
/// 4, 2
/// 4, 3
/// 5, 3
/// ```
/// [section]: http://www.redblobgames.com/grids/line-drawing.html#org3c085ed
/// [article]: http://www.redblobgames.com/grids/line-drawing.html
/// [`sorted_walk_grid`]: fn.sorted_walk_grid.html
pub fn walk_grid(mut start: Point, end: Point) -> Vec<Point> {
    // Set up the points
    let mut points = Vec::new();
    points.push(start);
    
    // Delta values between the points
    let (dx, dy) = (end.0 - start.0, end.1 - start.1);
    // Number of steps in each direction
    let (nx, ny) = (dx.abs() as f32, dy.abs() as f32);
    // Which way the steps are going
    let sign_x = if dx > 0 {1} else {-1};
    let sign_y = if dy > 0 {1} else {-1};

    // How many steps have been taken in either direction
    let (mut ix, mut iy) = (0.0, 0.0);

    // While there are steps to take
    while ix < nx || iy < ny {
        // Determine which direction to step in
        if (0.5 + ix) / nx < (0.5 + iy) / ny {
            start.0 += sign_x;
            ix += 1.0;
        } else {
            start.1 += sign_y;
            iy += 1.0;
        }
        // Add the point
        points.push(start);
    }

    points
}

/// Like [`walk_grid`] but takes diagonal steps if the line passes directly over a corner.
///
/// See [this section][section] of the [article] for an interactive demonstration.
/// 
/// This algorithm should always be symetrical.
///
/// Example: 
///
/// ```rust
/// extern crate line_drawing;
/// use line_drawing::supercover; 
///
/// fn main() {
///     for (x, y) in supercover((0, 0), (5, 5)) {
///         println!("{}, {}", x, y);
///     }
/// }
/// ```
/// Should print out:
///
/// ```text
/// 0, 0
/// 1, 1
/// 2, 2
/// 3, 3
/// 4, 4
/// 5, 5
/// ```
/// [`walk_grid`]: fn.walk_grid.html
/// [section]: http://www.redblobgames.com/grids/line-drawing.html#org1da485d
/// [article]: http://www.redblobgames.com/grids/line-drawing.html
pub fn supercover(mut start: Point, end: Point) -> Vec<Point> {
    let mut points = Vec::new();
    points.push(start);
    
    let (dx, dy) = (end.0 - start.0, end.1 - start.1);
    let (nx, ny) = (dx.abs() as f32, dy.abs() as f32);
    let sign_x = if dx > 0 {1} else {-1};
    let sign_y = if dy > 0 {1} else {-1};

    let (mut ix, mut iy) = (0.0, 0.0);

    while ix < nx || iy < ny {
        let comparison = ((0.5 + ix) / nx) - ((0.5 + iy) / ny);

        // If the comparison is equal then jump diagonally
        if comparison == 0.0 {
            start.0 += sign_x;
            start.1 += sign_y;
            ix += 1.0;
            iy += 1.0;
        } else if comparison < 0.0 {
            start.0 += sign_x;
            ix += 1.0;
        } else {
            start.1 += sign_y;
            iy += 1.0;
        }
        
        points.push(start);
    }

    points
}

/// A simple wrapper around [`bresenham-rs`] that includes the end point.
///
/// If all you need is this function then just using [`bresenham-rs`] would probably be easier.
///
/// Example: 
///
/// ```rust
/// extern crate line_drawing;
/// use line_drawing::bresenham; 
///
/// fn main() {
///     for (x, y) in bresenham((0, 0), (5, 6)) {
///         println!("{}, {}", x, y);
///     }
/// }
/// ```
/// Should print out:
///
/// ```text
/// 0, 0
/// 0, 1
/// 1, 2
/// 2, 3
/// 3, 4
/// 4, 5
/// 5, 6
/// ```
/// [`bresenham-rs`]: https://crates.io/crates/bresenham
pub fn bresenham(start: Point, end: Point) -> Vec<Point> {
    // Use the bresenham iterator to collect up the points
    let mut points: Vec<_> = bresenham::Bresenham::new(start, end).collect();
    // Add the last point
    points.push(end);
    points
}

/// A sorted version of [`walk_grid`].
/// 
/// Sorts the points to ensure that if the start and end points were swapped the line would be the
/// same.
/// [`walk_grid`]: fn.walk_grid.html
pub fn sorted_walk_grid(start: Point, end: Point) -> Vec<Point> {
    let (start, end, reordered) = sort(start, end);
    let points = walk_grid(start, end);

    if !reordered {
        points
    } else {
        reverse(&points)
    }
}

/// A sorted version of [`bresenham`].
/// [`bresenham`]: fn.bresenham.html
pub fn sorted_bresenham(start: Point, end: Point) -> Vec<Point> {
    let (start, end, reordered) = sort(start, end);
    let points = bresenham(start, end);

    if !reordered {
        points
    } else {
        reverse(&points)
    }
}

#[test]
fn walk_grid_tests() {
    assert_eq!(
        walk_grid((0, 0), (2, 2)),
        [(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)]
    );

    assert_eq!(
        walk_grid((0, 0), (3, 2)),
        [(0, 0), (1, 0), (1, 1), (2, 1), (2, 2), (3, 2)]
    );

    // by default, walk grid is asymmetrical
    assert_ne!(walk_grid((0, 0), (2, 2)), reverse(&walk_grid((2, 2), (0, 0))));

    // sorted walk grid should be symetrical
    assert_eq!(sorted_walk_grid((0, 0), (20, 20)), reverse(&sorted_walk_grid((20, 20), (0, 0))));
}

#[test]
fn supercover_tests() {
    // supercover should jump diagonally if the difference is equal

    assert_eq!(
        supercover((0, 0), (5, 5)),
        [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]
    );

    assert_eq!(
        supercover((0, 0), (3, 1)),
        [(0, 0), (1, 0), (2, 1), (3, 1)]
    );

    assert_ne!(walk_grid((0, 0), (-10, 10)), supercover((0, 0), (-10, 10)));
    assert_ne!(supercover((20, 10), (10, 20)), walk_grid((20, 10), (10, 20)));

    // otherwise it should do the same as walk grid    
    assert_eq!(supercover((0, 0), (4, 5)), walk_grid((0, 0), (4, 5)));

    // supercover should be symetrical
    assert_eq!(supercover((0, 0), (2, 3)), reverse(&supercover((2, 3), (0, 0))));
    assert_eq!(supercover((0, 0), (5, 5)), reverse(&supercover((5, 5), (0, 0))));
    assert_eq!(supercover((0, 0), (19, 13)), reverse(&supercover((19, 13), (0, 0))));
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