use std::fmt;

pub enum AutoWhiteBalanceMethod {
    GrayWorld,
    Retinex,
    GrayRetinex,
}

impl fmt::Display for AutoWhiteBalanceMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            AutoWhiteBalanceMethod::GrayWorld => "gray-world",
            AutoWhiteBalanceMethod::Retinex => "retinex",
            AutoWhiteBalanceMethod::GrayRetinex => "gray-retinex",
        };
        write!(f, "{}", name)
    }
}

pub trait AutoWhiteBalance {
    fn auto_white_balance(&self, method: AutoWhiteBalanceMethod) -> Self;
}

#[cfg(test)]
mod gray_test {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(AutoWhiteBalanceMethod::GrayWorld.to_string(), "gray-world");
    }
}