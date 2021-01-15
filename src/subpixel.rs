use std::{cmp::{Ordering, min}, collections::HashMap, io::{Write, Result}};
use image::{DynamicImage, ImageBuffer, Rgb, imageops};
use imageops::FilterType;

const KERNEL_DIM: (u32, u32) = (4, 4);

pub fn convert(
    img: DynamicImage,
    size: (u32, u32),
    output: &mut impl Write,
    font: &[u8],
    invert: bool,
) -> Result<()> {
    let kernels = Asciifier::from_font(font);

    let luma_img = img.resize_exact(
        size.0 * KERNEL_DIM.0,
        size.1 * KERNEL_DIM.1,
        FilterType::Lanczos3,
    ).to_luma8();

    for y in 0..size.1 {
        let mut line = String::new();

        for x in 0..size.0 {
            let mut kernel = CharKernel::default();

            for sub_x in 0..KERNEL_DIM.0 {
                for sub_y in 0..KERNEL_DIM.1 {
                    let luma = luma_img.get_pixel(
                        x * KERNEL_DIM.0 + sub_x,
                        y * KERNEL_DIM.1 + sub_y
                    ).0[0] as f32 / std::u8::MAX as f32;

                    let luma = if invert { 1. - luma } else { luma };

                    kernel.0[sub_y as usize][sub_x as usize] = luma;
                }
            }

            let char_pixel = kernels.find_closest_char(&kernel);
            line.push(char_pixel);
        }

        writeln!(output, "{}", line.trim_end())?;
    }

    Ok(())
}

#[derive(Default)]
struct CharKernel([[f32; KERNEL_DIM.0 as usize]; KERNEL_DIM.1 as usize]);

impl std::ops::Sub<&CharKernel> for &CharKernel {
    type Output = f32;

    fn sub(self, other: &CharKernel) -> Self::Output {
        let mut diff = 0.;

        for x in 0..KERNEL_DIM.0 as usize {
            for y in 0..KERNEL_DIM.1 as usize {
                diff += (self.0[y][x] - other.0[y][x]).abs();
            }
        }

        diff
    }
}

struct Asciifier(HashMap<char, CharKernel>);

impl Asciifier {
    fn from_font(font: &[u8]) -> Self {
        use image::Pixel;

        let mut kernels = HashMap::new();

        // Limit generated kernels to printable ASCII chars
        for ord in 0x20..min(0x7E, font.len() / 8) {
            let chr = &font[(ord * 8)..(ord * 8 + 8)];

            let char_img = ImageBuffer::from_fn(8, 8, |x, y|
                if (chr[y as usize] << x) & 0x80 != 0 {
                    Rgb([0xFFu8, 0xFFu8, 0xFFu8])
                } else {
                    Rgb([0u8, 0u8, 0u8])
                }
            );

            let kernel_img = imageops::resize(
                &char_img,
                KERNEL_DIM.0,
                KERNEL_DIM.1,
                imageops::FilterType::Gaussian,
            );

            let mut kernel = CharKernel::default();

            for (x, y, p) in kernel_img.enumerate_pixels() {
                kernel.0[y as usize][x as usize] = p.to_luma().0[0] as f32 / 255.;
            }

            kernels.insert(ord as u8 as char, kernel);
        }

        Self(kernels)
    }

    fn find_closest_char(&self, k: &CharKernel) -> char {
        self.0.iter()
            .map(|(ch, k1)| (ch, k1 - k))
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(&i, _)| i)
            .unwrap_or(' ')
    }
}
