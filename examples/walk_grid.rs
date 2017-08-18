extern crate line_drawing;
use line_drawing::walk_grid;

fn main() {
    for (x, y) in walk_grid((0, 0), (5, 3)) {
        print!("({}, {}), ", x, y);
    }
}