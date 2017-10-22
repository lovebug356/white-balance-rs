extern crate image;
extern crate clap;
extern crate white_balance;

use std::fs::File;
use std::path::Path;

use white_balance::{AutoWhiteBalance, AutoWhiteBalanceMethod};
use white_balance::image_ext::imageformat::image_format_from_string;

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
        .arg(clap::Arg::with_name("all")
            .help("use all methods")
            .short("a")
            .long("all")
            .required(false)
            .conflicts_with_all(&["auto", "output"])
        )
        .get_matches ();

    let input_filename = matches.value_of("input").unwrap();
    if !filename_has_extension(input_filename) {
        eprintln!("Filename does not have an extension.");
        return;
    }

    let input_image = image::open(&input_filename).unwrap();
    let rgb_image = input_image.to_rgb();
    let (width, height) = rgb_image.dimensions();

    println!("Auto white balancing:");
    println!("\tInput: {} ({}x{})", input_filename, width, height);

    let all_methods = matches.is_present("all");
    let user_method = match matches.value_of("auto") {
        Some(method_str) => {
            AutoWhiteBalanceMethod::try_from(method_str).ok()
        },
        None => {
            if !all_methods {
                eprintln!("Please select auto white balance method");
                return;
            }
            None
        },
    };

    match user_method {

        Some(method) => {
            do_auto_white_balance_for_method(input_filename,
                                             matches.value_of("output"),
                                             &method,
                                             &rgb_image);
        },
        None => {
            for method in AutoWhiteBalanceMethod::iter() {
                do_auto_white_balance_for_method(input_filename,
                                                 None,
                                                 &method,
                                                 &rgb_image);
            };
        }
    }
}

fn do_auto_white_balance_for_method(input_filename: &str,
                                    output_filename: Option<&str>,
                                    method: &AutoWhiteBalanceMethod,
                                    rgb_image: &image::RgbImage) {
    let (output_filename, image_format) = build_output_filename(input_filename, output_filename, &method);
    println!("\tOutput: {} -> {}", method, output_filename);
    let enhanced_image = rgb_image.auto_white_balance(&method);
    let fout = &mut File::create(&Path::new(&output_filename)).unwrap();
    image::ImageRgb8(enhanced_image).save(fout, image_format).unwrap();
}

fn filename_has_extension(filename: &str) -> bool {
    let split: Vec<&str> = filename.rsplitn(2, ".").collect();
    split.len() == 2
}

fn build_output_filename(input_filename: &str,
                         output_filename: Option<&str>,
                         method: &AutoWhiteBalanceMethod) -> (String, image::ImageFormat) {
    let string_split: Vec<&str> = input_filename.rsplitn(2,".").collect();
    let image_format = match image_format_from_string(string_split[0]) {
        Some(format) => format,
        None => {
            eprintln!("Did not find image format for '{}', using PNG as default", string_split[0]);
            image::ImageFormat::PNG
        }
    };

    match output_filename {
        Some(filename) => (String::from(filename), image_format),
        None => {
            (
                format!("{}-{}.{}", string_split[1], method.to_string(), string_split[0]),
                image_format
            )
        }
    }
}