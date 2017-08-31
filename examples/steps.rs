extern crate line_drawing;
use line_drawing::WalkGrid;

fn main() {
    for (start, end) in WalkGrid::new((0, 0), (5, 3)).steps() {
        println!("{:?} -> {:?}", start, end);
    }
}