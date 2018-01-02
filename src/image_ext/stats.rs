use std::ops::Deref;
use image::Pixel;
use image::ImageBuffer;
use image::Primitive;
use num_traits::NumCast;
use itertools::multizip;

pub trait PixelStats<S> {
    fn pixel_count(&self) -> u32;
    fn max(&self) -> [S; 4];
    fn min(&self) -> [S; 4];
    fn sum(&self) -> [f32; 4];
    fn mean(&self) -> [f32; 4];
    fn std(&self) -> [f32; 4];
}

impl<P, S, C> PixelStats<S> for ImageBuffer<P, C>
    where P: Pixel<Subpixel=S> + 'static,
          S: Primitive + 'static,
          C: Deref<Target=[P::Subpixel]> {

    fn pixel_count(&self) -> u32 {
        let (width, height) = self.dimensions();
        width * height
    }

    fn max(&self) -> [S; 4] {
        let mut res = [S::min_value(); 4];

        for pixel in self.pixels() {
            for (c, r) in pixel.channels().iter().zip(res.iter_mut()) {
                if *r < *c {
                    *r = *c;
                }
            }
        }

        res
    }

    fn min(&self) -> [S; 4] {
        let mut res = [S::max_value(); 4];

        for pixel in self.pixels() {
            for (c, r) in pixel.channels().iter().zip(res.iter_mut()) {
                if *r > *c {
                    *r = *c;
                }
            }
        }

        res
    }

    fn sum(&self) -> [f32; 4] {
        let mut res = [0f32; 4];

        for pixel in self.pixels() {
            for (c, r) in pixel.channels().iter().zip(res.iter_mut()) {
                let val: f32 = NumCast::from(*c).unwrap();
                *r += val;
            }
        }

        res
    }

    fn mean(&self) -> [f32; 4] {
        let pixel_count = self.pixel_count();
        let mut res = [0f32; 4];

        for (s, r) in self.sum().iter().zip(res.iter_mut()) {
            *r = *s / pixel_count as f32;
        }

        res
    }

    fn std(&self) -> [f32; 4] {
        let mut res = [0f32; 4];
        let mean = self.mean();
        let pixel_count = self.pixel_count() as f32;

        for pixel in self.pixels() {
            for (c, m, r) in multizip((pixel.channels(), mean.iter(), res.iter_mut())) {
                let val: f32 = NumCast::from(*c).unwrap();
                *r += (val - m).powi(2);
            }
        }

        for idx in 0..P::channel_count() {
            res[idx as usize] = (res[idx as usize] / (pixel_count - 1f32)).sqrt();
        }

        res
    }
}

#[cfg(test)]
mod stats_test {
    use super::*;
    use image::GrayImage;
    use image::GrayAlphaImage;

    #[test]
    fn test_mean_luma_image() {
        let data = vec![0u8; 10];
        let image = GrayImage::from_raw(10, 1, data).unwrap();
        let res = image.mean();
        assert_eq!(res[0], 0f32);

        let data = vec![10u8; 10];
        let image = GrayImage::from_raw(10, 1, data).unwrap();
        let res = image.mean();
        assert_eq!(res[0], 10f32);

        let data = vec![0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8];
        let image = GrayAlphaImage::from_raw(4, 1, data).unwrap();
        let res = image.mean();
        assert_eq!(res[0], 3f32);
        assert_eq!(res[1], 4f32);
    }

    #[test]
    fn test_max_luma_image() {
        let data = vec![0u8; 10];
        let image = GrayImage::from_raw(10, 1, data).unwrap();
        let res = image.max();
        assert_eq!(res[0], 0u8);

        let data = vec![10u8, 8u8, 9u8, 12u8, 1u8, 2u8, 4u8, 1u8, 2u8, 4u8];
        let image = GrayImage::from_raw(10, 1, data).unwrap();
        let res = image.max();
        assert_eq!(res[0], 12u8);

        let data = vec![10u8, 8u8, 9u8, 12u8, 1u8, 2u8, 4u8, 1u8, 2u8, 4u8];
        let image = GrayAlphaImage::from_raw(5, 1, data).unwrap();
        let res = image.max();
        assert_eq!(res[0], 10u8);
        assert_eq!(res[1], 12u8);
    }
}
