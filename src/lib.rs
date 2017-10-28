#![feature(test)]
#![feature(rand)]

extern crate image;
extern crate num;
extern crate num_traits;
extern crate test;
extern crate rand;

pub mod image_ext;
pub mod math;

pub mod auto;
pub use auto::*;
