use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

#[derive(Clone, Copy, Debug)]
struct Color {
	r: u8,
	g: u8,
	b: u8
}

#[derive(Clone, )]
struct Image {
	data: Vec<u8>,
	x: usize,
	y: usize
}

impl Color {
	fn new(r: u8, g: u8, b: u8) -> Color {
		Color { r: r, g: g, b: b}
	}
}

impl Image {
	fn new(x: usize, y: usize) -> Image {
		let mut result: Image = Image { data: vec![], x, y };
		result.data.resize(x*y * 3, 0);
		result
	}

	fn set_pixel(&mut self, x: usize, y: usize, clr: &Color) {
		let offset = (y * self.x + x) * 3;
		self.data[offset + 0] = clr.r;
		self.data[offset + 1] = clr.g;
		self.data[offset + 2] = clr.b;
	}

	fn get_data(&self) -> &[u8] {
		&self.data[..]
	}

	fn save_to_file(&self, filename: String) {
		let path = Path::new(&filename);
		let file = File::create(path).unwrap();
		let ref mut w = BufWriter::new(file);

		let mut encoder = png::Encoder::new(w, self.x as u32, self.y as u32);
		encoder.set_color(png::ColorType::RGB);
		encoder.set_depth(png::BitDepth::Eight);
		let mut writer = encoder.write_header().unwrap();

		writer.write_image_data(self.get_data()).unwrap();
	}
}

fn main() {
	let mut img = Image::new(100, 100);

	let white = Color::new(255, 255, 255);
	for x in 10..50 {
		for y in 10..20 {
			img.set_pixel(x, y, &white);
		}
	}

	img.save_to_file(String::from("image.png"));
}
