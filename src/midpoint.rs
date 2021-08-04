use crate::octant::Octant;
use crate::steps::Steps;
use crate::{FloatNum, Point, SignedNum};

/// An implementation of the [mid-point line drawing algorithm].
///
/// The biggest difference between this algorithm and [`Bresenham`] is that it uses floating-point points.
///
/// Example:
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::Midpoint;
///
/// fn main() {
///     for (x, y) in Midpoint::<f32, i8>::new((0.2, 0.02), (2.8, 7.7)) {
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
/// [`Bresenham`]: struct.bresenham.html
pub struct Midpoint<I, O> {
    octant: Octant,
    point: Point<O>,
    a: I,
    b: I,
    k: I,
    end_x: O,
}

impl<I: FloatNum, O: SignedNum> Midpoint<I, O> {
    #[inline]
    pub fn new(start: Point<I>, end: Point<I>) -> Self {
        // Get the octant to use
        let octant = Octant::new(start, end);

        // Convert the points into the octant versions
        let start = octant.to(start);
        let end = octant.to(end);

        // Initialise the variables

        let a = -(end.1 - start.1);
        let b = end.0 - start.0;
        let c = start.0 * end.1 - end.0 * start.1;

        Self {
            octant,
            a,
            b,
            point: (O::cast(start.0.round()), O::cast(start.1.round())),
            k: a * (start.0.round() + I::one()) + b * (start.1.round() + I::cast(0.5)) + c,
            end_x: O::cast(end.0.round()),
        }
    }

    #[inline]
    pub fn steps(self) -> Steps<Point<O>, Self> {
        Steps::new(self)
    }
}

impl<I: FloatNum, O: SignedNum> Iterator for Midpoint<I, O> {
    type Item = Point<O>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.point.0 <= self.end_x {
            let point = self.octant.from(self.point);

            // Take an N step
            if self.k <= I::zero() {
                self.k += self.b;
                self.point.1 += O::one();
            }

            // Take an E step
            self.k += self.a;
            self.point.0 += O::one();

            Some(point)
        } else {
            None
        }
    }
}

#[test]
fn tests() {
    let midpoint = |a, b| Midpoint::new(a, b).collect::<Vec<_>>();

    assert_eq!(
        midpoint((0.0, 0.0), (-5.0, -5.0)),
        [(0, 0), (-1, -1), (-2, -2), (-3, -3), (-4, -4), (-5, -5)]
    );

    assert_eq!(
        midpoint((0.0, 0.0), (6.0, 3.0)),
        [(0, 0), (1, 1), (2, 1), (3, 2), (4, 2), (5, 3), (6, 3)]
    );
}
