use {Point, sort_y, reverse};

/// Walk along a grid, taking only orthogonal steps.
///
/// See [this section] of the [article] for an interactive demonstration.
/// 
/// Note that this algorithm isn't symetrical; if you swap `start` and `end`, the reversed line
/// might not be the same. See [`walk_grid`] and [`walk_grid_sorted`] for a sorted version.
///
/// Example: 
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::WalkGrid;
///
/// fn main() {
///     for (x, y) in WalkGrid::new((0, 0), (5, 3)) {
///         print!("({}, {}), ", x, y);
///     }
/// }
/// ```
///
/// ```text
/// (0, 0), (1, 0), (1, 1), (2, 1), (2, 2), (3, 2), (4, 2), (4, 3), (5, 3),
/// ```
///
/// [this section]: http://www.redblobgames.com/grids/line-drawing.html#org3c085ed
/// [article]: http://www.redblobgames.com/grids/line-drawing.html
/// [`walk_grid`]: fn.walk_grid.html
/// [`walk_grid_sorted`]: fn.walk_grid_sorted.html
pub struct WalkGrid {
    point: Point<isize>,
    ix: f32,
    iy: f32,
    sign_x: isize,
    sign_y: isize,
    ny: f32,
    nx: f32,
}

impl WalkGrid {
    #[inline]
    pub fn new(start: Point<isize>, end: Point<isize>) -> WalkGrid {
        // Delta values between the points
        let (dx, dy) = (end.0 - start.0, end.1 - start.1);

        WalkGrid {
            point: start,
            ix: 0.0,
            iy: 0.0,
            sign_x: dx.signum(),
            sign_y: dx.signum(),
            nx: dx.abs() as f32,
            ny: dy.abs() as f32
        }
    }
}

impl Iterator for WalkGrid {
    type Item = Point<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.ix <= self.nx && self.iy <= self.ny {
            let point = self.point;

            if (0.5 + self.ix) / self.nx < (0.5 + self.iy) / self.ny {
                self.point.0 += self.sign_x;
                self.ix += 1.0;
            } else {
                self.point.1 += self.sign_y;
                self.iy += 1.0;
            }  

            Some(point)
        } else {
            None
        }
    }
}

/// A convenience function to collect the points from [`WalkGrid`] into a [`Vec`].
/// [`WalkGrid`]: struct.WalkGrid.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
pub fn walk_grid(start: Point<isize>, end: Point<isize>) -> Vec<Point<isize>> {
    WalkGrid::new(start, end).collect()
}

/// Like [`walk_grid`] but sorts the points before hand to ensure that the line is symmetrical.
/// [`walk_grid`]: fn.walk_grid.html
pub fn walk_grid_sorted(start: Point<isize>, end: Point<isize>) -> Vec<Point<isize>> {
    let (start, end, reordered) = sort_y(start, end);
    let points = walk_grid(start, end);

    if !reordered {
        points
    } else {
        reverse(&points)
    }
}

/// Like [`WalkGrid`] but takes diagonal steps if the line passes directly over a corner.
///
/// See [this section][section] of the [article] for an interactive demonstration.
/// 
/// This algorithm should always be symetrical.
///
/// Example: 
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::Supercover; 
///
/// fn main() {
///     for (x, y) in Supercover::new((0, 0), (5, 5)) {
///         print!("({}, {}), ", x, y);
///     }
/// }
/// ```
///
/// ```text
/// (0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5),
/// ```
///
/// [`WalkGrid`]: struct.WalkGrid.html
/// [section]: http://www.redblobgames.com/grids/line-drawing.html#org1da485d
/// [article]: http://www.redblobgames.com/grids/line-drawing.html
pub struct Supercover {
    point: Point<isize>,
    ix: f32,
    iy: f32,
    sign_x: isize,
    sign_y: isize,
    ny: f32,
    nx: f32
}

impl Supercover {
    #[inline]
    pub fn new(start: Point<isize>, end: Point<isize>) -> Supercover {
        // Delta values between the points
        let (dx, dy) = (end.0 - start.0, end.1 - start.1);

        Supercover {
            point: start,
            ix: 0.0,
            iy: 0.0,
            sign_x: dx.signum(),
            sign_y: dy.signum(),
            nx: dx.abs() as f32,
            ny: dy.abs() as f32
        }
    }
}

impl Iterator for Supercover {
    type Item = Point<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.ix <= self.nx && self.iy <= self.ny {
            let point = self.point;

            let comparison = ((0.5 + self.ix) / self.nx) - ((0.5 + self.iy) / self.ny);

            // If the comparison is equal then jump diagonally
            if comparison == 0.0 {
                self.point.0 += self.sign_x;
                self.point.1 += self.sign_y;
                self.ix += 1.0;
                self.iy += 1.0;
            } else if comparison < 0.0 {
                self.point.0 += self.sign_x;
                self.ix += 1.0;
            } else {
                self.point.1 += self.sign_y;
                self.iy += 1.0;
            }

            Some(point)
        } else {
            None
        }
    }
}

/// A convenience function to collect the points from [`Supercover`] into a [`Vec`].
/// [`Supercover`]: struct.Supercover.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
pub fn supercover(start: Point<isize>, end: Point<isize>) -> Vec<Point<isize>> {
    Supercover::new(start, end).collect()
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
    assert_eq!(walk_grid_sorted((0, 0), (20, 20)), reverse(&walk_grid_sorted((20, 20), (0, 0))));
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
    assert_ne!(walk_grid((20, 10), (10, 20)), supercover((20, 10), (10, 20)));

    // otherwise it should do the same as walk grid    
    assert_eq!(supercover((0, 0), (4, 5)), walk_grid((0, 0), (4, 5)));
}
