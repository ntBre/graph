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

fn main() {
    let w = BufWriter::new(File::create("try.png").unwrap());

    const HEIGHT: u32 = 100;
    const WIDTH: u32 = 100;

    let encoder = Encoder::new(w, WIDTH, HEIGHT);
    let mut writer = encoder.write_header().unwrap();

    let mut data = [0; (4 * WIDTH * HEIGHT) as usize];
    for chunk in data.chunks_mut(4) {
        chunk.copy_from_slice(&[255, 0, 255, 255]);
    }
    writer.write_image_data(&data).unwrap();
}
