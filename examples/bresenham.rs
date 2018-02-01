extern crate line_drawing;
use line_drawing::Bresenham;

fn main() {
    for (x, y) in Bresenham::new((0, 0), (5, 6)) {
        print!("({}, {}), ", x, y);
    }
}
