extern crate line_drawing;
use line_drawing::Supercover; 

fn main() {
    for (x, y) in Supercover::new((0, 0), (5, 5)) {
        print!("({}, {}), ", x, y);
    }
}