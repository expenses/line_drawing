extern crate line_drawing;
use line_drawing::WalkVoxels; 

fn main() {
    for (i, (x, y, z)) in WalkVoxels::new((0.0, 0.0, 0.0), (5.0, 6.0, 7.0)).enumerate() {
        if i > 0 && i % 5 == 0 {
            println!();
        }
        print!("({}, {}, {}), ", x, y, z);
    }
}