use crate::primitives::{AbstractPoint, Mode, Sector, Vector, Bounds, Line};
use crate::units::{Coordinate, Float};
use crate::Point;

#[derive(PartialEq, Debug, Clone)]
pub struct Straight {
    pub start: Point,
    pub end: Point,
    pub bounds: Bounds
}
impl Straight {
    pub fn is_endpoint(&self, point: &Point) -> bool {
        point == &self.start || point == &self.end
    }
    pub fn new(start: &Point, end: &Point) -> Straight {
        let bounds = Bounds::from_extremes(start.y(), start.x(), end.y(), end.x());
        Straight{start: start.clone(), end: end.clone(), bounds}
    }
    pub fn vector(&self) -> Vector {
        Vector::new(&self.start, &self.end)
    }
    pub fn upper_left(&self) -> &Point {
        if self.start.is_lower_right(&self.end) {
            &self.end
        } else {
            &self.start
        }
    }
    pub fn lower_right(&self) -> &Point {
        if self.start.is_lower_right(&self.end) {
            &self.start
        } else {
            &self.end
        }
    }
    pub fn midpoint(&self) -> Point {
        let x_float = (self.start.float_x() + self.end.float_x()) / 2f64;
        let x = Coordinate::from_float(x_float).expect("Unexpected out-of-bounds error");
        let y_float = (self.start.float_y() + self.end.float_y()) / 2f64;
        let y = Coordinate::from_float(y_float).expect("Unexpected out-of-bounds error");
        Point::unchecked(x, y)
    }
    pub fn is_null(&self) -> bool {
        self.start == self.end
    }
    pub fn to_line(&self) -> Line {
        let v = Vector::new(&self.start, &self.end);
        Line::new(&self.start, &v)
    }
    pub fn cross_with_vertical_or_upper(&self, x: Coordinate) -> Option<Float> {
        if self.upper_left().x() == x {
            Some(Float::from(self.upper_left().y()))
        } else {
            self.cross_with_vertical(x)
        }
    }
    pub fn cross_with_vertical(&self, x: Coordinate) -> Option<Float> {
        let left = self.bounds.left();
        let right = self.bounds.right();
        if left > x ||
            right < x ||
            left == right {
            None
        } else {
            let start = self.upper_left().y();
            let end = self.lower_right().y();

            let diff_x = f64::from((right - left).to_int());
            let offset = f64::from((x - left).to_int());
            let parameter = offset / diff_x;
            let diff_y = f64::from((end - start).to_int());
            let raise = diff_y * parameter;
            let float = Float::new(raise + f64::from(start.to_int())).unwrap();
            Some(float)
        }
    }


    pub fn may_cross(s1: &Straight, s2: &Straight) -> bool {
        if Bounds::have_collision(&s1.bounds, &s2.bounds) {
            let end_to_start1 = Vector::new(&s1.end, &s1.start);
            let d1 = Vector::cross_product(&end_to_start1, &Vector::new(&s1.end, &s2.start));
            let d2 = Vector::cross_product(&end_to_start1, &Vector::new(&s1.end, &s2.end));
            if d1 * d2 >= 0.0 {
                false
            } else {
                let end_to_start2 = Vector::new(&s2.end, &s2.start);
                let d3 = Vector::cross_product(&end_to_start2, &Vector::new(&s2.end, &s1.start));
                let d4 = Vector::cross_product(&end_to_start2, &Vector::new(&s2.end, &s1.end));
                d3 * d4 < 0.0
            }
        } else {
            false
        }
    }
    pub fn length(&self) -> f64 {
        let v = Vector::new(&self.start, &self.end);
        v.length()
    }
    pub fn reverse(&self) -> Straight {
        Straight::new(&self.end, &self.start)
    }
    pub fn inspect(&self) -> String {
        format!("{} -> {}", self.start.inspect(), self.end.inspect())
    }
    pub fn contains_proper(&self, point: &Point) -> bool {
        if !self.bounds.contains(point, &Mode::Closed) {
            false
        } else if point == &self.start || point == &self.end {
            false
        } else {
            let l1 = self.to_line();
            l1.contains(point)
        }
    }
}
impl Sector for Straight {
    fn contains (&self, point: &Point, mode: &Mode) -> bool {
        if point == &self.start {
            true
        } else if point == &self.end {
            *mode == Mode::Closed
        } else if !self.bounds.contains(point, &Mode::Closed) {
            false
        } else {
            let l1 = self.to_line();
            l1.contains(point)
        }
    }
}
#[test]
fn cross_with_vertical_test() {

    let p00 = Point::new(0, 0).expect("!");
    let p01 = Point::new(30, 20).expect("!");
    let s = Straight::new(&p00, &p01);
    assert_eq!(
        s.cross_with_vertical(Coordinate::new(0)).unwrap(),
        Float::new(0.0).unwrap()
    );
    assert_eq!(
        s.cross_with_vertical(Coordinate::new(10)).unwrap(),
        Float::new(6.666666666666666).unwrap()
    );
    assert_eq!(
        s.cross_with_vertical(Coordinate::new(20)).unwrap(),
        Float::new(13.333333333333332).unwrap()
    );
    assert_eq!(
        s.cross_with_vertical(Coordinate::new(30)).unwrap(),
        Float::new(20.0).unwrap()
    );

    let p00 = Point::new(0, -10).expect("!");
    let p01 = Point::new(30, 10).expect("!");
    let s = Straight::new(&p00, &p01);

    assert_eq!(
        s.cross_with_vertical(Coordinate::new(0)).unwrap(),
        Float::new(-10.0).unwrap()
    );
    assert_eq!(
        s.cross_with_vertical(Coordinate::new(10)).unwrap(),
        Float::new(-3.333333333333334).unwrap()
    );
    assert_eq!(
        s.cross_with_vertical(Coordinate::new(20)).unwrap(),
        Float::new(3.333333333333332).unwrap()
    );
    assert_eq!(
        s.cross_with_vertical(Coordinate::new(30)).unwrap(),
        Float::new(10.0).unwrap()
    );

    let p00 = Point::new(0, 10).expect("!");
    let p01 = Point::new(30, -10).expect("!");
    let s = Straight::new(&p00, &p01);
    assert_eq!(
        s.cross_with_vertical(Coordinate::new(0)).unwrap(),
        Float::new(10.0).unwrap()
    );
    assert_eq!(
        s.cross_with_vertical(Coordinate::new(10)).unwrap(),
        Float::new(3.333333333333334).unwrap()
    );
    assert_eq!(
        s.cross_with_vertical(Coordinate::new(20)).unwrap(),
        Float::new(-3.333333333333332).unwrap()
    );
    assert_eq!(
        s.cross_with_vertical(Coordinate::new(30)).unwrap(),
        Float::new(-10.0).unwrap()
    );

    let p00 = Point::new(-2, 0).expect("!");
    let p01 = Point::new(-1, 20).expect("!");
    let s = Straight::new(&p00, &p01);
    assert!(
        s.cross_with_vertical(Coordinate::new(0)).is_none()
    );
    let p00 = Point::new(1, 0).expect("!");
    let p01 = Point::new(2, 20).expect("!");
    let s = Straight::new(&p00, &p01);
    assert!(
        s.cross_with_vertical(Coordinate::new(0)).is_none()
    );
    let p00 = Point::new(0, 0).expect("!");
    let p01 = Point::new(0, 20).expect("!");
    let s = Straight::new(&p00, &p01);
    assert!(
        s.cross_with_vertical(Coordinate::new(0)).is_none()
    );


}
#[test]
fn contains_proper_test() {
    let p0 = Point::new(3, 0).unwrap();
    let p1 = Point::new(8, 1).unwrap();
    let p2 = Point::new(4, 0).unwrap();
    let s = Straight::new(&p0, &p1);
    assert!(s.contains_proper(&p2));
}
#[test]
fn midpoint_test() {
    let p1 = Point::new(-1, -1).expect("!");
    let p2 = Point::new(1, -1).expect("!");
    let p3 = Point::new(1, 1).expect("!");
    let p4 = Point::new(-1, 1).expect("!");

    let s1 = Straight::new(&p1, &p3);
    let s2 = Straight::new(&p3, &p1);
    let s3 = Straight::new(&p2, &p4);
    let s4 = Straight::new(&p4, &p2);

    let p0 = Point::new(0, 0).expect("!");
    assert_eq!(s1.midpoint(), p0);
    assert_eq!(s2.midpoint(), p0);
    assert_eq!(s3.midpoint(), p0);
    assert_eq!(s4.midpoint(), p0);
}
#[test]
fn straight_test() {
    let p0 = Point::new(0, 0).expect("!");
    let p1 = Point::new(4, 0).expect("!");
    let p2 = Point::new(4, 4).expect("!");
    let p3 = Point::new(0, 4).expect("!");
    let p6 = Point::new(-4, -4).expect("!");
    let s = Straight::new(&p0, &p0);
    assert!(s.is_null());
    assert_eq!(s.bounds.top(), Coordinate::default());
    assert_eq!(s.bounds.left(), Coordinate::default());
    assert_eq!(s.bounds.top(), Coordinate::default());
    assert_eq!(s.upper_left(), &p0);
    assert_eq!(s.lower_right(), &p0);
    assert_eq!(s.bounds.bottom(), Coordinate::default());
    let s1 = Straight::new(&p0, &p1);
    assert_eq!(s1.upper_left(), &p0);
    assert_eq!(s1.lower_right(), &p1);
    let s2 = Straight::new(&p0, &p2);
    let s3 = Straight::new(&p0, &p3);
    assert_eq!(s3.upper_left(), &p0);
    assert_eq!(s3.lower_right(), &p3);
    let s6 = Straight::new(&p0, &p6);
    assert_eq!(s6.bounds.top(), Coordinate::new(-4));
    assert_eq!(s6.bounds.left(), Coordinate::new(-4));
    assert_eq!(s6.bounds.bottom(), Coordinate::new(0));
    assert_eq!(s6.bounds.right(), Coordinate::new(0));
    assert_eq!(s6.upper_left(), &p6);
    assert_eq!(s6.lower_right(), &p0);


    let pt1 = Point::new(2, 2).expect("!");
    let pt2 = Point::new(-2, -2).expect("!");
    let pt3 = Point::new(6, 6).expect("!");
    assert!(s2.contains(&p0, &Mode::Open));
    assert!(s2.contains(&p0, &Mode::Closed));
    assert!(s2.contains(&pt1, &Mode::Open));
    assert!(s2.contains(&pt1, &Mode::Closed));
    assert!(!s2.contains(&p2, &Mode::Open));
    assert!(s2.contains(&p2, &Mode::Closed));
    assert!(!s2.contains(&pt2, &Mode::Open));
    assert!(!s2.contains(&pt2, &Mode::Closed));
    assert!(!s2.contains(&pt3, &Mode::Open));
    assert!(!s2.contains(&pt3, &Mode::Closed));

    let s9 = Straight::new(&Point::new(0, 0).expect("!"), &Point::new(1, 1000).expect("!"));
    let s10 = Straight::new(&Point::new(1, 0).expect("!"), &Point::new(2, 1000).expect("!"));
    let s11 = Straight::new(&Point::new(1, 0).expect("!"), &Point::new(0, 1000).expect("!"));
    assert!(!Straight::may_cross(&s9, &s10));
    assert!(Straight::may_cross(&s9, &s11));
}
