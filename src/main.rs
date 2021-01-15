mod console_8x8;

use std::{env, io, error::Error};

const USAGE: &str = "Usage: asciiart <image> <width> <height>";

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();

    let img_path = args.next().expect(USAGE);
    let width = args.next().expect(USAGE).parse()?;
    let height = args.next().expect(USAGE).parse()?;

    let img = image::open(img_path)
        .expect("Image not found");

    asciiart::subpixel::convert(
        img, (width, height),
        &mut io::stdout(),
        console_8x8::BIT_FONT, true
    )?;

    Ok(())
}
