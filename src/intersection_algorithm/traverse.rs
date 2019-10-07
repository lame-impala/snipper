use crate::units::{Pseudoangle, Float};
use crate::edge::Edge;
use crate::{Coordinate, Point};
use crate::edge::queue::AbstractQueue;
use crate::intersection_algorithm::constraint::Constraint;
use crate::intersection_algorithm::ray::Ray;
use crate::intersection_algorithm::scope::{Rhs, Key};
use std::ops::Range;

#[derive(Debug)]
pub struct Traverse {
    y: Float,
    pub constraint: Constraint
}
impl Traverse {
    pub fn new(y: Float, constraint: Constraint) -> Traverse {
        Traverse{ y, constraint }
    }
    pub fn float_y(&self) -> Float { self.y }
    pub fn range_key(y: Float) -> Range<Key> {
        let upper = Key::new(y, Pseudoangle::UP);
        let lower = Key::new(y, Pseudoangle::DOWN);
        upper..lower
    }
    pub fn edge(y: Float, rhs: &Rhs) -> &Edge {
        let ray = Traverse::ray(y, rhs);
        ray.edge()
    }
    pub fn key(&self, rhs: &Rhs) -> Key {
        let mut range = rhs.range(Traverse::range_key(self.float_y()));
        let (key, _) = range.nth(0).unwrap();
        debug_assert!(range.count() == 0, "Traverse should have exactly one ray");
        *key
    }
    pub fn ray(y: Float, rhs: &Rhs) -> &Ray {
        let mut range = rhs.range(Traverse::range_key(y));
        let (_, ray) = range.nth(0).expect("Traverse should have at least one ray");
        debug_assert!(range.count() == 0, "Traverse should have exactly one ray");
        ray
    }
    pub fn ray_mut(y: Float, rhs: &mut Rhs) -> &mut Ray {
        let mut range = rhs.range_mut(Traverse::range_key(y));
        let (_, ray) = range.nth(0).unwrap();
        debug_assert!(range.count() == 0, "Traverse should have exactly one ray");
        ray
    }
    pub fn can_take(&self, edge: &Edge, ray: &Ray) -> bool {
        ray.edge().upper_left() == edge.upper_left() &&
            ray.edge().lower_right() == edge.lower_right()
    }
    pub fn preferred_point(&self, x: &Coordinate) -> Point {
        let y = Traverse::preferred_position(self.y);
        Point::unchecked(*x, y)
    }
    pub fn preferred_position(y: Float) -> Coordinate {
        let y = f64::from(y);
        let decimal_part = y - y.floor();
        if decimal_part < 0.5 {
            Coordinate::from_float(y.floor()).unwrap()
        } else {
            Coordinate::from_float(y.ceil()).unwrap()
        }
    }
    pub fn insert(&mut self, edge: Edge, x: Coordinate, queued_edges: &mut dyn AbstractQueue, ray: &mut Ray) {
        debug_assert!(
            self.can_take(&edge, ray),
            format!(
                "At {}: edge expected to be identical to those already there {} vs. {}",
                self.y,
                edge.inspect(),
                ray.edge().inspect()
            )
        );
        let snippet = ray
            .insert(edge, x, self.y, queued_edges)
            .expect("Insertion should be infallible");
        debug_assert!(snippet.is_none(), "Insertion expected to produce zero snippets");
    }
    pub fn inspect(&self, rhs: &Rhs) -> String {
        let ray = Traverse::ray(self.float_y(), rhs);
        format!(
            "TRAVERSE at {}, thickness: {} -- {}",
            self.y, ray.thickness(), ray.edge().inspect()
        )
    }
}
