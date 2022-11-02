use image::{Image, Rgba};

fn main() {
    const HEIGHT: u32 = 100;
    const WIDTH: u32 = 100;

    let mut img = Image::new(WIDTH, HEIGHT);
    img.fill(Rgba::black());
    img.set(50, 10, Rgba::red());
    img.set(50, 50, Rgba::green());
    img.set(50, 90, Rgba::blue());

    img.write("try.png").unwrap();
}
