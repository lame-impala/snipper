use std::fmt;
use std::ops;
use std::convert::From;
use crate::error::{ValidationError, BasicValidationError};


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub struct Coordinate {
    value: i32
}
impl Coordinate {
    pub const MAX: i32 = 16777216; // 2^24
    pub const MIN: i32 = -16777216;
    pub const ONE: Coordinate = Coordinate{value: 1};
    pub fn checked(value: i32) -> Result<Coordinate, BasicValidationError> {
        if value < Coordinate::MIN {
            let error = BasicValidationError::new();
            Err(error)
        } else if value > Coordinate::MAX {
            let error = BasicValidationError::new();
            Err(error)
        } else {
            Ok(Coordinate::new(value))
        }
    }
    pub fn new(value: i32) -> Coordinate {
        Coordinate{value}
    }
    pub fn from_float (float: f64) -> Result<Coordinate, BasicValidationError> {
        let rounded = float.round();
        if rounded < std::i32::MIN as f64 || rounded > std::i32::MAX as f64 {
            let error = BasicValidationError::new();
            Err(error)
        } else {
            Ok(Coordinate::new(rounded as i32))
        }
    }
    pub fn min(self, rhs: Coordinate) -> Coordinate {
        let min_value = self.value.min(rhs.value);
        Coordinate::new(min_value)
    }
    pub fn max(self, rhs: Coordinate) -> Coordinate {
        let max_value = self.value.max(rhs.value);
        Coordinate::new(max_value)
    }
    pub fn to_int(&self) -> i32 {
        self.value
    }
}
impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl Default for Coordinate {
    fn default() -> Coordinate {
        return Coordinate{value: 0}
    }
}
impl ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;
    fn add (self, rhs: Coordinate) -> Coordinate {
        let value = self.value + rhs.value;
        Coordinate::new(value)
    }
}
impl ops::Sub< Coordinate> for  Coordinate {
    type Output = Coordinate;
    fn sub (self, rhs: Coordinate) -> Coordinate {
        let value = self.value - rhs.value;
        Coordinate::new(value)
    }
}

impl From<Coordinate> for f64 {
    fn from(value: Coordinate) -> f64 {
        value.value as f64
    }
}

impl std::ops::Neg for Coordinate {
    type Output = Coordinate;
    fn neg(self) -> Coordinate {
        Coordinate::new(-self.value)
    }
}

#[test]
fn coordinate_test() {
    let c1 = Coordinate::new(50);
    let c2 = Coordinate::new(20);
    assert_eq!((c1 - c2).to_int(), 30);
    assert_eq!((c1 + c2).to_int(), 70);
    assert_eq!((c2 - c1).to_int(), -30);
    assert_eq!((-c2).to_int(), -20);
    assert_eq!((c2.min(c1)).to_int(), 20);
    assert_eq!((c2.max(c1)).to_int(), 50);
}