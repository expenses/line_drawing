extern crate line_drawing;
use line_drawing::WalkGrid;

fn main() {
    for (x, y) in WalkGrid::new((0, 0), (5, 3)) {
        print!("({}, {}), ", x, y);
    }
}