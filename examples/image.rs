extern crate line_drawing;
extern crate image;

use line_drawing::{walk_grid, supercover, bresenham, midpoint, xiaolin_wu};
use image::{Rgb, DynamicImage, ImageBuffer};

type Point<T> = (T, T);
type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

// Draw a line of pixels onto the image with a specific colour
fn draw_line(image: &mut Image, line: Vec<Point<isize>>, colour: [u8; 3]) {
    for point in line {
        image.put_pixel(point.0 as u32, point.1 as u32, Rgb(colour));
    }
}

// Draw an anti-aliased line of pixels
fn draw_xiaolin_wu(image: &mut Image, line: Vec<(Point<isize>, f32)>) {
    for (point, value) in line {
        image.put_pixel(point.0 as u32, point.1 as u32, Rgb([(255.0 * value).round() as u8; 3]));
    }
}

fn main() {
    let mut image = DynamicImage::new_rgb8(300, 300).to_rgb();

    // Draw each of the different line types
    draw_line(&mut image, walk_grid((10, 10), (40, 290)), [255, 0, 0]);
    draw_line(&mut image, supercover((30, 10), (120, 100)), [255, 128, 0]);
    draw_line(&mut image, bresenham((50, 50), (150, 290)), [128, 255, 0]);
    draw_line(&mut image, midpoint((110.0, 10.0), (170.0, 290.0), false), [0, 255, 0]);
    draw_line(&mut image, midpoint((130.0, 10.0), (190.0, 290.0), true), [0, 255, 128]);

    // Draw two lines on top of each other to show how bresenham isn't symetrical
    draw_line(&mut image, bresenham((290, 10), (210, 290)), [255, 0, 0]);
    draw_line(&mut image, bresenham((210, 290), (290, 10)), [0, 128, 255]);

    // Draw a triangle made out of xiaolin wi lines

    let a = (150.0, 10.0);
    let b = (270.0, 15.0);
    let c = (200.0, 200.0);

    draw_xiaolin_wu(&mut image, xiaolin_wu(a, b));
    draw_xiaolin_wu(&mut image, xiaolin_wu(b, c));
    draw_xiaolin_wu(&mut image, xiaolin_wu(c, a));

    // Save the image
    image.save("example.png").unwrap();
}