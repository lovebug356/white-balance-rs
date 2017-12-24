use std::fmt;

#[derive(PartialEq, Debug)]
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

impl AutoWhiteBalanceMethod {
    pub fn iter() -> AutoWhiteBalanceMethodIterator {
        AutoWhiteBalanceMethodIterator::new()
    }

    pub fn try_from(val: &str) -> Result<Self, String> {
        for method in AutoWhiteBalanceMethod::iter() {
            if method.to_string() == val {
                return Ok(method)
            }
        }
        Err(format!("Auto white balance method '{}' not recognized", val))
    }
}

pub struct AutoWhiteBalanceMethodIterator {
    idx: u8,
}

impl AutoWhiteBalanceMethodIterator {
    fn new() -> AutoWhiteBalanceMethodIterator {
        AutoWhiteBalanceMethodIterator{idx: 0}
    }
}

impl Iterator for AutoWhiteBalanceMethodIterator {
    type Item = AutoWhiteBalanceMethod;

    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.idx {
            0 => Some(AutoWhiteBalanceMethod::GrayWorld),
            1 => Some(AutoWhiteBalanceMethod::Retinex),
            2 => Some(AutoWhiteBalanceMethod::GrayRetinex),
            _ => None
        };
        self.idx += 1;
        res
    }
}

#[cfg(test)]
mod methods_test {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(AutoWhiteBalanceMethod::GrayWorld.to_string(), "gray-world");
    }

    #[test]
    fn test_try_from() {
        let res = AutoWhiteBalanceMethod::try_from("gray-world");

        assert_eq!(res.is_err(), false);
        assert_eq!(res.unwrap(), AutoWhiteBalanceMethod::GrayWorld);

        let res = AutoWhiteBalanceMethod::try_from("retinex");
        assert_eq!(res.unwrap(), AutoWhiteBalanceMethod::Retinex);

        let res = AutoWhiteBalanceMethod::try_from("wrong");
        assert_eq!(res.is_err(), true);
    }
}