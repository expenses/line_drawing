use {Point, SignedNum, sort_y, collect_vec_deque};
use octant::Octant;
use steps::Steps;
use std::collections::VecDeque;

/// An implementation of [Bresenham's line algorithm].
///
/// Includes both the start and end point and is asymmetrical.
///
/// Example:
///
/// ```rust
/// extern crate line_drawing;
/// use line_drawing::Bresenham; 
///
/// fn main() {
///     for (x, y) in Bresenham::new((0, 0), (5, 6)) {
///         print!("({}, {}), ", x, y);
///     }
/// }
/// ```
///
/// ```text
/// (0, 0), (0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6),
/// ```
///
/// [Bresenham's line algorithm]: https://en.wikipedia.org/wiki/Bresenham's_line_algorithm
pub struct Bresenham<T> {
    point: Point<T>,
    end_x: T,
    delta_x: T,
    delta_y: T,
    error: T,
    octant: Octant
}

impl<T: SignedNum> Bresenham<T> {
    #[inline]
    pub fn new(start: Point<T>, end: Point<T>) -> Bresenham<T> {
        let octant = Octant::new(start, end);
        let start = octant.to(start);
        let end = octant.to(end);

        let delta_x = end.0 - start.0;
        let delta_y = end.1 - start.1;

        Bresenham {
            delta_x, delta_y, octant,
            point: start,
            end_x: end.0,
            error: delta_y - delta_x,
        }
    }

    #[inline]
    pub fn steps(self) -> Steps<Point<T>, Bresenham<T>> {
        Steps::new(self)
    }
}

impl<T: SignedNum> Iterator for Bresenham<T> {
    type Item = Point<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.point.0 <= self.end_x {
            let point = self.octant.from(self.point);

            if self.error >= T::zero() {
                self.point.1 += T::one();
                self.error -= self.delta_x;
            }

            self.point.0 += T::one();
            self.error += self.delta_y;

            Some(point)
        } else {
            None
        }
    }
}

/// A convenience function to collect the points from [`Bresenham`] into a [`Vec`].
/// [`Bresenham`]: struct.Bresenham.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
#[inline]
pub fn bresenham<T: SignedNum>(start: Point<T>, end: Point<T>) -> Vec<Point<T>> {
    Bresenham::new(start, end).collect()
}

/// Sorts the points before hand to ensure that the line is symmetrical and collects into a
/// [`VecDeque`].
/// [`VecDeque`]: https://doc.rust-lang.org/nightly/collections/vec_deque/struct.VecDeque.html
#[inline]
pub fn bresenham_sorted<T: SignedNum>(start: Point<T>, end: Point<T>) -> VecDeque<Point<T>> {
    let (start, end, reordered) = sort_y(start, end);
    collect_vec_deque(Bresenham::new(start, end), reordered)
}

#[test]
fn test() {
    assert_eq!(
        bresenham((0, 0), (5, 5)),
        [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]
    )
}