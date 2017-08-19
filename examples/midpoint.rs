extern crate line_drawing;
use line_drawing::Midpoint; 

fn main() {
    for (x, y) in Midpoint::new((0.2, 0.02), (2.8, 7.7), false) {
        print!("({}, {}), ", x, y);
    }
}