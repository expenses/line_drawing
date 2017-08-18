extern crate line_drawing;
use line_drawing::supercover; 

fn main() {
    for (x, y) in supercover((0, 0), (5, 5)) {
        print!("({}, {}), ", x, y);
    }
}