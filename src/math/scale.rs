use std::ops::{Div, Mul};

pub trait ToFromu8 {
    fn to_u8(self) -> u8;
    fn from_u8(v: u8) -> Self;
    fn from_f32(v: f32) -> Self;
}

macro_rules! impl_to_from_u8 {
    ($($ty:ty)*) => {
        $(
            impl ToFromu8 for $ty {
                #[inline]
                fn to_u8(self) -> u8 {
                    if self > Self::from(u8::max_value()) {
                        u8::max_value()
                    } else {
                        self as u8
                    }
                }
                #[inline]
                fn from_u8(v: u8) -> $ty {
                    v as $ty
                }
                #[inline]
                fn from_f32(v: f32) -> $ty {
                    v as $ty
                }
            }
        )*
    }
}

impl_to_from_u8!(f32 f64);

#[inline]
pub fn scale_pixel<T: ToFromu8>(pixel: u8, numerator: T, denominator: T) -> u8
        where T: Clone + Mul<T, Output = T> + Div<T, Output = T>{
    let res = T::from_u8(pixel) * numerator / denominator;
    res.to_u8()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_normal_scaling() {
        assert_eq!(scale_pixel(10u8, 2f32, 1f32), 20u8);
        assert_eq!(scale_pixel(10u8, 1f32, 2f32), 5u8);
        assert_eq!(scale_pixel(10u8, 2f64, 1f64), 20u8);
        assert_eq!(scale_pixel(10u8, 1f64, 2f64), 5u8);
    }

    #[test]
    fn test_overflow() {
        let new_pixel = scale_pixel(250, 2f32, 1f32);
        assert_eq!(new_pixel, 255u8);
    }
}
