use std::fs::File;
use std::io::{BufWriter, Write};

/// newtype wrapping `png::Encoder` and simplifying the API with more default
/// settings
struct Encoder<'a, W: Write>(png::Encoder<'a, W>);

impl<'a, W: Write> Encoder<'a, W> {
    fn new(w: W, width: u32, height: u32) -> Self {
        let mut e = png::Encoder::new(w, width, height);
        e.set_color(png::ColorType::Rgba);
        Self(e)
    }

    pub(crate) fn write_header(
        self,
    ) -> Result<png::Writer<W>, png::EncodingError> {
        self.0.write_header()
    }
}

struct Rgba(u8, u8, u8, u8);

impl Rgba {
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(r, g, b, a)
    }

    fn red() -> Self {
        Self::new(255, 0, 0, 255)
    }

    fn green() -> Self {
        Self::new(0, 255, 0, 255)
    }

    fn blue() -> Self {
        Self::new(0, 0, 255, 255)
    }

    fn black() -> Self {
        Self::new(0, 0, 0, 255)
    }

    fn as_array(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

struct Image {
    width: u32,
    #[allow(unused)]
    height: u32,
    data: Vec<u8>,
}

impl Image {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0; (4 * height * width) as usize],
        }
    }

    /// fill `self` with `color`
    fn fill(&mut self, color: Rgba) {
        for chunk in self.data.chunks_mut(4) {
            chunk.copy_from_slice(&color.as_array());
        }
    }

    /// set the pixel at row `x` and col `y` to `color`
    fn set(&mut self, x: usize, y: usize, color: Rgba) {
        let v = color.as_array();
        let row = 4 * x * self.width as usize;
        let col = 4 * y;
        self.data[row + col..row + col + 4].copy_from_slice(&v);
    }

    pub(crate) fn write(
        &self,
        mut writer: png::Writer<BufWriter<File>>,
    ) -> Result<(), png::EncodingError> {
        writer.write_image_data(&self.data)
    }
}

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
