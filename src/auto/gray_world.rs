use image;
use image::Rgb;
use image::Pixel;

use ::math::scale::{scale_pixel};
use ::image_ext::math::PixelMath;

pub fn auto_white_balance(image: &image::RgbImage) -> image::RgbImage {
    let (width, height) = image.dimensions();
    let mut out = image::RgbImage::new(width, height);
    let avg = image.avg_per_channel();

    for y in 0..height {
        for x in 0..width {
            let channels = image.get_pixel(x, y).channels();
            let new_pixel = Rgb([
                scale_pixel(channels[0], avg.1, avg.0),
                channels[1],
                scale_pixel(channels[2], avg.1, avg.2)
            ]);
            out.put_pixel(x, y, new_pixel);
        }
    }

    out
}

pub fn auto_white_balance_get_pixel(image: &image::RgbImage, out: &mut image::RgbImage) {
    let (width, height) = image.dimensions();
    let avg = image.avg_per_channel();

    for y in 0..height {
        for x in 0..width {
            let channels = image.get_pixel(x, y).channels();
            let new_pixel = image::Rgb([
                scale_pixel(channels[0], avg.1, avg.0),
                channels[1],
                scale_pixel(channels[2], avg.1, avg.2)
            ]);
            out.put_pixel(x, y, new_pixel);
        }
    }
}

#[cfg(test)]
mod gray_test {
    use super::*;
    use test::Bencher;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_equal_resolution() {
        let orig_image = image::RgbImage::new(10, 10);
        let new_image = auto_white_balance(&orig_image);

        assert_eq!(orig_image.dimensions(), new_image.dimensions());
    }

    #[test]
    fn test_should_be_exact_copy_if_white_balance_is_ok() {
        let data = vec![
            128, 128, 128,
            128, 128, 128,
            128, 128, 128,
            128, 128, 128
        ];
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

    #[test]
    fn test_should_correct_red_and_blue_channels_to_average_green_channel() {
        let (width, height) = (2, 2);
        let data = vec![
            100, 120, 200,
            100, 120, 200,
            100, 120, 200,
            100, 120, 200
        ];
        let orig_image = image::RgbImage::from_vec(width, height, data)
            .unwrap();
        let white_image = auto_white_balance(&orig_image);

        let exp_data = vec![
            120, 120, 120,
            120, 120, 120,
            120, 120, 120,
            120, 120, 120
        ];
        let exp_image = image::RgbImage::from_vec(width, height, exp_data)
            .unwrap();

        for y in 0..2 {
            for x in 0..2 {
                let white_p = white_image.get_pixel(x, y);
                let exp_p = exp_image.get_pixel(x, y);

                assert_eq!(white_p, exp_p);
            }
        }
    }

    #[bench]
    fn bench_gray_world_hd_image(b: &mut Bencher) {
        let frame_size = 1920 * 1080 * 3;
        let mut data = vec![0x00; frame_size];
        thread_rng().fill_bytes(&mut data);
        let x = image::RgbImage::from_vec(
            1920, 1080, data).unwrap();

        b.iter(|| {
            auto_white_balance(&x);
        });
    }

    #[bench]
    fn bench_gray_world_hd_image_get_pixel(b: &mut Bencher) {
        let frame_size = 1920 * 1080 * 3;
        let mut data = vec![0x00; frame_size];
        thread_rng().fill_bytes(&mut data);
        let x = image::RgbImage::from_vec(
            1920, 1080, data).unwrap();
        let mut out = image::RgbImage::new(1920, 1080);

        b.iter(||{
            auto_white_balance_get_pixel(&x, &mut out);
        })
    }
}
