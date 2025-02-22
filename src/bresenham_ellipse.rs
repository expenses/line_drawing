// Created by Simon Varey: github.com/simonvarey. Released under the MIT license.

use {Point, SignedNum};
use steps::Steps;

/// An implementation of [Bresenham's ellipse algorithm].
///
/// This uses four quadrants, so calling `next()` will return a point for the first quadrant,
/// then the second, third, fourth and then back to first.
///
/// Example:
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::BresenhamEllipse;
///
/// fn main() {
///     for (x, y) in BresenhamEllipse::new(5, 5, 2, 3) {
///         print!("({}, {}), ", x, y);
///     }
/// }
/// ```
///
/// ```text
/// (7, 5), (3, 5), (3, 5), (7, 5), (7, 6), (3, 6), (3, 4), (7, 4), (6, 7), (4, 7), (4, 3), (6, 3), (5, 8), (5, 8), (5, 2), (5, 2), (6, 8), (4, 8), (4, 2), (6, 2),
/// ```
///
/// [Bresenham's ellipse algorithm]: https://dai.fmph.uniba.sk/upload/0/01/Ellipse.pdf
pub struct BresenhamEllipse<T> {
    x: T,
    y: T,
    center_x: T,
    center_y: T,
    radius_x: T,
    radius_y: T,
    delta_x: T,
    delta_y: T,
    error: T,
    quadrant: u8,
    radius_x_squared_doubled: T,
    radius_y_squared_doubled: T,
    end_x: T,
    end_y: T,
    first_region: bool
}

impl<T: SignedNum> BresenhamEllipse<T> {
    #[inline]
    pub fn new(center_x: T, center_y: T, radius_x: T, radius_y: T) -> Self {
        // Variables initialized for first region, where tangent line slope greater than -1
        let radius_y_squared_doubled = T::cast(2) * radius_y * radius_y;

        Self {
            center_x,
            center_y,
            radius_x,
            radius_y,
            x: radius_x,
            y: T::zero(),
            delta_x: radius_y * radius_y * (T::one() - T::cast(2) * radius_x),
            delta_y: radius_x * radius_x,
            error: T::zero(),
            radius_x_squared_doubled: T::cast(2) * radius_x * radius_x,
            radius_y_squared_doubled,
            end_x: radius_y_squared_doubled * radius_x,
            end_y: T::zero(),
            quadrant: 1,
            first_region: true
        }
    }

    #[inline]
    pub fn steps(self) -> Steps<Point<T>, Self> {
        Steps::new(self)
    }
}

impl<T: SignedNum> Iterator for BresenhamEllipse<T> {
    type Item = Point<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {

        // First region: tangent line slope greater than -1
        if self.first_region && self.end_x >= self.end_y {
            let point = match self.quadrant {
                1 => (self.center_x + self.x, self.center_y + self.y),
                2 => (self.center_x - self.x, self.center_y + self.y),
                3 => (self.center_x - self.x, self.center_y - self.y),
                4 => (self.center_x + self.x, self.center_y - self.y),
                _ => unreachable!(),
            };

            // Update the variables after each set of quadrants
            if self.quadrant == 4 {
                self.y += T::one();
                self.end_y += self.radius_x_squared_doubled;
                self.error += self.delta_y;
                self.delta_y += self.radius_x_squared_doubled;

                if self.error * T::cast(2) + self.delta_x > T::zero() {
                    self.x -= T::one();
                    self.end_x -= self.radius_y_squared_doubled;
                    self.error += self.delta_x;
                    self.delta_x += self.radius_y_squared_doubled;
                }
            }

            self.quadrant = self.quadrant % 4 + 1;

            Some(point)
        } else if self.end_x <= self.end_y {
            // Update variables a single time for second region, where tangent line slope less than -1
            if self.first_region {
                self.first_region = false;
    
                self.x = T::zero();
                self.y = self.radius_y;
                self.delta_x = self.radius_y * self.radius_y;
                self.delta_y = self.radius_x * self.radius_x * (T::one() - T::cast(2) * self.radius_y);
                self.error = T::zero();
                self.end_x = T::zero();
                self.end_y = self.radius_x_squared_doubled * self.radius_y;
            }

            // Second region: tangent line slope less than -1
            let point = match self.quadrant {
                1 => (self.center_x + self.x, self.center_y + self.y),
                2 => (self.center_x - self.x, self.center_y + self.y),
                3 => (self.center_x - self.x, self.center_y - self.y),
                4 => (self.center_x + self.x, self.center_y - self.y),
                _ => unreachable!(),
            };

            // Update the variables after each set of quadrants
            if self.quadrant == 4 {
                self.x += T::one();
                self.end_x += self.radius_y_squared_doubled;
                self.error += self.delta_x;
                self.delta_x += self.radius_y_squared_doubled;

                if self.error * T::cast(2) + self.delta_y > T::zero() {
                    self.y -= T::one();
                    self.end_y -= self.radius_x_squared_doubled;
                    self.error += self.delta_y;
                    self.delta_y += self.radius_x_squared_doubled;
                }
            }

            self.quadrant = self.quadrant % 4 + 1;

            Some(point)
        } else {
            None
        }
    }
}

#[test]
fn tests() {
    let ellipse = |a, b, c, d| 
        BresenhamEllipse::new(a, b, c, d).collect::<Vec<_>>();

    assert_eq!(
        ellipse(50, 50, 10, 15),
        [(60, 50), (40, 50), (40, 50), (60, 50), (60, 51), (40, 51), (40, 49), (60, 49), (60, 52), 
         (40, 52), (40, 48), (60, 48), (60, 53), (40, 53), (40, 47), (60, 47), (60, 54), (40, 54), 
         (40, 46), (60, 46), (59, 55), (41, 55), (41, 45), (59, 45), (59, 56), (41, 56), (41, 44), 
         (59, 44), (59, 57), (41, 57), (41, 43), (59, 43), (58, 58), (42, 58), (42, 42), (58, 42), 
         (58, 59), (42, 59), (42, 41), (58, 41), (57, 60), (43, 60), (43, 40), (57, 40), (57, 61), 
         (43, 61), (43, 39), (57, 39), (56, 62), (44, 62), (44, 38), (56, 38), (50, 65), (50, 65), 
         (50, 35), (50, 35), (51, 65), (49, 65), (49, 35), (51, 35), (52, 65), (48, 65), (48, 35), 
         (52, 35), (53, 64), (47, 64), (47, 36), (53, 36), (54, 64), (46, 64), (46, 36), (54, 36), 
         (55, 63), (45, 63), (45, 37), (55, 37)]
    );
}
