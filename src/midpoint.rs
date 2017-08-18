use {reverse, sort, Point};

// Get the relevant octant from the start and end points
fn octant(start: Point<f32>, end: Point<f32>) -> u8 {
    let mut dx = end.0 - start.0;
    let mut dy = end.1 - start.1;

    let mut octant = 0;

    if dy < 0.0 {
        dx = -dx;
        dy = -dy;
        octant += 4;
    }

    if dx < 0.0 {
        let tmp = dx;
        dx = dy;
        dy = -tmp;
        octant += 2
    }

    if dx < dy {
        octant += 1
    }

    octant
}

// Convert a point to its position in the octant
fn to_octant(octant: u8, point: Point<f32>) -> Point<f32> {
    match octant {
        0 => ( point.0,  point.1),
        1 => ( point.1,  point.0),
        2 => ( point.1, -point.0),
        3 => (-point.0,  point.1),
        4 => (-point.0, -point.1),
        5 => (-point.1, -point.0),
        6 => (-point.1,  point.0),
        7 => ( point.0, -point.1),
        _ => unreachable!()
    }
}

// Convert a point from its position in the octant
fn from_octant(octant: u8, point: Point<isize>) -> Point<isize> {
    match octant {
        0 => ( point.0,  point.1),
        1 => ( point.1,  point.0),
        2 => (-point.1,  point.0),
        3 => (-point.0,  point.1),
        4 => (-point.0, -point.1),
        5 => (-point.1, -point.0),
        6 => ( point.1, -point.0),
        7 => ( point.0, -point.1),
        _ => unreachable!()
    }
}

/// An implementation of the [mid-point line drawing algorithm].
///
/// The biggest difference between this algorithm and [`walk_grid`][`walk_grid`] (with orthogonal stepping)
/// and [`bresenham`][`bresenham`] (without) is that it uses floating-point points. See
/// [`sorted_midpoint`] for a sorted version.
///
/// Example without orthogonal steps:
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::midpoint; 
///
/// fn main() {
///     for (x, y) in midpoint((0.2, 0.02), (2.8, 7.7), false) {
///         print!("({}, {}), ", x, y);
///     }
/// }
/// ```
/// 
/// ```text
/// (0, 0), (1, 1), (1, 2), (1, 3), (2, 4), (2, 5), (2, 6), (3, 7), (3, 8),
/// ```
///
/// Example with orthogonal steps:
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::midpoint; 
///
/// fn main() {
///     for (x, y) in midpoint((0.2, 0.02), (2.8, 7.7), true) {
///         print!("({}, {}), ", x, y);
///     }
/// }
/// ```
///
/// ```text
/// (0, 0), (1, 0), (1, 1), (1, 2), (1, 3), (2, 3), (2, 4), (2, 5), (2, 6), (3, 6), (3, 7), (3, 8),
/// ```
///
/// [mid-point line drawing algorithm]: http://www.mat.univie.ac.at/~kriegl/Skripten/CG/node25.html
/// [`walk_grid`]: fn.walk_grid.html
/// [`bresenham`]: fn.bresenham.html
/// [`sorted_midpoint`]: fn.sorted_midpoint.html
pub fn midpoint(start: Point<f32>, end: Point<f32>, orthogonal: bool) -> Vec<Point<isize>> {
    // Get the octant to use
    let octant = octant(start, end);

    // Convert the points into the octant versions
    let start = to_octant(octant, start);
    let end = to_octant(octant, end);

    // Initialise the variables

    let a = -(end.1 - start.1);
    let b = end.0 - start.0;
    let c = start.0 * end.1 - end.0 * start.1;

    let mut x = start.0.round() as isize;
    let mut y = ((-a * x as f32 - c) / b).round() as isize;

    let mut k = a * (x as f32 + 1.0) + b * (y as f32  + 0.5) + c;

    // Set up the points
    let mut points = Vec::new();
    points.push(from_octant(octant, (x, y)));

    while x < end.0.round() as isize {
        // Taken an E step
        if k > 0.0 {
            k += a;
        // Take a NE step
        } else {
            k += a + b;
            y += 1;
            
            // If orthagonal mode is on, add the point
            if orthogonal {
                points.push(from_octant(octant, (x, y)));
            }
        }

        x += 1;

        points.push(from_octant(octant, (x, y)));
    }

    points
}

/// A sorted version of [`midpoint`].
/// [`midpoint`]: fn.midpoint.html
pub fn sorted_midpoint(start: Point<f32>, end: Point<f32>, orthogonal: bool) -> Vec<Point<isize>> {
    let (start, end, reordered) = sort(start, end);
    let points = midpoint(start, end, orthogonal);

    if !reordered {
        points
    } else {
        reverse(&points)
    }
}

#[test]
fn midpoint_tests() {
    assert_eq!(
        midpoint((0.0, 0.0), (-5.0, -5.0), false),
        [(0, 0), (-1, -1), (-2, -2), (-3, -3), (-4, -4), (-5, -5)]
    );

    assert_eq!(
        midpoint((0.0, 0.0), (-5.0, -5.0), true),
        [(0, 0), (0, -1), (-1, -1), (-1, -2), (-2, -2), (-2, -3),
        (-3, -3), (-3, -4), (-4, -4), (-4, -5), (-5, -5)]
    );

    // The midpoint algorithm is not normally symetrical

    assert_ne!(
        midpoint((0.0, 0.0), (-5.0, 10.0), false),
        reverse(&midpoint((-5.0, 10.0), (0.0, 0.0), false))
    );

    assert_ne!(
        midpoint((0.0, 0.0), (-5.0, 10.0), true),
        reverse(&midpoint((-5.0, 10.0), (0.0, 0.0), true))
    );

    // sorted_midpoint should be symetrical

    assert_eq!(
        sorted_midpoint((0.0, 0.0), (-5.0, 10.0), false),
        reverse(&sorted_midpoint((-5.0, 10.0), (0.0, 0.0), false))
    );

    assert_eq!(
        sorted_midpoint((0.0, 0.0), (-5.0, 10.0), true),
        reverse(&sorted_midpoint((-5.0, 10.0), (0.0, 0.0), true))
    );
}