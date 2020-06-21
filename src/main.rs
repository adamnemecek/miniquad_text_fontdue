use std::fs::File;
use std::io::Write;

// Scratch pad for glyphs: ⅞ g
const CHARACTER: char = 'g';
const SIZE: f32 = 50.0;

pub fn main() {
    // Loading and rasterization
    let font = include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
    let (metrics, bitmap) = font.rasterize(CHARACTER, SIZE);
    // font.ras

    // Output
    let mut o = File::create("fontdue.pgm").unwrap();
    let _ = o.write(format!("P5\n{} {}\n255\n", metrics.width, metrics.height).as_bytes());
    let _ = o.write(&bitmap);
}