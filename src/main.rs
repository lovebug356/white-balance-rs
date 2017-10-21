extern crate image;
extern crate clap;
extern crate white_balance;

use std::fs::File;
use std::path::Path;

use white_balance::traits::AutoWhiteBalance;

fn main() {
    let matches = clap::App::new("white-balance")
        .version("0.1.0")
        .author("Thijs Vermeir <thijsvermeir@gmail.com>")
        .about("Automatic white balance for images")
        .arg(clap::Arg::with_name("input")
            .help("input image filename")
            .short("i")
            .long("input")
            .takes_value(true)
            .required(true)
        )
        .arg(clap::Arg::with_name("output")
            .help("output image filename")
            .short("o")
            .long("output")
            .takes_value(true)
            .required(true)
        ).get_matches ();

    match matches.subcommand_name() {
        Some("gray-world") => {
            let matches = matches.subcommand_matches("gray-world").unwrap();
            let input_filename = matches.value_of("input").unwrap();
            let output_filename = matches.value_of("output").unwrap();

            println!("Gray world conversion:");
            println!("\tInput: {}", input_filename);
            println!("\tOutput: {}", output_filename);

            let original_image = image::open(&input_filename)
                .unwrap();
            let orig_rgb = original_image.to_rgb();
            let enhanced_image = white_balance::GrayWorld::white_balance(&orig_rgb);

            let fout = &mut File::create(&Path::new(output_filename)).unwrap();
            image::ImageRgb8(enhanced_image).save(fout, image::PNG)
                .unwrap();
        }
        _ => {
            eprintln!("Subcommand not recognized.")
        }
    }
}
