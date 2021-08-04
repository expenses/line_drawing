use crate::steps::Steps;
use crate::{Point, SignedNum};

/// Walk along a grid, taking only orthogonal steps.
///
/// See [this section] of the [article] for an interactive demonstration.
///
/// Note that this algorithm isn't symetrical; if you swap `start` and `end`, the reversed line
/// might not be the same.
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
pub struct WalkGrid<T> {
    point: Point<T>,
    ix: f32,
    iy: f32,
    sign_x: T,
    sign_y: T,
    ny: f32,
    nx: f32,
}

impl<T: SignedNum> WalkGrid<T> {
    #[inline]
    pub fn new(start: Point<T>, end: Point<T>) -> WalkGrid<T> {
        // Delta values between the points
        let (dx, dy) = (end.0 - start.0, end.1 - start.1);

        WalkGrid {
            point: start,
            ix: 0.0,
            iy: 0.0,
            sign_x: dx.signum(),
            sign_y: dy.signum(),
            nx: dx.abs().to_f32().unwrap(),
            ny: dy.abs().to_f32().unwrap(),
        }
    }

    #[inline]
    pub fn steps(self) -> Steps<Point<T>, WalkGrid<T>> {
        Steps::new(self)
    }
}

impl<T: SignedNum> Iterator for WalkGrid<T> {
    type Item = Point<T>;

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
pub struct Supercover<T> {
    point: Point<T>,
    ix: f32,
    iy: f32,
    sign_x: T,
    sign_y: T,
    ny: f32,
    nx: f32,
}

impl<T: SignedNum> Supercover<T> {
    #[inline]
    pub fn new(start: Point<T>, end: Point<T>) -> Self {
        // Delta values between the points
        let (dx, dy) = (end.0 - start.0, end.1 - start.1);

        Self {
            point: start,
            ix: 0.0,
            iy: 0.0,
            sign_x: dx.signum(),
            sign_y: dy.signum(),
            nx: dx.abs().to_f32().unwrap(),
            ny: dy.abs().to_f32().unwrap(),
        }
    }

    #[inline]
    pub fn steps(self) -> Steps<Point<T>, Self> {
        Steps::new(self)
    }
}

impl<T: SignedNum> Iterator for Supercover<T> {
    type Item = Point<T>;

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

#[test]
fn walk_grid_tests() {
    use crate::fuzzing::reverse_slice;
    let walk_grid = |a, b| WalkGrid::new(a, b).collect::<Vec<_>>();

    assert_eq!(
        walk_grid((0, 0), (2, 2)),
        [(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)]
    );

    assert_eq!(
        walk_grid((0, 0), (3, 2)),
        [(0, 0), (1, 0), (1, 1), (2, 1), (2, 2), (3, 2)]
    );

    assert_eq!(
        walk_grid((0, 0), (0, 5)),
        [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)]
    );

    assert_eq!(
        walk_grid((0, 0), (5, 0)),
        [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]
    );

    // by default, walk grid is asymmetrical
    assert_ne!(
        walk_grid((0, 0), (2, 2)),
        reverse_slice(&walk_grid((2, 2), (0, 0)))
    );
}

#[test]
fn supercover_tests() {
    let walk_grid = |a, b| WalkGrid::new(a, b).collect::<Vec<_>>();
    let supercover = |a, b| Supercover::new(a, b).collect::<Vec<_>>();

    // supercover should jump diagonally if the difference is equal

    assert_eq!(
        supercover((0, 0), (5, 5)),
        [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]
    );

    assert_eq!(supercover((0, 0), (3, 1)), [(0, 0), (1, 0), (2, 1), (3, 1)]);

    assert_eq!(
        supercover((0, 0), (0, 5)),
        [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)]
    );

    assert_eq!(
        supercover((0, 0), (5, 0)),
        [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]
    );

    assert_ne!(walk_grid((0, 0), (-10, 10)), supercover((0, 0), (-10, 10)));
    assert_ne!(
        walk_grid((20, 10), (10, 20)),
        supercover((20, 10), (10, 20))
    );

    // otherwise it should do the same as walk grid
    assert_eq!(supercover((0, 0), (4, 5)), walk_grid((0, 0), (4, 5)));
}
