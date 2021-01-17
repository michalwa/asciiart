mod console_8x8;

use std::{error::Error, io, path::PathBuf};
use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1")]
struct Opts {
    mode: String,
    image_path: PathBuf,
    width: u32,
    height: u32,

    #[clap(short, long)]
    invert: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();

    let img = image::open(&opts.image_path)
        .expect("Image not found");

    match &opts.mode[..] {
        "simple" => {
            asciiart::simple::convert(
                img,
                (opts.width, opts.height),
                &mut io::stdout(),
                asciiart::simple::Charset::DEFAULT,
                opts.invert,
            )?;
        }
        "subpixel" => {
            asciiart::subpixel::convert(
                img,
                (opts.width, opts.height),
                &mut io::stdout(),
                console_8x8::BIT_FONT,
                opts.invert,
            )?;
        }
        _ => panic!("Unknown mode: {}", opts.mode),
    }

    Ok(())
}
