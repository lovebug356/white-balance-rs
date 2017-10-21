extern crate image;
extern crate clap;
extern crate white_balance;

use std::fs::File;
use std::path::Path;

use white_balance::{AutoWhiteBalance, AutoWhiteBalanceMethod};

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
            .required(false)
        )
        .arg(clap::Arg::with_name("auto")
            .help("white balancing auto")
            .short("m")
            .long("auto")
            .takes_value(true)
            .required(false)
        )
        .arg(clap::Arg::with_name("all-methods")
            .help("use all methods")
            .short("a")
            .long("all")
            .takes_value(false)
            .required(false)
        )
        .get_matches ();

    let input_filename = matches.value_of("input").unwrap();
    if !filename_has_extension(input_filename) {
        eprintln!("Filename does not have an extension.");
        return;
    }

    let method = match matches.value_of("auto") {
        Some(method_str) => {
            match method_str {
                "gray-world" => Some(AutoWhiteBalanceMethod::GrayWorld),
                "retinex" => Some(AutoWhiteBalanceMethod::Retinex),
                "gray-retinex" => Some(AutoWhiteBalanceMethod::GrayRetinex),
                _ => {
                    eprintln!("Auto white balancing auto '{}' not found", method_str);
                    return;
                }
            }
        },
        None => {
            Some(AutoWhiteBalanceMethod::GrayWorld)
        }
    }.unwrap();

    let input_image = image::open(&input_filename).unwrap();
    let rgb_image = input_image.to_rgb();
    let (width, height) = rgb_image.dimensions();

    println!("Auto white balancing:");
    println!("\tInput: {} ({}x{})", input_filename, width, height);

    let output_filename: String = build_output_filename(input_filename,
                                                        matches.value_of("output"),
                                                        &method);
    println!("\tOutput: {} -> {}", method, output_filename);
    let enhanced_image = rgb_image.auto_white_balance(method);
    let fout = &mut File::create(&Path::new(&output_filename)).unwrap();
    image::ImageRgb8(enhanced_image).save(fout, image::PNG).unwrap();
}

fn filename_has_extension(filename: &str) -> bool {
    let split: Vec<&str> = filename.rsplitn(2, ".").collect();
    split.len() == 2
}

fn build_output_filename(input_filename: &str,
                         output_filename: Option<&str>,
                         method: &AutoWhiteBalanceMethod) -> String {
    match output_filename {
        Some(filename) => String::from(filename),
        None => {
            let string_split: Vec<&str> = input_filename.rsplitn(2,".").collect();
            format!("{}-{}.{}", string_split[1], method.to_string(), string_split[0])
        }
    }
}