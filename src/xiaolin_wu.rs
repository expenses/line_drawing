use {Point, sort_x, reverse};

use std::mem::swap;

fn fpart(x: f32) -> f32 {
    x - x.floor()
}

fn rfpart(x: f32) -> f32 {
    1.0 - fpart(x)
}

type Points = Vec<(Point<isize>, f32)>;

// Add a point to the vec if it's value is over 0
fn add_point(points: &mut Points, point: Point<isize>, value: f32) {
    if value > 0.0 {
        points.push((point, value));
    }
}

// Add a set of points depending on whether the line is steep or not
fn add_points(points: &mut Points, steep: bool, x: isize, y: isize, v_a: f32, v_b: f32) {
    if steep {
        add_point(points, (y, x), v_a);
        add_point(points, (y + 1, x), v_b);
    } else {
        add_point(points, (x, y), v_a);
        add_point(points, (x, y + 1), v_b);
    }
}

/// An implementation of [Xiaolin Wu's line algorithm].
///
/// This algorithm works based on floating-points and returns an extra variable for how much a
/// a point is covered, which is useful for anti-aliasing.
/// 
/// Note that due to the implementation, the returned line will always go from left to right. See
/// [`sorted_xiaolin_wu`] for a version that reverses the resulting line in this case.
/// 
/// Example:
/// 
/// ```
/// extern crate line_drawing;
/// use line_drawing::xiaolin_wu; 
///
/// fn main() {
///     for ((x, y), value) in xiaolin_wu((0.0, 0.0), (3.0, 6.0)) {
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
/// [`sorted_xiaolin_wu`]: fn.sorted_xiaolin_wu.html
pub fn xiaolin_wu(mut start: Point<f32>, mut end: Point<f32>) -> Points {
    // Change the points around depending on whether the line is steep

    let steep = (end.1 - start.1).abs() > (end.0 - start.0).abs();

    if steep {
        start = (start.1, start.0);
        end = (end.1, end.0);
    }

    // Calculate whether to flip the points around

    if start.0 > end.0 {
        swap(&mut start, &mut end);
    }

    // Calculate the gradient

    let mut gradient = (end.1 - start.1) / (end.0 - start.0);
    
    if gradient == 0.0 {
        gradient = 1.0;
    }

    let (start_x, end_x) = (start.0.round(), end.0.round());
    let (start_x_i, end_x_i) = (start_x as isize, end_x as isize);

    let mut points = Vec::new();

    // Add the start point

    let start_y = start.1 + gradient * (start_x - start.0);
    let gap_x = rfpart(start.0 + 0.5);

    add_points(
        &mut points, steep,
        start_x_i, start_y.floor() as isize,
        rfpart(start_y) * gap_x, fpart(start_y) * gap_x
    );

    // Add all the middle points

    let mut y = start_y + gradient;

    for x in start_x_i + 1 .. end_x_i {
        add_points(
            &mut points, steep,
            x, y as isize,
            rfpart(y), fpart(y)
        );
        y += gradient;
    }

    // Add the end point

    let end_y = end.1 + gradient * (end_x - end.0);
    let gap_x = fpart(end.0 + 0.5);

    add_points(
        &mut points, steep,
        end_x_i, end_y.floor() as isize,
        rfpart(end_y) * gap_x, fpart(end_y) * gap_x
    );

    points
}

/// Like [`xiaolin_wu`] but reverses the resulting line if the start and end points get reordered.
/// [`xiaolin_wu`]: fn.xiaolin_wu.html
pub fn sorted_xiaolin_wu(start: Point<f32>, end: Point<f32>) -> Points {
    let (start, end, reordered) = sort_x(start, end);

    let points = xiaolin_wu(start, end);

    if !reordered {
        points
    } else {
        reverse(&points)
    }
}

#[test] 
fn test_xiaolin_wu() {
    assert_eq!(
        xiaolin_wu((0.0, 0.0), (6.0, 6.0)),
        [((0, 0), 0.5), ((1, 1), 1.0), ((2, 2), 1.0), ((3, 3), 1.0), ((4, 4), 1.0), ((5, 5), 1.0), ((6, 6), 0.5)]
    );

    // The algorithm reorders the points to be left-to-right

    assert_eq!(
        xiaolin_wu((340.5, 290.77), (110.0, 170.0)),
        xiaolin_wu((110.0, 170.0), (340.5, 290.77))
    );

    // sorted_xiaolin_wu should prevent this

    assert_eq!(
        sorted_xiaolin_wu((340.5, 290.77), (110.0, 170.0)),
        reverse(&sorted_xiaolin_wu((110.0, 170.0), (340.5, 290.77)))
    );
}