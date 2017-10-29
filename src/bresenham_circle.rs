use {Point, SignedNum};

/// An implementation of [Bresenham's circle algorithm].
///
/// This uses four quadrants, so calling `next()` will return a point for the first quadrant,
/// then the second, third, fourth and then back to first.
///
/// Example:
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::BresenhamCircle; 
///
/// fn main() {
///     for (x, y) in BresenhamCircle::new(0, 0, 1) {
///         print!("({}, {}), ", x, y);
///     }
/// }
/// ```
///
/// ```text
/// (1, 0), (0, 1), (-1, 0), (0, -1),
/// ```
///
/// [Bresenham's circle algorithm]: http://members.chello.at/~easyfilter/bresenham.html
pub struct BresenhamCircle<T> {
    x: T,
    y: T,
    center_x: T,
    center_y: T,
    radius: T,
    error: T,
    quadrant: u8
}

impl<T: SignedNum> BresenhamCircle<T> {
    #[inline]
    pub fn new(center_x: T, center_y: T, radius: T) -> Self {
        Self {
            center_x, center_y, radius,
            x: -radius,
            y: T::zero(),
            error: T::cast(2) - T::cast(2) * radius,
            quadrant: 1
        }
    }
}

impl<T: SignedNum> Iterator for BresenhamCircle<T> {
    type Item = Point<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < T::zero() {
            let point = match self.quadrant {
                1 => (self.center_x - self.x, self.center_y + self.y),
                2 => (self.center_x - self.y, self.center_y - self.x),
                3 => (self.center_x + self.x, self.center_y - self.y),
                4 => (self.center_x + self.y, self.center_y + self.x),
                _ => unreachable!()
            };

            // Update the variables after each set of quadrants
            if self.quadrant == 4 {
                self.radius = self.error;

                if self.radius <= self.y {
                    self.y += T::one();
                    self.error += self.y * T::cast(2) + T::one();
                }

                if self.radius > self.x || self.error > self.y {
                    self.x += T::one();
                    self.error += self.x * T::cast(2) + T::one();
                }
            }

            self.quadrant = self.quadrant % 4 + 1;

            Some(point)
        } else {
            None
        }
    }
}
