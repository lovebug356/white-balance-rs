extern crate image;
extern crate num_traits;
extern crate nalgebra;

use image::Pixel;
use image::math::utils::clamp;
use self::nalgebra::Matrix2;
use self::nalgebra::Determinant;

use traits::AutoWhiteBalance;
use ::math::PixelMath;

pub struct GrayRetinex;

fn resolve_matrix(a: Matrix2<f64>, c1: f64, c2: f64) -> (f64, f64) {
    let det = a.determinant();
    if det == 0f64 {
        // keep it as is
        return (0f64, 1f64);
    }

    let d1 = Matrix2::new(c1, a.m12, c2, a.m22);
    let d2 = Matrix2::new(a.m11, c1, a.m21, c2);

    return (d1.determinant() / det, d2.determinant() / det)
}

impl AutoWhiteBalance for GrayRetinex {
    fn white_balance(image: &image::RgbImage) -> image::RgbImage {
        let (width, height) = image.dimensions();
        let mut out = image::RgbImage::new(width, height);

        let sum_squares = image.sum_of_squares_per_channel();
        let sum = image.sum_per_channel();
        let rgb_max = image.max_per_channel();
        let avg_square = image.avg_square_per_channel();
        let avg = image.avg_per_channel();
        let rgb_max_square = (
            rgb_max.0 as u64 * rgb_max.0 as u64, rgb_max.1 as u64 * rgb_max.1 as u64, rgb_max.2 as u64 * rgb_max.2 as u64
        );

        let matrix_r = Matrix2::new(sum_squares.0, sum.0, rgb_max_square.0 as f64, rgb_max.0 as f64);
        let matrix_b = Matrix2::new(sum_squares.2, sum.2, rgb_max_square.2 as f64, rgb_max.2 as f64);

//        let matrix_r = Matrix2::new(avg_square.0 as f64, avg.0 as f64, rgb_max_square.0 as f64, rgb_max.0 as f64);
//        let matrix_b = Matrix2::new(avg_square.2 as f64, avg.2 as f64, rgb_max_square.2 as f64, rgb_max.2 as f64);
//        let matrix_a = Matrix2::new(avg_square.0 as f64, avg.0 as f64, rgb_max_square.0 as f64, rgb_max.0 as f64);
//        let matrix_c = (avg_square.1 as f64, )
//        let d1 = Matrix2::new(avg.1 as f64 * (width * height) as f64, avg.0 as f64, rgb_max.1 as f64, rgb_max.0 as f64);
//        let d2 = Matrix2::new(avg_square.0 as f64, avg.1 as f64 * (width * height) as f64, rgb_max_square.0 as f64, rgb_max.1 as f64);
//        println!("matrix_r\n{}", matrix_r);
//        println!("Inverse A: {:?}", matrix_r.inverse());
//        println!("d1\n{}", d1);
//        println!("d2\n{}", d2);

        let (det_x, det_y) = resolve_matrix(matrix_r, avg.1 as f64 * (width * height) as f64, rgb_max.1 as f64);
        let (blue_x, blue_y) = resolve_matrix(matrix_b, avg.1 as f64 * (width * height) as f64, rgb_max.1 as f64);

//        let det_x = d1.determinant() / matrix_r.determinant();
//        let det_y = d2.determinant() / matrix_r.determinant();
//        let y = d2.determinant() / matrix_r.determinant();

        println!("x = {}, y = {}", det_x, det_y);


        println!("Determinant {}", avg_square.0 as f64 * rgb_max.0 as f64 - avg.0 as f64 * rgb_max_square.0 as f64);

        for y in 0..height {
            for x in 0..width {
                let p = image.get_pixel(x, y);
                let c = p.channels4();
                let red = c.0 as f64;
                let new_red = red * red * det_x + red * det_y;
                let blue = c.2 as f64;
                let new_blue = blue * blue * blue_x + blue * blue_y;
                if x < 1 && y < 1 {
                    println!("new red {} -> {}", red, new_red);
                    println!("new blue {} -> {}", blue, new_blue);
                    println!("{} * {} + {} * {}", red * red, det_x, red, det_y);
                }
                let p = image::Rgb::from_channels(
                    clamp((new_red as f32), 0f32, 255f32) as u8,
                    c.1,
                    clamp((new_blue as f32), 0f32, 255f32) as u8,
                    0
                );
                out.put_pixel(x, y, p);
            }
        }

        out
    }
}
