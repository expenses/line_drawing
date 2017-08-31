use {Point, sort_x, collect_vec_deque};
use steps::Steps;
use std::mem::swap;
use std::collections::VecDeque;

/// An implementation of [Xiaolin Wu's line algorithm].
///
/// This algorithm works based on floating-points and returns an extra variable for how much a
/// a point is covered, which is useful for anti-aliasing.
/// 
/// Note that due to the implementation, the returned line will always go from left to right. Also
/// see [`xiaolin_wu`] and [`xiaolin_wu_sorted`] for a version that reverses the resulting line in
/// this case.
/// 
/// Example:
/// 
/// ```
/// extern crate line_drawing;
/// use line_drawing::XiaolinWu; 
///
/// fn main() {
///     for ((x, y), value) in XiaolinWu::new((0.0, 0.0), (3.0, 6.0)) {
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
/// [`xiaolin_wu`]: fn.xiaolin_wu.html
/// [`xiaolin_wu_sorted`]: fn.xiaolin_wu_sorted.html
pub struct XiaolinWu {
    steep: bool,
    gradient: f32,
    x: isize,
    y: f32,
    end_x: isize,
    lower: bool
}

impl XiaolinWu {
    #[inline]
    pub fn new(mut start: Point<f32>, mut end: Point<f32>) -> XiaolinWu {
        let steep = (end.1 - start.1).abs() > (end.0 - start.0).abs();

        if steep {
            start = (start.1, start.0);
            end = (end.1, end.0);
        }

        if start.0 > end.0 {
            swap(&mut start, &mut end);
        }

        let mut gradient = (end.1 - start.1) / (end.0 - start.0);
    
        if gradient == 0.0 {
            gradient = 1.0;
        }

        XiaolinWu {
            steep, gradient,
            x: start.0.round() as isize,
            y: start.1,
            end_x: end.0.round() as isize,
            lower: false
        }
    }

    #[inline]
    pub fn steps(self) -> Steps<(Point<isize>, f32), XiaolinWu> {
        Steps::new(self)
    }
}

impl Iterator for XiaolinWu {
    type Item = (Point<isize>, f32);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x <= self.end_x {
            // get the fractional part of y
            let fpart = self.y - self.y.floor();
            
            // Calculate the integer value of y
            let mut y = self.y as isize;
            if self.lower {
                y += 1;
            }

            // Get the point
            let point = if self.steep {
                (y, self.x)
            } else {
                (self.x, y)
            };

            if self.lower {
                // Return the lower point
                self.lower = false;
                self.x += 1;
                self.y += self.gradient;
                Some((point, fpart))
            } else {
                if fpart > 0.0 {
                    // Set to return the lower point if the fractional part is > 0
                    self.lower = true;
                } else {
                    // Otherwise move on
                    self.x += 1;
                    self.y += self.gradient;
                }

                // Return the remainer of the fractional part
                Some((point, 1.0 - fpart))
            }
        } else {
            None
        }
    }
}

/// A convenience function to collect the points from [`XiaolinWu`] into a [`Vec`].
/// [`XiaolinWu`]: struct.XiaolinWu.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
pub fn xiaolin_wu(start: Point<f32>, end: Point<f32>) -> Vec<(Point<isize>, f32)> {
    XiaolinWu::new(start, end).collect()
}

/// Sorts the points before hand to ensure that the line is symmetrical and collects into a
/// [`VecDeque`].
/// [`VecDeque`]: https://doc.rust-lang.org/nightly/collections/vec_deque/struct.VecDeque.html
pub fn xiaolin_wu_sorted(start: Point<f32>, end: Point<f32>) -> VecDeque<(Point<isize>, f32)> {
    let (start, end, reordered) = sort_x(start, end);
    collect_vec_deque(XiaolinWu::new(start, end), reordered)
}

#[test] 
fn tests() {
    assert_eq!(
        xiaolin_wu((0.0, 0.0), (6.0, 3.0)),
        [((0, 0), 1.0), ((1, 0), 0.5), ((1, 1), 0.5), ((2, 1), 1.0), ((3, 1), 0.5),
         ((3, 2), 0.5), ((4, 2), 1.0), ((5, 2), 0.5), ((5, 3), 0.5), ((6, 3), 1.0)]
    );

    // The algorithm reorders the points to be left-to-right

    assert_eq!(
        xiaolin_wu((340.5, 290.77), (110.0, 170.0)),
        xiaolin_wu((110.0, 170.0), (340.5, 290.77))
    );
}