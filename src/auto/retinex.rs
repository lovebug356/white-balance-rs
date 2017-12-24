use image;
use image::Pixel;
use ::image_ext::PixelMath;
use ::math::scale::scale_pixel;

pub fn auto_white_balance(image: &image::RgbImage) -> image::RgbImage {
    let (width, height) = image.dimensions();
    let mut out = image::RgbImage::new(width, height);

    let rgb_max = image.max_per_channel();

    for (old_pixel, new_pixel) in image.pixels().zip(out.pixels_mut()) {
        let channels = old_pixel.channels();
        *new_pixel = image::Rgb([
            scale_pixel(channels[0], rgb_max.1 as f32, rgb_max.0 as f32),
            channels[1],
            scale_pixel(channels[2], rgb_max.1 as f32, rgb_max.2 as f32)
        ]);
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{thread_rng, Rng};
    use test::Bencher;

    #[test]
    fn test_equal_resolution() {
        let orig_image = image::RgbImage::new(10, 10);
        let new_image = auto_white_balance(&orig_image);

        assert_eq!(orig_image.dimensions(), new_image.dimensions());
    }

    #[test]
    fn test_should_be_exact_copy_if_white_balance_is_ok() {
        let data = vec![128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128];
        let orig_image = image::RgbImage::from_vec(2, 2, data)
            .unwrap();
        let white_image = auto_white_balance(&orig_image);

        for y in 0..2 {
            for x in 0..2 {
                let orig_p = orig_image.get_pixel(x, y);
                let white_p = white_image.get_pixel(x, y);

                assert_eq!(orig_p, white_p);
            }
        }
    }

    #[bench]
    fn bench_retinex_hd_image(b: &mut Bencher) {
        let frame_size = 1920 * 1080 * 3;
        let mut data = vec![0x00; frame_size];
        thread_rng().fill_bytes(&mut data);
        let x = image::RgbImage::from_vec(
            1920, 1080, data).unwrap();

        // On Mac Mini:
        // 19,383,469 ns/iter
        b.iter(|| {
            auto_white_balance(&x);
        });
    }
}
