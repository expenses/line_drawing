use {FloatNum, Point, SignedNum};
use steps::Steps;
use std::mem::swap;

/// An implementation of [Xiaolin Wu's line algorithm].
///
/// This algorithm works based on floating-points and returns an extra variable for how much a
/// a point is covered, which is useful for anti-aliasing.
///
/// Note that due to the implementation, the returned line will always go from left to right.
///
/// Example:
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::XiaolinWu;
///
/// fn main() {
///     for ((x, y), value) in XiaolinWu::<f32, i8>::new((0.0, 0.0), (3.0, 6.0)) {
///         print!("(({}, {}), {}), ", x, y, value);
///     }
/// }
/// ```
///
/// ```text
/// ((0, 0), 0.5), ((0, 1), 0.5), ((1, 1), 0.5), ((1, 2), 1), ((1, 3), 0.5), ((2, 3), 0.5), ((2, 4), 1), ((2, 5), 0.5), ((3, 5), 0.5), ((3, 6), 0.5),
/// ```
///
/// [Xiaolin Wu's line algorithm]: https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
pub struct XiaolinWu<I, O> {
    steep: bool,
    gradient: I,
    x: O,
    y: I,
    end_x: O,
    lower: bool,
}

impl<I: FloatNum, O: SignedNum> XiaolinWu<I, O> {
    #[inline]
    pub fn new(mut start: Point<I>, mut end: Point<I>) -> Self {
        let steep = (end.1 - start.1).abs() > (end.0 - start.0).abs();

        if steep {
            start = (start.1, start.0);
            end = (end.1, end.0);
        }

        if start.0 > end.0 {
            swap(&mut start, &mut end);
        }

        let mut gradient = (end.1 - start.1) / (end.0 - start.0);

        if gradient == I::zero() {
            gradient = I::one();
        }

        Self {
            steep,
            gradient,
            x: O::cast(start.0.round()),
            y: start.1,
            end_x: O::cast(end.0.round()),
            lower: false,
        }
    }

    #[inline]
    pub fn steps(self) -> Steps<(Point<O>, I), Self> {
        Steps::new(self)
    }
}

impl<I: FloatNum, O: SignedNum> Iterator for XiaolinWu<I, O> {
    type Item = (Point<O>, I);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x <= self.end_x {
            // get the fractional part of y
            let fpart = self.y - self.y.floor();

            // Calculate the integer value of y
            let mut y = O::cast(self.y);
            if self.lower {
                y += O::one();
            }

            // Get the point
            let point = if self.steep { (y, self.x) } else { (self.x, y) };

            if self.lower {
                // Return the lower point
                self.lower = false;
                self.x += O::one();
                self.y += self.gradient;
                Some((point, fpart))
            } else {
                if fpart > I::zero() {
                    // Set to return the lower point if the fractional part is > 0
                    self.lower = true;
                } else {
                    // Otherwise move on
                    self.x += O::one();
                    self.y += self.gradient;
                }

                // Return the remainer of the fractional part
                Some((point, I::one() - fpart))
            }
        } else {
            None
        }
    }
}

#[test]
fn tests() {
    let xiaolin_wu = |a, b| XiaolinWu::new(a, b).collect::<Vec<_>>();

    assert_eq!(
        xiaolin_wu((0.0, 0.0), (6.0, 3.0)),
        [
            ((0, 0), 1.0),
            ((1, 0), 0.5),
            ((1, 1), 0.5),
            ((2, 1), 1.0),
            ((3, 1), 0.5),
            ((3, 2), 0.5),
            ((4, 2), 1.0),
            ((5, 2), 0.5),
            ((5, 3), 0.5),
            ((6, 3), 1.0)
        ]
    );

    // The algorithm reorders the points to be left-to-right

    assert_eq!(
        xiaolin_wu((340.5, 290.77), (110.0, 170.0)),
        xiaolin_wu((110.0, 170.0), (340.5, 290.77))
    );
}
