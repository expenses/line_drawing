//! An example of generating basic shapes
extern crate image;
extern crate num_complex;
extern crate line_drawing;

fn draw_circle(imgbuf: &mut image::RgbaImage, xc: i32, yc: i32, r: i32) {
	for (x, y) in line_drawing::BresenhamCircle::new(xc, yc, r) {
		imgbuf.put_pixel(x as u32, y as u32, image::Rgba([255, 255, 0, 255]));
    }
}

fn draw_lines(imgbuf: &mut image::RgbaImage, lines: &[((u32, u32), (u32, u32))], anti_alias: bool) {
	for (p_1, p_2) in lines.iter() {
		let p_1 = (p_1.0 as f32, p_1.1 as f32);
		let p_2 = (p_2.0 as f32, p_2.1 as f32);
		for ((x, y), alpha) in line_drawing::XiaolinWu::<f32, i32>::new(p_1, p_2) {
			let pixel_color = image::Rgba([255, 0, 0, if anti_alias {(alpha*255.0) as u8} else {255} ]) ;
			imgbuf.put_pixel(x as u32, y as u32, pixel_color); 
		}
	}
}

fn draw_triangle(imgbuf: &mut image::RgbaImage, points: [(u32, u32); 3], anti_alias: bool) {
	let lines = [
		(points[0], points[1]),
		(points[0], points[2]),
		(points[1], points[2])
	];
	return draw_lines(imgbuf, &lines, anti_alias);
}

fn draw_rectangle(imgbuf: &mut image::RgbaImage, top_left: (u32, u32), bottom_right: (u32, u32)) {
	let top_right = (bottom_right.0, top_left.1);
	let bottom_left = (top_left.0, bottom_right.1);
	let lines = [
		(top_left, top_right),
		(top_right, bottom_right),
		(bottom_right, bottom_left),
		(bottom_left, top_left)
	];
	return draw_lines(imgbuf, &lines, false);
}

fn main() {
	let mut imgbuf: image::RgbaImage = image::ImageBuffer::new(800, 800);

	// triangle
	let tri_points = [
		(400,100),
		(600,300),
		(200,300)
	];
	draw_triangle(&mut imgbuf, tri_points, true);
	draw_rectangle(&mut imgbuf, (200, 300), (600, 700));
	draw_circle(&mut imgbuf, 80, 80, 50);
	
	imgbuf.save("basic_shapes.png").unwrap();
}