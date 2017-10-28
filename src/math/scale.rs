use std::cmp::PartialOrd;
use std::convert::From;
use std::ops::{Div, Mul};
use std::cmp::Eq;
use num_traits::ToPrimitive;

use num::Bounded;
use image::math::utils::clamp;

//pub trait ToFromf64 {
//    fn to_f64(self) -> f64;
//    fn from_f64(v: f64) -> Self;
//}
//
//macro_rules! impl_to_from_f64 {
//    ($($ty:ty)*) => {
//        $(
//            impl ToFromf64 for $ty {
//                #[inline]
//                fn to_f64(self) -> f64 {
//                    self as f64
//                }
//                #[inline]
//                fn from_f64(v: f64) -> $ty {
//                    v as $ty
//                }
//            }
//        )*
//    }
//}
//
//impl_to_from_f64!(u8 u16 f32);
//
//#[inline]
//pub fn scale_pixel<T: ToFromf64+Bounded, S: ToFromf64> (input: T, numerator: S, denominator: S) -> T {
//    let res = input.to_f64() * numerator.to_f64() / denominator.to_f64();
//    let res = clamp(res, 0f64, T::max_value().to_f64());
//    T::from_f64(res)
//}

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
                    self as u8
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
    let res = clamp(res.to_u8(), u8::min_value(), u8::max_value());
    res
}