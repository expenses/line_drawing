use {sort_y, Point, collect_vec_deque};
use octant::Octant;
use steps::Steps;
use std::collections::VecDeque;

/// An implementation of the [mid-point line drawing algorithm].
///
/// The biggest difference between this algorithm and [`bresenham`] is that it uses floating-point points. Also see
/// [`midpoint`] and [`midpoint_sorted`] for a sorted version.
///
/// Example:
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::Midpoint; 
///
/// fn main() {
///     for (x, y) in Midpoint::new((0.2, 0.02), (2.8, 7.7)) {
///         print!("({}, {}), ", x, y);
///     }
/// }
/// ```
/// 
/// ```text
/// (0, 0), (1, 1), (1, 2), (1, 3), (2, 4), (2, 5), (2, 6), (3, 7), (3, 8),
/// ```
///
/// [mid-point line drawing algorithm]: http://www.mat.univie.ac.at/~kriegl/Skripten/CG/node25.html
/// [`bresenham`]: fn.bresenham.html
/// [`midpoint`]: fn.midpoint.html
/// [`midpoint_sorted`]: fn.midpoint_sorted.html
pub struct Midpoint {
    octant: Octant,
    point: Point<isize>,
    a: f32,
    b: f32,
    k: f32,
    end_x: isize
}

impl Midpoint {
    #[inline]
    pub fn new(start: Point<f32>, end: Point<f32>) -> Midpoint {
        // Get the octant to use
        let octant = Octant::new(start, end);

        // Convert the points into the octant versions
        let start = octant.to(start);
        let end = octant.to(end);

        // Initialise the variables

        let a = -(end.1 - start.1);
        let b = end.0 - start.0;
        let c = start.0 * end.1 - end.0 * start.1;

        Midpoint {
            octant, a, b,
            point: (start.0.round() as isize, start.1.round() as isize),
            k: a * (start.0.round() + 1.0) + b * (start.1.round()  + 0.5) + c,
            end_x: end.0.round() as isize
        }
    }

    #[inline]
    pub fn steps(self) -> Steps<Point<isize>, Midpoint> {
        Steps::new(self)
    }    
}

impl Iterator for Midpoint {
    type Item = Point<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.point.0 <= self.end_x {
            let point = self.octant.from(self.point);

            // Take an N step
            if self.k <= 0.0 {
                self.k += self.b;
                self.point.1 += 1;
            }

            // Take an E step
            self.k += self.a;
            self.point.0 += 1;

            Some(point)
        } else {
            None
        }
    }
}

/// A convenience function to collect the points from [`Midpoint`] into a [`Vec`].
/// [`Midpoint`]: struct.Midpoint.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
pub fn midpoint(start: Point<f32>, end: Point<f32>) -> Vec<Point<isize>> {
    Midpoint::new(start, end).collect()
}

/// Sorts the points before hand to ensure that the line is symmetrical and collects into a
/// [`VecDeque`].
/// [`VecDeque`]: https://doc.rust-lang.org/nightly/collections/vec_deque/struct.VecDeque.html
pub fn midpoint_sorted(start: Point<f32>, end: Point<f32>) -> VecDeque<Point<isize>> {
    let (start, end, reordered) = sort_y(start, end);
    collect_vec_deque(Midpoint::new(start, end), reordered)
}

#[test]
fn tests() {
    assert_eq!(
        midpoint((0.0, 0.0), (-5.0, -5.0)),
        [(0, 0), (-1, -1), (-2, -2), (-3, -3), (-4, -4), (-5, -5)]
    );

    assert_eq!(
        midpoint((0.0, 0.0), (6.0, 3.0)),
        [(0, 0), (1, 1), (2, 1), (3, 2), (4, 2), (5, 3), (6, 3)]
    );
}