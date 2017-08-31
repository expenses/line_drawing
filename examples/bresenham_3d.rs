extern crate line_drawing;
use line_drawing::Bresenham3d; 

fn main() {
    for (x, y, z) in Bresenham3d::new((0, 0, 0), (5, 6, 7)) {
        print!("({}, {}, {}), ", x, y, z);
    }
}