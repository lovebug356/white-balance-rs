extern crate image;

pub trait AutoWhiteBalance {
    fn white_balance(image: &image::RgbImage) -> image::RgbImage;
}
