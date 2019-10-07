use std::{fmt, ops};
use std::hash::{Hash, Hasher};
use crate::units::integer_decode::integer_decode;
use crate::error::{ValidationError, BasicValidationError};
use std::cmp::Ordering;
use crate::units::Coordinate;

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Float {
    value: f64
}
impl Float {
    pub fn new(value: f64) -> Result<Float, BasicValidationError> {
        if value.is_finite() {
            Ok(Float{value})
        } else {
            Err(BasicValidationError::new())
        }
    }
    pub fn abs(self) -> Float {
        Float{ value: self.value.abs() }
    }
    pub fn min(self, rhs: Float) -> Float {
        let min_value = self.value.min(rhs.value);
        Float::new(min_value).unwrap()
    }
    pub fn max(self, rhs: Float) -> Float {
        let max_value = self.value.max(rhs.value);
        Float::new(max_value).unwrap()
    }
    pub fn round(&self) -> f64 {
        self.value.round()
    }
    pub fn integer_decode(&self) -> (u64, i16, i8){
        if self.value == 0f64 {
            (0u64, 0i16, 0i8)
        } else {
            integer_decode(self.value)
        }
    }
    pub fn decimal_part(&self) -> f64 {
        (self.value - self.value.trunc())
    }
    pub fn floor(&self) -> f64 {
        self.value.floor()
    }
    pub fn ceil(&self) -> f64 {
        self.value.ceil()
    }
}
impl Eq for Float {}
impl Ord for Float {
    fn cmp(&self, other: &Float) -> Ordering {
        if self.value < other.value {
            Ordering::Less
        } else if self.value > other.value {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl Default for Float {
    fn default() -> Float {
        return Float{value: 0.0}
    }
}
impl Hash for Float {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let (m, e, s) = self.integer_decode();
        state.write_u64(m);
        state.write_i16(e);
        state.write_i8(s);
    }
}
impl From<i32> for Float {
    fn from(int: i32) -> Float {
        Float{value: int as f64}
    }
}
impl From<Coordinate> for Float {
    fn from(coo: Coordinate) -> Float {
        Float{value: coo.to_int() as f64}
    }
}
impl From<&Coordinate> for Float {
    fn from(coo: &Coordinate) -> Float {
        Float{value: coo.to_int() as f64}
    }
}
impl From<Float> for f64 {
    fn from(float: Float) -> f64 {
        float.value
    }
}

impl<'a, 'b> ops::Add<&'b Float> for &'a Float{
    type Output = Float;
    fn add (self, rhs: &'b Float) -> Float {
        let value = self.value + rhs.value;
        Float::new(value).unwrap()
    }
}
impl<'a, 'b> ops::Sub<&'b Float> for &'a Float{
    type Output = Float;
    fn sub (self, rhs: &'b Float) -> Float {
        let value = self.value - rhs.value;
        Float::new(value).unwrap()
    }
}

#[test]
fn test_floor_and_ceil() {
    assert_eq!(Float::new(1.49).unwrap().floor(), 1.0);
    assert_eq!(Float::new(1.49).unwrap().ceil(), 2.0);
    assert_eq!(Float::new(-1.49).unwrap().floor(), -2.0);
    assert_eq!(Float::new(-1.49).unwrap().ceil(), -1.0);

}
#[test]
fn test_decimal_part() {
    assert_eq!(Float::new(1.49).unwrap().decimal_part(), 0.49);
    assert_eq!(Float::new(1.51).unwrap().decimal_part(), 0.51);
    assert_eq!(Float::new(-1.49).unwrap().decimal_part(), -0.49);
    assert_eq!(Float::new(-1.51).unwrap().decimal_part(), -0.51);
}