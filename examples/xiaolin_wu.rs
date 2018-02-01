extern crate line_drawing;
use line_drawing::XiaolinWu;

fn main() {
    for ((x, y), value) in XiaolinWu::<f32, i8>::new((0.0, 0.0), (3.0, 6.0)) {
        print!("(({}, {}), {}), ", x, y, value);
    }
}
