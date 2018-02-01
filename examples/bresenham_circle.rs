extern crate line_drawing;
use line_drawing::BresenhamCircle;

fn main() {
    for (x, y) in BresenhamCircle::new(0, 0, 1) {
        print!("({}, {}), ", x, y);
    }
}
