use std::fs::File;
use std::io::BufWriter;
use image::{Encoder, Image, Rgba};

fn main() {
    let w = BufWriter::new(File::create("try.png").unwrap());

    const HEIGHT: u32 = 100;
    const WIDTH: u32 = 100;

    let encoder = Encoder::new(w, WIDTH, HEIGHT);
    let writer = encoder.write_header().unwrap();

    let mut img = Image::new(WIDTH, HEIGHT);
    img.fill(Rgba::black());
    img.set(50, 10, Rgba::red());
    img.set(50, 50, Rgba::green());
    img.set(50, 90, Rgba::blue());

    img.write(writer).unwrap();
}
