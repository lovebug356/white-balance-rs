use image;
use image::Pixel;

use ::scale::scale_pixel;
use ::math::PixelMath;

pub fn auto_white_balance(image: &image::RgbImage) -> image::RgbImage {
    let (width, height) = image.dimensions();
    let mut out = image::RgbImage::new(width, height);
    let avg = image.avg_per_channel();

    for y in 0..height {
        for x in 0..width {
            let p = image.get_pixel(x, y);
            let c = p.channels4();
            let p = image::Rgb::from_channels(
                scale_pixel(c.0, avg.1, avg.0),
                c.1,
                scale_pixel(c.2, avg.1, avg.2),
                0
            );
            out.put_pixel(x, y, p);
        }
    }

    out
}

#[cfg(test)]
mod gray_test {
    use super::*;

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

    #[test]
    fn test_should_correct_red_and_blue_channels_to_average_green_channel() {
        let (width, height) = (2, 2);
        let data = vec![100, 120, 200, 100, 120, 200, 100, 120, 200, 100, 120, 200];
        let orig_image = image::RgbImage::from_vec(width, height, data)
            .unwrap();
        let white_image = auto_white_balance(&orig_image);

        let exp_data = vec![120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120];
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
}