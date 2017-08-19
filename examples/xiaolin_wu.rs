extern crate line_drawing;
use line_drawing::xiaolin_wu; 

fn main() {
    for ((x, y), value) in xiaolin_wu((0.0, 0.0), (3.0, 6.0)) {
        print!("(({}, {}), {}), ", x, y, value);
    }
}