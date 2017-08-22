extern crate line_drawing;
extern crate image;
extern crate bresenham;

use bresenham::Bresenham;
use line_drawing::*;
use image::{Rgb, DynamicImage, ImageBuffer};

type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

// Draw a line of pixels onto the image with a specific colour
fn draw_line<T>(image: &mut Image, line: T, colour: [u8; 3]) where T: Iterator<Item = Point<isize>> {
    for point in line {
        image.put_pixel(point.0 as u32, point.1 as u32, Rgb(colour));
    }
}

// Draw an anti-aliased line of pixels
fn draw_xiaolin_wu(image: &mut Image, line: XiaolinWu) {
    for (point, value) in line {
        image.put_pixel(point.0 as u32, point.1 as u32, Rgb([(255.0 * value).round() as u8; 3]));
    }
}

fn main() {
    let mut image = DynamicImage::new_rgb8(300, 300).to_rgb();

    // Draw each of the different line types
    draw_line(&mut image, WalkGrid::new((10, 230), (50, 290)), [255, 0, 0]);
    draw_line(&mut image, Supercover::new((10, 210), (90, 290)), [255, 128, 0]);
    draw_line(&mut image, Midpoint::new((10.0, 187.5), (122.22, 290.0)), [128, 255, 0]);
    draw_line(&mut image, Bresenham::new((10, 165), (170, 290)), [0, 255, 0]);

    // Draw two lines on top of each other to show how bresenham isn't symetrical
    let a = (10, 10);
    let b = (200, 290);
    draw_line(&mut image, Bresenham::new(a, b), [255, 0, 0]);
    draw_line(&mut image, Bresenham::new(b, a), [0, 128, 255]);

    // Draw a triangle made out of xiaolin wi lines
    let a = (275.0, 150.0);
    let b = (210.0, 285.0);
    let c = (290.0, 290.0);
    draw_xiaolin_wu(&mut image, XiaolinWu::new(a, b));
    draw_xiaolin_wu(&mut image, XiaolinWu::new(b, c));
    draw_xiaolin_wu(&mut image, XiaolinWu::new(c, a));

    // Save the image
    image.save("example.png").unwrap();
}