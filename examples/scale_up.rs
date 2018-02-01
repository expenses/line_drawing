extern crate image;
extern crate line_drawing;

use line_drawing::*;
use image::*;

const BLACK: [u8; 4] = [255, 255, 255, 255];
const GREY: [u8; 4] = [128, 128, 128, 255];
const SIZE: u32 = 15;
const SCALE: u32 = 15;

const SCALE_I: i32 = SCALE as i32;
const SCALE_HALF: i32 = SCALE_I / 2;

// Draw a line of pixels onto the image with a specific colour
fn draw_line<T>(image: &mut DynamicImage, line: T, colour: [u8; 4])
where
    T: Iterator<Item = Point<i32>>,
{
    for point in line {
        image.put_pixel(point.0 as u32, point.1 as u32, Rgba(colour));
    }
}

fn main() {
    let mut image = DynamicImage::new_rgb8(SIZE, SIZE);
    let a = (0, 0);
    let b = (6, 14);

    draw_line(&mut image, WalkGrid::new(a, b), BLACK);

    let mut image = image.resize(SIZE * SCALE, SIZE * SCALE, FilterType::Nearest);

    let a = (a.0 * SCALE_I + SCALE_HALF, a.1 * SCALE_I + SCALE_HALF);
    let b = (b.0 * SCALE_I + SCALE_HALF, b.1 * SCALE_I + SCALE_HALF);

    draw_line(&mut image, Bresenham::new(a, b), GREY);

    image.to_rgb().save("test.png").unwrap();
}
