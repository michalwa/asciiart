use std::io::{Write, Result};
use image::{DynamicImage, imageops::FilterType};

pub struct Charset<'a>(&'a str);

impl Charset<'_> {
    pub const DEFAULT: Self = Self("  .`,-=+:;#%&@");

    fn get_char_by_lightness(&self, lightness: f32) -> char {
        let lightness_index = (lightness * (self.0.len() as f32 - 1.)) as usize;
        self.0.chars().skip(lightness_index).next().unwrap()
    }
}

pub fn convert(
    img: DynamicImage,
    size: (u32, u32),
    output: &mut impl Write,
    charset: Charset<'_>,
    invert: bool,
) -> Result<()> {
    use image::Pixel;

    let img = img.resize(size.0, size.1, FilterType::Gaussian).to_rgb8();

    for row in img.rows() {
        let mut line = String::new();

        for &pixel in row {
            let lightness = pixel.to_luma().0[0] as f32 / std::u8::MAX as f32;
            let lightness = if invert { 1. - lightness } else { lightness };
            line.push(charset.get_char_by_lightness(lightness));
        }

        writeln!(output, "{}", line.trim_end())?;
    }

    Ok(())
}
