use crate::primitives::{Straight, AbstractPoint, Point};
use crate::units::{Pseudoangle};
use crate::primitives::Line;
use std::hash::{Hasher, Hash};

#[derive(Clone)]
pub struct Edge {
    pub index: usize,
    pub subject: usize,
    pub clipping: usize,
    pub straight: Straight,
    pub pseudoangle: Pseudoangle,
}

impl Edge {
    pub fn original(index: usize, origin: Operand, start: &Point, end: &Point) -> Option<Edge> {
        let (subject, clipping) = match origin {
            Operand::Subject => (1, 0),
            Operand::Clipping => (0, 1)
        };
        Edge::new(
            index,
            subject,
            clipping,
            start,
            end,
        )
    }
    pub fn new(
        index: usize,
        subject: usize,
        clipping: usize,
        start: &Point,
        end: &Point,
    ) -> Option<Edge> {
        let (start, end) = if start.is_lower_right(end) {
            (end, start)
        } else {
            (start, end)
        };
        let straight = Straight::new(start, end);
        if straight.is_null() {
            None
        } else {
            let pseudoangle = straight.vector().pseudoangle().unwrap();
            Some(Edge { index, straight, subject, clipping, pseudoangle })
        }
    }
    pub fn count(&self, operand: Option<Operand>) -> usize {
        match operand {
            Some(operand) => {
                match operand {
                    Operand::Subject => self.subject,
                    Operand::Clipping => self.clipping
                }
            },
            None => self.subject + self.clipping
        }
    }
    pub fn is_endpoint(&self, point: &Point) -> bool {
        self.straight.is_endpoint(point)
    }
    pub fn upper_left(&self) -> &Point {
        debug_assert!(self.straight.end.is_lower_right(&self.straight.start));
        &self.straight.start
    }
    pub fn lower_right(&self) -> &Point {
        debug_assert!(self.straight.end.is_lower_right(&self.straight.start));
        &self.straight.end
    }
    pub fn slope(&self) -> Line {
        Line::new(self.upper_left(), &self.straight.vector())
    }
    pub fn merge(mut self, other: Edge) -> Edge {
        debug_assert!(
            self.straight.start == other.straight.start &&
            self.straight.end == other.straight.end,
            "Merged edges must be identical"
        );
        self.subject += other.subject;
        self.clipping += other.clipping;
        self
    }
    pub fn left_split(
        self, point: &Point,
        left_index: usize,
        right_index: usize
    ) -> (Option<Edge>, Option<Edge>) {
        let left = {
            Edge::new(
                left_index,
                self.subject,
                self.clipping,
                self.upper_left(),
                &point,
            )
        };
        let right = {
            Edge::new(
                right_index,
                self.subject,
                self.clipping,
                &point,
                &self.lower_right(),
            )
        };
        (left, right)
    }
    pub fn inspect(&self) -> String {
        format!(
            "Edge#{}-{}/{} [{}, {}] -> [{}, {}], {}",
            self.index,
            self.subject,
            self.clipping,
            self.straight.start.x(),
            self.straight.start.y(),
            self.straight.end.x(),
            self.straight.end.y(),
            self.pseudoangle.display_value()
        )
    }
    pub fn pseudoangle_for_upper_left(&self) -> Pseudoangle {
        self.pseudoangle
    }
    pub fn pseudoangle_for_lower_right(&self) -> Pseudoangle {
        self.pseudoangle.reverse()
    }
    pub fn pseudoangle_for_pivot(&self, pivot: &Point) -> Option<Pseudoangle> {
        if pivot == &self.straight.start {
            Some(self.pseudoangle)
        } else if pivot == &self.straight.end {
            Some(self.pseudoangle_for_lower_right())
        } else {
            None
        }
    }
}
impl std::fmt::Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inspect())
    }
}
impl Hash for Edge {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        state.write_usize(self.index);
    }
}
impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.index == other.index
    }
}
impl Eq for Edge {}
#[cfg(test)]
use crate::helpers::approx_eq;
use crate::operation::Operand;

#[test]
fn normalize_test() {
    let p0 = Point::new(-20, 20).expect("!");
    let p1 = Point::new(-10, 10).expect("!");
    let e = Edge::original(0, Operand::Subject, &p1, &p0).unwrap();
    assert_eq!(e.straight.start, p0);
    assert_eq!(e.straight.end, p1);
    let p0 = Point::new(-20, 20).expect("!");
    let p1 = Point::new(-10, 10).expect("!");
    let e = Edge::original(0, Operand::Subject, &p0, &p1).unwrap();
    assert_eq!(e.straight.start, p0);
    assert_eq!(e.straight.end, p1);
    let p0 = Point::new(0, 10).expect("!");
    let p1 = Point::new(0, 20).expect("!");
    let e = Edge::original(0, Operand::Subject, &p1, &p0).unwrap();
    assert_eq!(e.straight.start, p0);
    assert_eq!(e.straight.end, p1);
    let p0 = Point::new(0, 10).expect("!");
    let p1 = Point::new(0, 20).expect("!");
    let e = Edge::original(0, Operand::Subject, &p0, &p1).unwrap();
    assert_eq!(e.straight.start, p0);
    assert_eq!(e.straight.end, p1);
}
#[test]
fn pseudoangle_test() {
    let p0 = Point::new(10, -10).expect("!");
    let p1 = Point::new(20, -20).expect("!");
    let e = Edge::original(0, Operand::Subject, &p0, &p1).unwrap();
    assert_eq!(e.pseudoangle_for_upper_left().to_float(), 0.5);
    assert_eq!(e.pseudoangle_for_lower_right().to_float(), 2.5);
    assert_eq!(e.pseudoangle_for_pivot(&p0).unwrap().to_float(), 0.5);
    assert_eq!(e.pseudoangle_for_pivot(&p1).unwrap().to_float(), 2.5);

    let p0 = Point::new(10, 10).expect("!");
    let p1 = Point::new(20, 20).expect("!");
    let e = Edge::original(0, Operand::Clipping, &p0, &p1).unwrap();
    assert_eq!(e.pseudoangle_for_upper_left().to_float(), 1.5);
    assert_eq!(e.pseudoangle_for_lower_right().to_float(), 3.5);

    let p0 = Point::new(-10, 10).expect("!");
    let p1 = Point::new(-20, 20).expect("!");
    let e = Edge::original(0, Operand::Subject, &p0, &p1).unwrap();
    assert_eq!(e.pseudoangle_for_upper_left().to_float(), 0.5);
    assert_eq!(e.pseudoangle_for_lower_right().to_float(), 2.5);

    let p0 = Point::new(-10, -10).expect("!");
    let p1 = Point::new(-20, -20).expect("!");
    let e = Edge::original(0, Operand::Clipping, &p0, &p1).unwrap();
    assert_eq!(e.pseudoangle_for_upper_left().to_float(), 1.5);
    assert_eq!(e.pseudoangle_for_lower_right().to_float(), 3.5);
}
#[test]
fn edge_split_test() {
    let p0 = Point::new(10, -10).expect("!");
    let p1 = Point::new(20, -20).expect("!");
    let e = Edge::original(0, Operand::Subject, &p0, &p1).unwrap();

    assert!(approx_eq(e.pseudoangle_for_lower_right().to_float(), 2.5, 0.0001),
            format!("Actual: {}", &e.pseudoangle_for_lower_right().display_value()));
    assert_eq!(e.subject, 1);
    assert_eq!(e.clipping, 0);

    if let (Some(first), Some(second)) = e.left_split(
        &Point::new(15, -15).expect("!"), 10, 11
    ) {
        assert_eq!(first.index, 10);
        assert_eq!(first.subject, 1);
        assert_eq!(first.clipping, 0);
        assert_eq!(first.straight.start, Point::new(10, -10).expect("!"));
        assert_eq!(first.straight.end, Point::new(15, -15).expect("!"));
        assert_eq!(second.index, 11);
        assert_eq!(second.subject, 1);
        assert_eq!(second.clipping, 0);
        assert_eq!(second.straight.start, Point::new(15, -15).expect("!"));
        assert_eq!(second.straight.end, Point::new(20, -20).expect("!"));
    } else { panic!("Split unsuccessful"); }
}
#[test]
fn edge_count_test() {
    let p0 = Point::new(10, -10).expect("!");
    let p1 = Point::new(20, -20).expect("!");
    let e0 = Edge::new(0, 2, 3, &p0, &p1).unwrap();

    assert_eq!(e0.count(None), 5);
    assert_eq!(e0.count(Some(Operand::Subject)), 2);
    assert_eq!(e0.count(Some(Operand::Clipping)), 3);
}
#[test]
fn edge_merge_test() {
    let p0 = Point::new(10, -10).expect("!");
    let p1 = Point::new(20, -20).expect("!");
    let e0 = Edge::new(0, 2, 3, &p0, &p1).unwrap();
    let e1 = Edge::new(0, 5, 11, &p0, &p1).unwrap();

    let m = e0.merge(e1);

    assert_eq!(m.count(None), 21);
    assert_eq!(m.count(Some(Operand::Subject)), 7);
    assert_eq!(m.count(Some(Operand::Clipping)), 14);
}