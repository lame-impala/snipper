use std::hash::Hash;
use std::fmt::Debug;
use crate::units::{Coordinate, Float};
use crate::error::{ValidationError, Validation, BasicValidationError, ValidationErrors};
use crate::Error;
use crate::primitives::Vector;

pub trait AbstractPoint: Debug + PartialEq + Eq + Hash + Clone {
    type Coordinate: PartialEq + Eq + PartialOrd + Ord;
    fn x(&self) -> Self::Coordinate;
    fn y(&self) -> Self::Coordinate;
    fn inspect(&self) -> String;
    fn is_lower_right(&self, other: &Self) -> bool {
        if self.x() == other.x() {
            self.y() > other.y()
        } else {
            self.x() > other.x()
        }
    }

}
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Point {
    x: Coordinate,
    y: Coordinate
}
impl Point {
    pub fn new(x_int: i32, y_int: i32) -> Result<Point, Error> {
        let mut validation: Validation<&str, BasicValidationError> = Validation::new();
        let x_result = Coordinate::checked(x_int);
        let y_result = Coordinate::checked(y_int);
        validation.report_result(&"x", &x_result.as_ref());
        validation.report_result(&"y", &y_result.as_ref());
        if validation.is_ok() {
            let x = x_result.unwrap();
            let y = y_result.unwrap();
            Ok(Point::unchecked(x, y))
        } else {
            std::result::Result::Err(Error::out_of_bounds(&validation).unwrap())
        }
    }
    pub fn from_float(point: &FloatPoint) -> Result<Point, Error> {
        let x_float = point.x().round();
        let y_float = point.y().round();
        let x_result = Coordinate::from_float(x_float);
        let y_result = Coordinate::from_float(y_float);
        let mut validation: Validation<&str, BasicValidationError> = Validation::new();
        validation.report_result(&"x", &x_result.as_ref());
        validation.report_result(&"y", &y_result.as_ref());
        if validation.is_ok() {
            let x = x_result.unwrap();
            let y = y_result.unwrap();
            Ok(Point::unchecked(x, y))
        } else {
            std::result::Result::Err(Error::out_of_bounds(&validation).unwrap())
        }

    }
    pub fn unchecked(x: Coordinate, y: Coordinate) -> Point {
        Point{x, y}
    }
    pub fn shift(&self, v: &Vector) -> Point {
        let x = self.x() + v.x();
        let y = self.y() + v.y();
        Point::unchecked(x, y)
    }
    pub fn float_x (&self) -> f64 {
        Coordinate::into(self.x())
    }
    pub fn float_y (&self) -> f64 {

        Coordinate::into(self.y())
    }
}
impl AbstractPoint for Point {
    type Coordinate = Coordinate;
    fn x(&self) -> Coordinate {
        self.x
    }
    fn y(&self) -> Coordinate {
        self.y
    }
    fn inspect(&self) -> String {
        format!("[{}, {}]", self.x().to_int(), self.y().to_int())
    }
}
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inspect())
    }
}
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct FloatPoint {
    x: Float,
    y: Float,
}
impl FloatPoint {
    pub fn new(x: f64, y: f64) -> Result<FloatPoint, Error> {
        let mut validation: Validation<&str, BasicValidationError> = Validation::new();
        let x_result = Float::new(x);
        let y_result = Float::new(y);
        validation.report_result(&"x", &x_result.as_ref());
        validation.report_result(&"y", &y_result.as_ref());
        if validation.is_ok() {
            let x = x_result.unwrap();
            let y = y_result.unwrap();
            Ok(FloatPoint{x, y})
        } else {
            std::result::Result::Err(Error::not_a_number(&validation).unwrap())
        }

    }

    pub fn x(&self) -> Float {
        self.x
    }
    pub fn y(&self) -> Float {
        self.y
    }
    pub fn inspect(&self) -> String {
        format!("[{:.2}, {:.2}]", self.x(), self.y())
    }
}
impl From<&Point> for FloatPoint {
    fn from(point: &Point) -> Self {
        let x = Float::new(point.float_x()).unwrap();
        let y = Float::new(point.float_y()).unwrap();
        FloatPoint{x, y}
    }
}
impl std::fmt::Debug for FloatPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inspect())
    }
}
#[test]
fn point_test() {
    let p1 = Point::new(1, 2).expect("!");
    let p2 = Point::new(1, 1).expect("!");
    let p3 = Point::new(0, 2).expect("!");
    assert_eq!(p1, p1);
    assert_ne!(p1, p2);
    assert_ne!(p1, p3);
}