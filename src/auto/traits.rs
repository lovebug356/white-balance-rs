use super::methods::AutoWhiteBalanceMethod;

pub trait AutoWhiteBalance {
    fn auto_white_balance(&self, method: &AutoWhiteBalanceMethod) -> Self;
}
