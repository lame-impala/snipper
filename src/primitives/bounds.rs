use crate::Point;
use crate::primitives::{AbstractPoint, Mode, Sector};
use crate::units::Coordinate;

#[derive(PartialEq, Debug, Clone)]
pub struct Bounds {
    top: Coordinate,
    left: Coordinate,
    bottom: Coordinate,
    right: Coordinate
}
impl Bounds{
    pub fn top(&self) -> Coordinate { self.top }
    pub fn left(&self) -> Coordinate { self.left }
    pub fn bottom(&self) -> Coordinate { self.bottom }
    pub fn right(&self) -> Coordinate { self.right }
    pub fn width(&self) -> Coordinate {
        self.right - self.left
    }
    pub fn height(&self) -> Coordinate {
        self.bottom - self.top
    }
    pub fn union(a: &Bounds, b: &Bounds) -> Bounds{
        let top = a.top().min(b.top());
        let left = a.left().min(b.left());
        let bottom = a.bottom().max(b.bottom());
        let right = a.right().max(b.right());
        Bounds{top, left, bottom, right}
    }
    pub fn new(y1: i32, x1: i32, y2: i32, x2: i32) -> Bounds {
        let top = Coordinate::new(y1.min(y2));
        let left = Coordinate::new(x1.min(x2));
        let bottom = Coordinate::new(y1.max(y2));
        let right = Coordinate::new(x1.max(x2));
        Bounds{top, left, bottom, right}
    }
    pub fn from_extremes(
        y1: Coordinate,
        x1: Coordinate,
        y2: Coordinate,
        x2: Coordinate
    ) -> Bounds {
        let top = y1.min(y2);
        let left = x1.min(x2);
        let bottom = y1.max(y2);
        let right = x1.max(x2);
        Bounds{top, left, bottom, right}
    }
    pub fn contains_closed_mode(&self, point: &Point) -> bool {
        if point.y() < self.top() { return false; }
        if point.x() < self.left() { return false; }
        if point.y() > self.bottom() { return false; }
        if point.x() > self.right() { return false; }
        true
    }
    pub fn contains_open_mode(&self, point: &Point) -> bool {
        if point.y() <= self.top() { return false; }
        if point.x() <= self.left() { return false; }
        if point.y() >= self.bottom() { return false; }
        if point.x() >= self.right() { return false; }
        true
    }
    pub fn have_collision(b1: &Bounds, b2: &Bounds) -> bool {
        if b1.left() >= b2.right() || b2.left() >= b1.right() ||
            b1.top() >= b2.bottom() || b2.top() >= b1.bottom() {
            false
        } else {
            true
        }
    }
}
impl Sector for Bounds {
    fn contains(&self, point: &Point, mode: &Mode) -> bool {
        match mode {
            &Mode::Open => self.contains_open_mode(point),
            &Mode::Closed => self.contains_closed_mode(point)
        }
    }
}
#[test]
fn bounds_test() {
    let b = Bounds::new(2, 5, 0, 0);
    assert_eq!(b.top(), Coordinate::default());
    assert_eq!(b.left(), Coordinate::default());
    assert_eq!(b.bottom(), Coordinate::new(2));
    assert_eq!(b.right(), Coordinate::new(5));

    let bu1 = Bounds::new(-1, -1, 4, 6);
    let u1 = Bounds::union(&b, &bu1);
    assert_eq!(u1.top(), Coordinate::new(-1));
    assert_eq!(u1.left(), Coordinate::new(-1));
    assert_eq!(u1.bottom(), Coordinate::new(4));
    assert_eq!(u1.right(), Coordinate::new(6));
    let u2 = Bounds::union(&bu1, &b);
    assert_eq!(u2.top(), Coordinate::new(-1));
    assert_eq!(u2.left(), Coordinate::new(-1));
    assert_eq!(u2.bottom(), Coordinate::new(4));
    assert_eq!(u2.right(), Coordinate::new(6));


    assert!(!b.contains(&Point::new(0, -1).expect("!"), &Mode::Closed));
    assert!(!b.contains(&Point::new(-1, 0).expect("!"), &Mode::Closed));
    assert!(!b.contains(&Point::new(5, -1).expect("!"), &Mode::Closed));
    assert!(!b.contains(&Point::new(6, 2).expect("!"), &Mode::Closed));
    assert!(!b.contains(&Point::new(-1, -1).expect("!"), &Mode::Closed));
    assert!(!b.contains(&Point::new(6, 3).expect("!"), &Mode::Closed));
    assert!(b.contains(&Point::new(1, 1).expect("!"), &Mode::Closed));
    assert!(b.contains(&Point::new(0, 0).expect("!"), &Mode::Closed));
    assert!(b.contains(&Point::new(5, 2).expect("!"), &Mode::Closed));

    let b1 = Bounds::new(0, 0, 10, 10);

    let b2f = Bounds::new(0, -10, 10, 0);
    let b2t = Bounds::new(0, -10, 10, 1);
    assert!(!Bounds::have_collision(&b1, &b2f));
    assert!(!Bounds::have_collision(&b2f, &b1));
    assert!(Bounds::have_collision(&b1, &b2t));
    assert!(Bounds::have_collision(&b2t, &b1));

    let b3f = Bounds::new(-10, 0, 0, 10);
    let b3t = Bounds::new(-10, 0, 1, 10);
    assert!(!Bounds::have_collision(&b1, &b3f));
    assert!(!Bounds::have_collision(&b3f, &b1));
    assert!(Bounds::have_collision(&b1, &b3t));
    assert!(Bounds::have_collision(&b3t, &b1));

    let b4f = Bounds::new(0, 10, 10, 20);
    let b4t = Bounds::new(0, 9, 10, 20);
    assert!(!Bounds::have_collision(&b1, &b4f));
    assert!(!Bounds::have_collision(&b4f, &b1));
    assert!(Bounds::have_collision(&b1, &b4t));
    assert!(Bounds::have_collision(&b4t, &b1));

    let b5f = Bounds::new(10, 0, 20, 10);
    let b5t = Bounds::new(9, 0, 20, 10);
    assert!(!Bounds::have_collision(&b1, &b5f));
    assert!(!Bounds::have_collision(&b5f, &b1));
    assert!(Bounds::have_collision(&b1, &b5t));
    assert!(Bounds::have_collision(&b5t, &b1));
}
