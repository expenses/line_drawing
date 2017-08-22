use {reverse, sort_y, Point, Octant};

/// An implementation of the [mid-point line drawing algorithm].
///
/// The biggest difference between this algorithm and [`bresenham`] is that it uses floating-point points. Also see
/// [`midpoint`] and [`sorted_midpoint`] for a sorted version.
///
/// Example without orthogonal steps:
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
/// [`sorted_midpoint`]: fn.sorted_midpoint.html
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

/// Like [`midpoint`] but sorts the points before hand to ensure that the line is symmetrical.
/// [`midpoint`]: fn.midpoint.html
pub fn midpoint_sorted(start: Point<f32>, end: Point<f32>) -> Vec<Point<isize>> {
    let (start, end, reordered) = sort_y(start, end);
    
    let points = midpoint(start, end);

    if !reordered {
        points
    } else {
        reverse(&points)
    }
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