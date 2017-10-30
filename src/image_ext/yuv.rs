use image;
use image::Pixel;

#[derive(PartialEq, Default, Eq, Clone, Debug, Copy, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct Yuv {
    pub data: [u8; 3]
}

#[inline]
fn scale_comp(channels: &[u8], r: f32, g:f32, b: f32, offset: f32) -> u8 {
    let res = channels[0] as f32 * r + channels[1] as f32 * g + channels[2] as f32 * b + offset;
    res as u8
}

impl From<image::Rgb<u8>> for Yuv {
    fn from(pixel: image::Rgb<u8>) -> Self {
        let channels = pixel.channels();
        Self {
            data: [
                scale_comp(channels, 0.299, 0.587, 0.114, 0f32),
                scale_comp(channels, -0.14713, -0.28886, 0.436, 128f32),
                scale_comp(channels, 0.615, -0.51499, -0.1001, 128f32)
            ]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_rgb() {
        let rgb = image::Rgb([0u8, 0u8, 0u8]);
        assert_eq!(Yuv::from(rgb).data, [0u8, 128u8, 128u8]);

        let rgb = image::Rgb([100u8, 120u8, 130u8]);
        assert_eq!(Yuv::from(rgb).data, [115u8, 135u8, 114u8]);

        let rgb = image::Rgb([255u8, 255u8, 255u8]);
        assert_eq!(Yuv::from(rgb).data, [255u8, 128u8, 127u8]);
    }
}