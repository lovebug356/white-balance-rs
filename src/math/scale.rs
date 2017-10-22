use num::Bounded;
use image::math::utils::clamp;

pub trait ToFromf64 {
    fn to_f64(self) -> f64;
    fn from_f64(v: f64) -> Self;
}

macro_rules! impl_to_from_f64 {
    ($($ty:ty)*) => {
        $(
            impl ToFromf64 for $ty {
                #[inline]
                fn to_f64(self) -> f64 {
                    self as f64
                }
                #[inline]
                fn from_f64(v: f64) -> $ty {
                    v as $ty
                }
            }
        )*
    }
}

impl_to_from_f64!(u8 u16 f32);

pub fn scale_pixel<T: ToFromf64+Bounded, S: ToFromf64> (input: T, numerator: S, denominator: S) -> T {
    let res = input.to_f64() * numerator.to_f64() / denominator.to_f64();
    let res = clamp(res, 0f64, T::max_value().to_f64());
    T::from_f64(res)
}