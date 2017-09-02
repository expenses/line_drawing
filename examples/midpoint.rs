extern crate line_drawing;
use line_drawing::Midpoint; 

fn main() {
    for (x, y) in Midpoint::<_, i8>::new((0.2, 0.02), (2.8, 7.7)) {
        print!("({}, {}), ", x, y);
    }
}