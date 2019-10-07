use crate::units::{Pseudoangle, Coordinate};
use crate::primitives::{AbstractPoint, Point};
use crate::error::{Validation, Error, ValidationErrors, ValidationError};
use std::ops;
use std::ops::Neg;

#[derive(PartialEq, Debug, Clone)]
pub struct Vector {
    x: Coordinate,
    y: Coordinate
}
impl Vector {
    pub fn new(start: &Point, end: &Point) -> Vector {
        let x = end.x() - start.x();
        let y = end.y() - start.y();
        Vector{x, y}
    }
    pub fn x(&self) -> Coordinate { self.x }
    pub fn y(&self) -> Coordinate { self.y }
    pub fn is_right_down (&self) -> bool {
        if self.x > Coordinate::default() {
            return true
        } else if self.x == Coordinate::default() {
            self.y > Coordinate::default()
        } else {
            false
        }
    }
    pub fn float_x(&self) -> f64 {
        Coordinate::into(self.x())
    }
    pub fn float_y(&self) -> f64 {
        Coordinate::into(self.y())
    }
    pub fn is_null(&self) -> bool {
        let zero = Coordinate::default();
        self.x() == zero && self.y() == zero
    }
    pub fn normalized(&self) -> Option<(f64, f64)> {
        if self.is_null() {
            return None
        } else {
            let len = self.length();
            let x = self.float_x() / len;
            let y = self.float_y() / len;
            Some((x, y))
        }
    }
    pub fn length(&self) -> f64 {
        (self.float_x().powi(2) + self.float_y().powi(2)).sqrt()
    }
    pub fn pseudoangle(&self) -> Option<Pseudoangle> {
        if self.is_null() { return None }

        let dx = self.float_x();
        let dy = self.float_y();
        let denominator = dx.abs() + dy.abs();
        if denominator == 0.0 {
            Some(Pseudoangle::new(0.0))
        } else {
            let p = dx / denominator;
            if dy < 0.0 {
                Some(Pseudoangle::new(0.0 + p))
            } else {
                Some(Pseudoangle::new(2.0 - p))
            }
        }
    }
    pub fn dot_product(v1: &Vector, v2: &Vector) -> f64 {
        v1.float_x() * v2.float_x() + v1.float_y() * v2.float_y()
    }
    pub fn cross_product(v1: &Vector, v2: &Vector) -> f64 {
        v1.float_x() * v2.float_y() - v1.float_y() * v2.float_x()
    }
    pub fn perpendicular_projection(&self, point: &Point) -> Option<f64> {
        if self.is_null() { return None; }
        let v1 = Vector{x: point.x(), y: point.y()};
        let sp1 = Vector::dot_product(&v1, self);
        let sp2 = Vector::dot_product(self, self);
        return Some(sp1 / sp2)
    }
    pub fn same_direction(&self, rhs: &Vector) -> bool {
        // Assuming but not asserting vectors are parallel
        let zero = Coordinate::default();
        let result = if self.is_null() {
            if rhs.is_null() {
                true
            } else {
                false
            }
        } else if rhs.is_null() {
            false
        } else if self.x() == zero || rhs.x() == zero {
            self.x() == rhs.x() && self.float_y() / rhs.float_y() > 0f64
        } else if self.y() == zero || rhs.y() == zero {
            self.y() == rhs.y() && self.float_x() / rhs.float_x() > 0f64
        } else {
            let p1 = self.float_x() / rhs.float_x();
            if p1 < 0f64 {
                false
            } else {
                let p2 = self.float_y() / rhs.float_y();
                if p2 < 0f64 {
                    false
                } else {
                    true
                }
            }
        };
        result
    }
    pub fn inspect(&self) -> String {
        format!("-> {}, {}", self.x(), self.y())
    }
}
impl Neg for &Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector{x: -self.x, y: -self.y}
    }
}
impl<'a, 'b> ops::Add<&'b Vector> for &'a Vector{
    type Output = Vector;
    fn add (self, rhs: &'b Vector) -> Vector {
        let x = self.x() + rhs.x();
        let y = self.y() + rhs.y();
        Vector{x, y}
    }
}
impl<'a> ops::Mul<f64> for &'a Vector{
    type Output = Result<Vector, Error>;
    fn mul (self, rhs: f64) -> Result<Vector, Error> {
        let x_float = self.float_x() * rhs;
        let y_float = self.float_y() * rhs;
        let x_result = Coordinate::from_float(x_float);
        let y_result = Coordinate::from_float(y_float);
        let mut validation= Validation::new();
        validation.report_result(&"x", &x_result.as_ref());
        validation.report_result(&"y", &y_result.as_ref());
        if validation.is_ok() {
            let x = x_result.unwrap();
            let y = y_result.unwrap();
            Ok(Vector{x, y})
        } else {
            std::result::Result::Err(Error::out_of_bounds(&validation).unwrap())
        }
    }
}
impl From<&Point> for Vector {
    fn from(point: &Point) -> Vector {
        Vector{x: point.x(), y: point.y()}
    }
}
#[test]
fn pseudoangle_test() {
    let max = crate::units::coordinate::Coordinate::MAX;
    let min = crate::units::coordinate::Coordinate::MIN;
    let p0 = Point::new(0, 0).expect("!");

    let p1a = Point::new(-1, min).expect("!");
    let p1 = Point::new(0, min).expect("!");
    let p1b = Point::new(1, min).expect("!");

    let p2a = Point::new(max - 1, min).expect("!");
    let p2 = Point::new(max, min).expect("!");
    let p2b = Point::new(max, min + 1).expect("!");

    let p3a = Point::new(max, -1).expect("!");
    let p3 = Point::new(max, 0).expect("!");
    let p3b = Point::new(max, 1).expect("!");

    let p4a = Point::new(max, max - 1).expect("!");
    let p4 = Point::new(max, max).expect("!");
    let p4b = Point::new(max - 1, max).expect("!");

    let p5a = Point::new(1, max).expect("!");
    let p5 = Point::new(0, max).expect("!");
    let p5b = Point::new(-1, max).expect("!");

    let p6a = Point::new(min + 1, max).expect("!");
    let p6 = Point::new(min, max).expect("!");
    let p6b = Point::new(min, max - 1).expect("!");

    let p7a = Point::new(min, 1).expect("!");
    let p7 = Point::new(min, 0).expect("!");
    let p7b = Point::new(min, -1).expect("!");

    let p8a = Point::new(min, min + 1).expect("!");
    let p8 = Point::new(min, min).expect("!");
    let p8b = Point::new(min + 1, min).expect("!");

    let mut va = Vector::new(&p0, &p1a);
    let mut vb = Vector::new(&p0, &p1);
    let mut vc = Vector::new(&p0, &p1b);

    let tad0 = 0.9999999403953588;
    let tad1 = 0.0000000596046412;
    let tad2 = 0.4999999850988384;
    let tad3 = 0.5000000149011616;


    assert_eq!(va.pseudoangle().unwrap(), Pseudoangle::new(3.0 + tad0));
    assert_eq!(vb.pseudoangle().unwrap(), Pseudoangle::new(0.0));
    assert_eq!(vc.pseudoangle().unwrap(), Pseudoangle::new(0.00000005960464122267716));

    va = Vector::new(&p0, &p2a);
    vb = Vector::new(&p0, &p2);
    vc = Vector::new(&p0, &p2b);

    assert_eq!(va.pseudoangle().unwrap(), Pseudoangle::new(0.49999998509883836));
    assert_eq!(vb.pseudoangle().unwrap(), Pseudoangle::new(0.5));
    assert_eq!(vc.pseudoangle().unwrap(), Pseudoangle::new(0.0 + tad3));

    va = Vector::new(&p0, &p3a);
    vb = Vector::new(&p0, &p3);
    vc = Vector::new(&p0, &p3b);

    assert_eq!(va.pseudoangle().unwrap(), Pseudoangle::new(0.0 + tad0));
    assert_eq!(vb.pseudoangle().unwrap(), Pseudoangle::new(1.0));
    assert_eq!(vc.pseudoangle().unwrap(), Pseudoangle::new(1.000000059604641222676946));

    va = Vector::new(&p0, &p4a);
    vb = Vector::new(&p0, &p4);
    vc = Vector::new(&p0, &p4b);

    assert_eq!(va.pseudoangle().unwrap(), Pseudoangle::new(1.0 + tad2));
    assert_eq!(vb.pseudoangle().unwrap(), Pseudoangle::new(1.5));
    assert_eq!(vc.pseudoangle().unwrap(), Pseudoangle::new(1.0 + tad3));

    va = Vector::new(&p0, &p5a);
    vb = Vector::new(&p0, &p5);
    vc = Vector::new(&p0, &p5b);

    assert_eq!(va.pseudoangle().unwrap(), Pseudoangle::new(1.0 + tad0));
    assert_eq!(vb.pseudoangle().unwrap(), Pseudoangle::new(2.0));
    assert_eq!(vc.pseudoangle().unwrap(), Pseudoangle::new(2.0 + tad1));

    va = Vector::new(&p0, &p6a);
    vb = Vector::new(&p0, &p6);
    vc = Vector::new(&p0, &p6b);

    assert_eq!(va.pseudoangle().unwrap(), Pseudoangle::new(2.0 + tad2));
    assert_eq!(vb.pseudoangle().unwrap(), Pseudoangle::new(2.5));
    assert_eq!(vc.pseudoangle().unwrap(), Pseudoangle::new(2.0 + tad3));

    va = Vector::new(&p0, &p7a);
    vb = Vector::new(&p0, &p7);
    vc = Vector::new(&p0, &p7b);

    assert_eq!(va.pseudoangle().unwrap(), Pseudoangle::new(2.0 + tad0));
    assert_eq!(vb.pseudoangle().unwrap(), Pseudoangle::new(3.0));
    assert_eq!(vc.pseudoangle().unwrap(), Pseudoangle::new(3.0 + tad1));

    va = Vector::new(&p0, &p8a);
    vb = Vector::new(&p0, &p8);
    vc = Vector::new(&p0, &p8b);

    assert_eq!(va.pseudoangle().unwrap(), Pseudoangle::new(3.0 + tad2));
    assert_eq!(vb.pseudoangle().unwrap(), Pseudoangle::new(3.5));
    assert_eq!(vc.pseudoangle().unwrap(), Pseudoangle::new(3.0 + tad3));
}
#[test]
fn vector_cross_product_test() {
    let start = Point::new(0, 0).expect("!");
    let end1 = Point::new(3, 2).expect("!");
    let end2 = Point::new(3, 0).expect("!");
    let v1 = Vector::new(&start, &end1);
    let v2 = Vector::new(&start, &end2);
    assert_eq!(Vector::cross_product(&v2, &v1), 6.0);
}
#[test]
fn vector_test() {
    let v0 = Vector{x: Coordinate::default(), y: Coordinate::default()};
    assert_eq!(v0.length(), 0f64);
    assert!(v0.is_null());
    let start = Point::new(2, 5).expect("!");
    let end = Point::new(6, 3).expect("!");
    let v1 = Vector::new(&start, &end);
    assert_eq!(v1.x().to_int(), 4);
    assert_eq!(v1.y().to_int(), -2);
    assert_eq!(v1.length(), 20f64.sqrt());
    assert_eq!(Vector::cross_product(&v1, &v1), 0f64);
    assert_eq!(Vector::dot_product(&v1, &v1), 20f64);
    let v2 = Vector{x: Coordinate::new(2), y: Coordinate::new(3)};
    assert_eq!(Vector::cross_product(&v0, &v2), 0f64);
    assert_eq!(Vector::dot_product(&v0, &v2), 0f64);
    let v3 = Vector{x: Coordinate::new(-2), y: Coordinate::new(-3)};
    assert_eq!(Vector::cross_product(&v3, &v2), 0f64);
    assert_eq!(Vector::dot_product(&v3, &v2), -13f64);
    let v4 = Vector{x: Coordinate::new(3), y: Coordinate::new(-2)};
    assert_eq!(Vector::cross_product(&v4, &v2), 13f64);
    assert_eq!(Vector::dot_product(&v4, &v2), 0f64);

    assert_eq!(Vector{x: Coordinate::new(5), y: Coordinate::new(1)}, &v2 + &v4);
    assert_eq!(Vector{x: Coordinate::new(4), y: Coordinate::new(6)}, (&v2 * 2f64).unwrap());


    assert!(v0.same_direction(&v0));
    assert!(!v1.same_direction(&v0));
    assert!(!v0.same_direction(&v1));
    let v5 = Vector{x: Coordinate::new(0), y: Coordinate::new(5)};
    assert!(v5.same_direction(&v5));
    let v6 = Vector{x: Coordinate::new(5), y: Coordinate::new(0)};
    assert!(v6.same_direction(&v6));
    assert!(v3.same_direction(&v3));

}
