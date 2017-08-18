extern crate line_drawing;
use line_drawing::bresenham; 

fn main() {
    for (x, y) in bresenham((0, 0), (5, 6)) {
        print!("({}, {}), ", x, y);
    }
}