use image;
use std::cmp;
use image::Pixel;

#[derive(Default)]
pub struct RgbImageStats {
    pub max: [u8; 3],
    pub avg: [f64; 3],
    pub sum: [u64; 3],
    pub sum_squares: [u64; 3],
//    avg_square: [f32; 3],
//    max_square: [u64; 3],
}

//impl Default for RgbImageStats {
//    fn default() -> RgbImageStats {
//        RgbImageStats {
//            max: [0u8, 0u8, 0u8],
//            avg: [0f32, 0f32, 0f32],
//            sum: [0u64, 0u64, 0u64]
//        }
//    }
//}

pub trait PixelMath {
    fn max_per_channel(&self) -> (u8, u8, u8);
    fn avg_per_channel(&self) -> (f32, f32, f32);
    fn sum_of_squares_per_channel(&self) -> (f64, f64, f64);
    fn sum_per_channel(&self) -> (f64, f64, f64);
    fn avg_square_per_channel(&self) -> (f32, f32, f32);
    fn measure_stats(&self) -> Box<RgbImageStats>;
}

impl PixelMath for image::RgbImage {
    fn measure_stats(&self) -> Box<RgbImageStats> {
        let (width, height) = self.dimensions();
        let mut stats: Box<RgbImageStats> = Box::new(Default::default());

        for pixel in self.pixels() {
            let channels = pixel.channels();
            for channel_idx in 0..3 {
                stats.sum[channel_idx] += channels[channel_idx] as u64;
                stats.sum_squares[channel_idx] += channels[channel_idx] as u64 * channels[channel_idx] as u64;
                stats.max[channel_idx] = cmp::max(stats.max[channel_idx], channels[channel_idx]);
            }
        }

        for channel_idx in 0..3 {
            stats.avg[channel_idx] = (stats.sum[channel_idx] as f64) / (width * height) as f64;
        }

        stats
    }

    fn max_per_channel(&self) -> (u8, u8, u8) {
        let x = self.pixels().fold((0u8, 0u8, 0u8), |m, p| {
            let c = p.channels4();
            (cmp::max(m.0, c.0), cmp::max(m.1, c.1), cmp::max(m.2, c.2))
        });
        x
    }

    fn avg_per_channel(&self) -> (f32, f32, f32) {
        let (width, height) = self.dimensions();
        let pixel_count = width * height;
        let mut sum = (0u64, 0u64, 0u64);
        for pixel in self.pixels() {
            let channels = pixel.channels();
            sum.0 += channels[0] as u64;
            sum.1 += channels[1] as u64;
            sum.2 += channels[2] as u64;
        };
        (
            sum.0 as f32 / (pixel_count) as f32,
            sum.1 as f32 / (pixel_count) as f32,
            sum.2 as f32 / (pixel_count) as f32
        )
    }

    fn sum_of_squares_per_channel(&self) -> (f64, f64, f64) {
        self.pixels().fold((0f64, 0f64, 0f64), |m, p| {
            let c = p.channels4();
            (m.0 + c.0 as f64 * c.0 as f64,
             m.1 + c.1 as f64 * c.1 as f64,
             m.2 + c.2 as f64 * c.2 as f64)
        })
    }

    fn sum_per_channel(&self) -> (f64, f64, f64) {
        self.pixels().fold((0f64, 0f64, 0f64), |m, p| {
            let c = p.channels4();
            (m.0 + c.0 as f64,
             m.1 + c.1 as f64,
             m.2 + c.2 as f64)
        })
    }

    fn avg_square_per_channel(&self) -> (f32, f32, f32) {
        let x = self.pixels().fold((0f32, 0f32, 0f32), |m, p| {
            let c = p.channels4();
            (m.0 + c.0 as f32 * c.0 as f32, m.1 + c.1 as f32 * c.1 as f32, m.2 + c.2 as f32 * c.2 as f32)
        });
        x
    }
}