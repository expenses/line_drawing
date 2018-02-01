extern crate line_drawing;
use line_drawing::{VoxelOrigin, WalkVoxels};

fn main() {
    let a = (0.0, 0.0, 0.0);
    let b = (5.0, 6.0, 7.0);

    for (i, (x, y, z)) in WalkVoxels::<f32, i8>::new(a, b, &VoxelOrigin::Center).enumerate() {
        if i > 0 && i % 5 == 0 {
            println!();
        }
        print!("({}, {}, {}), ", x, y, z);
    }
}
