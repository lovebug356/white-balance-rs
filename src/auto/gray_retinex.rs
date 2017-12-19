extern crate image;
extern crate num_traits;
extern crate nalgebra;

use image::Pixel;
use image::math::utils::clamp;
use self::nalgebra::Matrix2;
use self::nalgebra::Determinant;

use image_ext::math::PixelMath;

fn resolve_matrix(a: Matrix2<f32>, c1: f32, c2: f32) -> (f32, f32) {
    let det = a.determinant();
    if det == 0f32 {
        // keep it as is
        return (0f32, 1f32);
    }

    let d1 = Matrix2::new(c1, a.m12, c2, a.m22);
    let d2 = Matrix2::new(a.m11, c1, a.m21, c2);

    return (d1.determinant() / det, d2.determinant() / det)
}

pub fn auto_white_balance(image: &image::RgbImage) -> image::RgbImage {
    let (width, height) = image.dimensions();
    let pixel_count = width * height;
    let mut out = image::RgbImage::new(width, height);

    let stats = image.measure_stats();

    let rgb_max_square = (
        stats.max[0] as u64 * stats.max[0] as u64,
        stats.max[1] as u64 * stats.max[1] as u64,
        stats.max[2] as u64 * stats.max[2] as u64
    );

    let matrix_r = Matrix2::new(stats.sum_squares[0] as f32, stats.sum[0] as f32,
                                rgb_max_square.0 as f32, stats.max[0] as f32);
    let matrix_b = Matrix2::new(stats.sum_squares[2] as f32, stats.sum[2] as f32,
                                rgb_max_square.2 as f32, stats.max[2] as f32);

    let (det_x, det_y) = resolve_matrix(matrix_r,
                                        stats.avg[1] as f32 * (pixel_count) as f32,
                                        stats.max[1] as f32);
    let (blue_x, blue_y) = resolve_matrix(matrix_b,
                                          stats.avg[1] as f32 * (pixel_count) as f32,
                                          stats.max[1] as f32);

    for (old_pixel, new_pixel) in image.pixels().zip(out.pixels_mut()) {
        let channels = old_pixel.channels();
            let red = channels[0] as f32;
            let new_red = red * red * det_x as f32 + red * det_y as f32;
            let blue = channels[2] as f32;
            let new_blue = blue * blue * blue_x as f32 + blue * blue_y as f32;
            *new_pixel = image::Rgb([
                clamp(new_red, 0f32, 255f32) as u8,
                channels[1],
                clamp(new_blue, 0f32, 255f32) as u8
            ]);
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{thread_rng, Rng};
    use test::Bencher;

    #[bench]
    fn bench_retinex_hd_image(b: &mut Bencher) {
        let frame_size = 1920 * 1080 * 3;
        let mut data = vec![0x00; frame_size];
        thread_rng().fill_bytes(&mut data);
        let x = image::RgbImage::from_vec(
            1920, 1080, data).unwrap();

        b.iter(|| {
            auto_white_balance(&x);
        });
    }
}
