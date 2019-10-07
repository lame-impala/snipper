use crate::primitives::{Vector, FloatPoint, AbstractPoint, Point};
use crate::units::{Coordinate, Pseudoangle};

#[derive(PartialEq, Debug, Clone)]
pub struct Line {
    pub point: Point,
    pub vector: Vector
}

impl Line {
    pub fn new(point: &Point, vector: &Vector) -> Line {
        Line {point: point.clone(), vector: vector.clone()}
    }
    pub fn is_null(&self) -> bool {
        self.vector.is_null()
    }
    pub fn coordinates_for_parameter(&self, parameter: &f64) -> (f64, f64) {
        let x_float = self.point.float_x() + parameter * self.vector.float_x();
        let y_float = self.point.float_y() + parameter * self.vector.float_y();
        (x_float, y_float)
    }
    pub fn right_down_vector(&self) -> Vector {
        if self.vector.is_right_down() {
            self.vector.clone()
        } else {
            -&self.vector
        }
    }
    pub fn point_at(&self, parameter: &f64) -> Option<Point> {
        if self.is_null() {return None};
        let (x_float, y_float) = self.coordinates_for_parameter(parameter);
        if x_float.is_finite() && y_float.is_finite() {
            let x_result = Coordinate::from_float(x_float);
            let y_result = Coordinate::from_float(y_float);
            if x_result.is_err() || y_result.is_err() {
                None
            } else {
                let x = x_result.unwrap();
                let y = y_result.unwrap();
                Some(Point::unchecked(x, y))
            }
        } else {
            None
        }
    }
    pub fn float_point_at(&self, parameter: &f64) -> Option<FloatPoint> {
        if self.is_null() {return None};
        let (x_float, y_float) = self.coordinates_for_parameter(parameter);
        let result = FloatPoint::new(x_float, y_float);
        if let Ok(point) = result {
            Some(point)
        } else {
            None
        }
    }
    pub fn contains(&self, point: &Point) -> bool {
        if self.is_null() {return false;}
        let candidate = if self.vector.x() == Coordinate::default() {
            let y_factor = (point.float_y() - self.point.float_y()) / self.vector.float_y();
            self.point_at(&y_factor)
        } else {
            let x_factor = (point.float_x() - self.point.float_x()) / self.vector.float_x();
            self.point_at(&x_factor)
        };
        match candidate{
            None => false,
            Some(candidate) => {
                candidate == *point
            }
        }
    }
    pub fn parameter_at_intersection(&self, other: &Line) -> Option<f64>{
        let dtor = self.vector.float_x() * other.vector.float_y() -
            self.vector.float_y() * other.vector.float_x();
        if dtor == 0f64 {
            None
        } else {
            let ntor = self.point.float_y() * other.vector.float_x() -
                self.point.float_x() * other.vector.float_y() +
                other.point.float_x() * other.vector.float_y() -
                other.point.float_y() * other.vector.float_x();
            Some(ntor / dtor)
        }
    }
    pub fn intersection(l1: &Line, l2: &Line) -> Option<Point> {
        if let Some(fp) = Line::float_intersection(l1, l2) {
            Some(Line::tiebreaker(fp, l1, l2))
        } else {
            None
        }
    }
    pub fn bisection_for_tiebreaker(v1: Vector, v2: Vector) -> (f64, f64) {
        let one = Pseudoangle::new(1.0);
        let pa1 = *&v1.pseudoangle().unwrap();
        let pa2 = *&v2.pseudoangle().unwrap();
        let (upper, lower) = if pa1 < pa2 {
            ((v1, pa1), (v2, pa2))
        } else {
            ((v2, pa2), (v1, pa1))
        };
        let (v1, v2) = if &lower.1 - &upper.1 <= one {
            (upper.0, lower.0)
        } else {
            let half_diff: f64 = 1.0 - ((&lower.1 - &upper.1).to_float() / 2.0);
            if upper.1.to_float() > half_diff {
                (-&lower.0, upper.0)

            } else {
                (lower.0, -&upper.0)
            }
        };

        let n1 = v1.normalized().unwrap();
        let n2 = v2.normalized().unwrap();
        let sum_x = n1.0 + n2.0;
        let sum_y = n1.1 + n2.1;
        (sum_x, sum_y)
    }
    pub fn tiebreaker(fp: FloatPoint, l1: &Line, l2: &Line) -> Point {
        let xdp = fp.x().decimal_part().abs();
        let ydp = fp.y().decimal_part().abs();
        if xdp != 0.5 && ydp != 0.5 {
            Point::from_float(&fp).unwrap()
        } else {
            let v1 = l1.right_down_vector();
            let v2 = l2.right_down_vector();
            let (sum_x, sum_y) = Line::bisection_for_tiebreaker(v1, v2);

            let x = if xdp == 0.5 {
                if sum_x < 0.0 {
                    fp.x().floor()
                } else {
                    fp.x().ceil()
                }
            } else {
                fp.x().round()
            };
            let y = if ydp == 0.5 {
                if sum_y < 0.0 {
                    fp.y().floor()
                } else {
                    fp.y().ceil()
                }
            } else {
                fp.y().round()
            };
            Point::unchecked(
                Coordinate::from_float(x).unwrap(),
                Coordinate::from_float(y).unwrap()
            )
        }
    }
    pub fn float_intersection(l1: &Line, l2: &Line) -> Option<FloatPoint> {
        let parameter = Line::parameter_at_intersection(l1, l2);
        if let Some(parameter) = parameter {
            l1.float_point_at(&parameter)
        } else {
            None
        }
    }
    pub fn inspect(&self) -> String {
        format!(
            "[{}, {}] -> [{}, {}]",
            self.point.x().to_int(),
            self.point.y().to_int(),
            self.vector.x().to_int(),
            self.vector.y().to_int()
        )
    }
}
#[test]
fn two_value_tiebreaker_test() {
    test((0, 0), (1, 1), (-1, 0), (2, 1), (1, 1));
    test((0, 0), (1, 1), (-1, 1), (2, 0), (1, 1));
    test((0, 1), (1, 0), (-1, 0), (2, 1), (1, 0));
    test((0, 1), (1, 0), (0, -1), (1, 2), (1, 0));
    test((0, 0), (1, 1), (0, -1), (1, 2), (1, 1));
    test((0, 0), (1, 1), (1, -1), (0, 2), (1, 1));
    test((1, 0), (0, 1), (1, 2), (0, -1), (1, 0));
    test((1, 0), (0, 1), (1, -1), (0, 2), (1, 0));

    fn test(
        first_start: (i32, i32),
        first_end: (i32, i32),
        second_start: (i32, i32),
        second_end: (i32, i32),
        expected: (i32, i32)

    ) {
        let p0a = Point::new(first_start.0, first_start.1).expect("!");
        let p1a = Point::new(first_end.0, first_end.1).expect("!");
        let p0b = Point::new(second_start.0, second_start.1).expect("!");
        let p1b = Point::new(second_end.0, second_end.1).expect("!");
        let ex = Point::new(expected.0, expected.1).expect("!");
        let la = Line::new(&p0a, &Vector::new(&p0a, &p1a));
        let lb = Line::new(&p0b, &Vector::new(&p0b, &p1b));
        assert_eq!(&Line::intersection(&la, &lb).unwrap(), &ex);
    }

}
#[test]
fn one_value_tiebreaker_test() {
    let p0 = Point::new(0, 0).expect("!");
    let pnx = Point::new(2, 0).expect("!");
    let pny = Point::new(1, 1).expect("!");

    let s1 = Point::new(1, -1).expect("!");
    let s2 = Point::new(1, 1).expect("!");

    let e1 = Point::new(2, -1).expect("!");
    let e2 = Point::new(2, 1).expect("!");

    let che = Point::new(2, 0).expect("!");
    let cve = Point::new(1, 1).expect("!");

    let nx = Line::new(&pnx, &Vector::new(&pnx, &p0));
    let ny = Line::new(&pny, &Vector::new(&pny, &s1));

    let vl1 = Line::new(&e2, &Vector::new(&e2, &s1));
    let vl2 = Line::new(&e1, &Vector::new(&e1, &s2));

    let hl1 = Line::new(&e1, &Vector::new(&e1, &p0));
    let hl2 = Line::new(&e2, &Vector::new(&e2, &p0));

    assert_eq!(&Line::intersection(&nx, &vl1).unwrap(), &che);
    assert_eq!(&Line::intersection(&nx, &vl2).unwrap(), &che);
    assert_eq!(&Line::intersection(&ny, &hl1).unwrap(), &s1);
    assert_eq!(&Line::intersection(&ny, &hl2).unwrap(), &cve);
}
#[test]
fn line_test() {
    let l0 = Line{
        point: Point::new(0, 0).expect("!"),
        vector: Vector::from(&Point::new(0, 0).unwrap())
    };
    assert!(l0.is_null());
    assert_eq!(l0.point_at(&0.0), Option::None);
    assert!(!l0.contains(&Point::new(0, 0).expect("!")));
    assert_eq!(Line::intersection(&l0, &l0), Option::None);

    let l1 = Line{
        point: Point::new(0, 0).expect("!"),
        vector: Vector::from(&Point::new(2, 6).unwrap())
    };
    assert!(!l1.is_null());
    assert_eq!(l1.point_at(&0.0).unwrap(), Point::new(0, 0).expect("!"));
    assert_eq!(l1.point_at(&0.5).unwrap(), Point::new(1, 3).expect("!"));
    assert_eq!(l1.point_at(&1.0).unwrap(), Point::new(2, 6).expect("!"));
    assert!(l1.contains(&Point::new(0, 0).expect("!")));
    assert!(l1.contains(&Point::new(0, 0).expect("!")));
    assert!(l1.contains(&Point::new(-1, -3).expect("!")));
    assert_eq!(Line::intersection(&l1, &l0), Option::None);
    assert_eq!(Line::intersection(&l0, &l1), Option::None);
    assert_eq!(Line::intersection(&l1, &l1), Option::None);
    let l2 = Line{
        point: Point::new(0, 6).expect("!"),
        vector: Vector::from(&Point::new(2, -6).unwrap())
    };
    assert_eq!(Line::intersection(&l1, &l2).unwrap(), Point::new(1, 3).expect("!"));
    assert_eq!(Line::intersection(&l2, &l1).unwrap(), Point::new(1, 3).expect("!"));
    let l3 = Line{
        point: Point::new(2, 0).expect("!"),
        vector: Vector::from(&Point::new(-1, 3).unwrap())
    };
    assert_eq!(Line::intersection(&l1, &l3).unwrap(), Point::new(1, 3).expect("!"));
    assert_eq!(Line::intersection(&l3, &l1).unwrap(), Point::new(1, 3).expect("!"));
}
