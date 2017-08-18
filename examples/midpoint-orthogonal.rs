extern crate line_drawing;
use line_drawing::midpoint; 

fn main() {
    for (x, y) in midpoint((0.2, 0.02), (2.8, 7.7), true) {
        print!("({}, {}), ", x, y);
    }
}