use image;

use super::traits::AutoWhiteBalance;
use super::methods::AutoWhiteBalanceMethod;

use super::retinex;
use super::gray_world;
use super::gray_retinex;

impl AutoWhiteBalance for image::RgbImage {
    fn auto_white_balance(&self, method: &AutoWhiteBalanceMethod) -> image::RgbImage {
        match method {
            &AutoWhiteBalanceMethod::GrayWorld => {
                gray_world::auto_white_balance(&self)
            },
            &AutoWhiteBalanceMethod::Retinex => {
                retinex::auto_white_balance(&self)
            },
            &AutoWhiteBalanceMethod::GrayRetinex => {
                gray_retinex::auto_white_balance(&self)
            }
        }
    }
}