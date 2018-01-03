#![feature(test)]
#![feature(rand)]

extern crate image;
extern crate num;
extern crate num_traits;
extern crate test;
extern crate rand;
extern crate itertools;

mod auto;
mod image_ext;
mod math;

pub use self::auto::AutoWhiteBalance;
pub use self::auto::AutoWhiteBalanceMethod;

pub use self::image_ext::image_format_from_string;
