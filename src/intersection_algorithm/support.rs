use std::collections::btree_map::{Range, RangeMut};
use crate::units::{Pseudoangle, Float};
use crate::edge::Edge;
use crate::intersection_algorithm::snippet::{Snippet};
use crate::intersection_algorithm::ray::Ray;
use crate::intersection_algorithm::scope::{Lhs, Rhs, Key, ReversedKey};
use crate::edge::queue::AbstractQueue;
use crate::{Error, Point, AbstractPoint};
use crate::intersection_algorithm::constraint::Constraint;
use std::ops::Bound::{Excluded, Included};

#[derive(Debug, Clone)]
pub struct Support {
    pub point: Point
}
impl Support {
    pub fn float_y(&self) -> Float {
        Float::from(self.point().y())
    }
    pub fn point(&self) -> &Point {
        &self.point
    }
    pub fn key_at(&self, angle: Pseudoangle) -> Key {
        Key::new(self.float_y(), angle)
    }
    pub fn reversed_key_at(&self, angle: Pseudoangle) -> ReversedKey {
        ReversedKey::new(self.float_y(), angle)
    }
    fn edges<'scope>(&self, lhs: &'scope Lhs, rhs: &'scope Rhs) -> Vec<&'scope Edge> {
        let up = self.key_at(Pseudoangle::UP);
        let down = self.key_at(Pseudoangle::DOWN);
        let stop = self.reversed_key_at(Pseudoangle::STOP);
        let mut all: Vec<&Edge> = rhs.range(up..down).map(|(_, ray)| ray.edge()).collect();
        let left: Vec<&Edge> = lhs.range((Excluded(stop), Included(down.reversed()))).map(|(_, ray)| ray.edge()).collect();
        all.extend(left);
        all
    }

    pub fn new(point: Point) -> Support {
        Support{ point }
    }
    pub fn inspect(&self, lhs: &Lhs, rhs: &Rhs) -> String {
        let edges = self.edges(lhs, rhs);
        let strings: Vec<String> = edges.iter().map(|edge| edge.inspect()).collect();
        format!(
            "SUPPORT at {} -- {}",
            self.point.inspect(), strings.join(", ")
        )
    }
    pub fn contains_right_hand_side_ray(&self, angle: &Pseudoangle, rhs: &Rhs) -> bool {
        debug_assert!(angle < &Pseudoangle::DOWN);
        let key = self.key_at(*angle);
        rhs.contains_key(&key)
    }
    pub fn contains_left_hand_side_ray(&self, angle: &Pseudoangle, lhs: &Lhs) -> bool {
        debug_assert!(angle > &Pseudoangle::DOWN);
        let key = self.reversed_key_at(*angle);
        lhs.contains_key(&key)
    }
    pub fn insert_vertical_ray(&self, ray: Ray, lhs: &mut Lhs) {
        debug_assert!(self.vertical_ray(lhs).is_none());
        let key = self.reversed_key_at(Pseudoangle::DOWN);
        lhs.insert(key, ray);
    }
    pub fn vertical_ray<'scope>(&self, lhs: &'scope Lhs) -> Option<&'scope Ray> {
        let key = self.reversed_key_at(Pseudoangle::DOWN);
        lhs.get(&key)
    }
    pub fn vertical_ray_mut<'scope>(&self, lhs: &'scope mut Lhs) -> Option<&'scope mut Ray> {
        let key = self.reversed_key_at(Pseudoangle::DOWN);
        lhs.get_mut(&key)
    }
    pub fn take_vertical_ray(&self, lhs: &mut Lhs) -> Option<Ray> {
        lhs.remove(&self.reversed_key_at(Pseudoangle::DOWN))
    }
    pub fn has_vertical_edge(&self, lhs: &Lhs) -> bool {
        lhs.contains_key(&self.reversed_key_at(Pseudoangle::DOWN))
    }
    pub fn right_hand_side<'scope>(&self, rhs: &'scope Rhs) -> Range<'scope, Key, Ray> {
        let upper = self.key_at(Pseudoangle::UP);
        let lower = self.key_at(Pseudoangle::DOWN);
        rhs.range((Excluded(upper), Excluded(lower)))
    }
    pub fn right_hand_side_mut<'scope>(&self, rhs: &'scope mut Rhs) -> RangeMut<'scope, Key, Ray> {
        let upper = self.key_at(Pseudoangle::UP);
        let lower = self.key_at(Pseudoangle::DOWN);
        rhs.range_mut((Excluded(upper), Excluded(lower)))
    }
    pub fn remove_from_right(&self, angle: &Pseudoangle, rhs: &mut Rhs) -> Option<Ray> {
        debug_assert!(angle > &Pseudoangle::UP && angle < &Pseudoangle::DOWN);
        let key = self.key_at(*angle);
        rhs.remove(&key)
    }
    #[cfg(test)]
    pub fn left_hand_side<'scope>(&self, lhs: &'scope Lhs) -> Range<'scope, ReversedKey, Ray> {
        let stop = self.reversed_key_at(Pseudoangle::STOP);
        let down = self.reversed_key_at(Pseudoangle::DOWN);
        lhs.range((Excluded(stop), Excluded(down)))
    }

    pub fn in_scope(&self, rhs: &Rhs) -> bool {
        self.right_hand_side(rhs).nth(0).is_some()
    }
    pub fn insert(&self, edge: Edge, queued_edges: &mut dyn AbstractQueue, lhs: &mut Lhs, rhs: &mut Rhs) -> Result<(Option<Snippet>, bool, bool, bool), Error> {
        let start =  edge.upper_left();
        let end = edge.lower_right();
        if start.x() < self.point.x() {
            if end.x() > self.point.x() {
                let snippet= self.insert_traverse(edge, queued_edges, lhs)?;
                Ok((snippet, false, false, false))
            } else if end.x() == self.point.x() {
                self.insert_to_left(edge, lhs);
                Ok((None, false, false, false))
            } else {
                panic!("Edge end is to the left of the sweep line");
            }
        } else if start.x() == self.point.x() {
            if end.x() == self.point.x() {
                self.insert_to_vertical(edge, queued_edges, lhs)
            } else {
                self.insert_to_right(edge, queued_edges, rhs)
            }
        } else {
            panic!("Edge start is to the right of the sweep line");
        }
    }
    pub fn insert_traverse(&self, edge: Edge, queued_edges: &mut dyn AbstractQueue, lhs: &mut Lhs) -> Result<Option<Snippet>, Error> {
        let point: &Point = self.point();
        let snippet = Snippet::snip(point, edge, &Constraint::LOOSE, queued_edges)?;
        let (left, snippet) = snippet.take_left();
        self.insert_to_left(left.unwrap().0, lhs);
        Ok(Some(snippet))
    }
    pub fn insert_ray_to_left(&self, angle: Pseudoangle, ray: Ray, lhs: &mut Lhs) {
        debug_assert!(angle > Pseudoangle::DOWN);
        let key = self.key_at(angle).reversed();
        lhs.insert(key, ray);
    }
    pub fn insert_to_left(&self, edge: Edge, lhs: &mut Lhs) {
        debug_assert!(edge.upper_left().x() < self.point().x(), "Edge doesn't start left to the position");
        debug_assert!(edge.lower_right().x() == self.point().x(), "Edge doesn't end at the position");
        let angle = edge.pseudoangle_for_pivot(&self.point).unwrap();
        let key = self.key_at(angle).reversed();
        if let Some(ray) = lhs.get_mut(&key) {
            ray.insert_unsafe(edge);
        } else {
            let ray = Ray::new(edge, angle);
            self.insert_ray_to_left(angle, ray, lhs);
        }
    }
    pub fn insert_to_right(
        &self,
        edge: Edge,
        queued_edges: &mut dyn AbstractQueue,
        rhs: &mut Rhs
    ) -> Result<(Option<Snippet>, bool, bool, bool), Error>{
        debug_assert!(edge.upper_left() == self.point(), "Edge doesn't start at the position");
        debug_assert!(edge.lower_right().x() > self.point().x(), "Edge is vertical");
        let angle = edge.pseudoangle_for_upper_left();
        let x = self.point().x();
        let y = self.float_y();
        let key = self.key_at(angle);
        if self.contains_right_hand_side_ray(&angle, rhs) {
            let snippet = rhs.get_mut(&key).unwrap().insert(edge, x, y, queued_edges)?;
            // Resulting ray can only be shorter or equal as before
            // so that it can never get dirty
            Ok((snippet, false, false, false))
        } else {
            let ray = Ray::new(edge, angle);
            let top_dirty = self.first_ray(rhs).is_none() || self.first_ray(rhs).unwrap().angle > angle;
            let bottom_dirty = self.last_ray(rhs).is_none() || self.last_ray(rhs).unwrap().angle < angle;
            rhs.insert(key, ray);
            Ok((None, top_dirty, bottom_dirty, false))
        }
    }
    fn insert_to_vertical(
        &self,
        edge: Edge,
        queued_edges: &mut dyn AbstractQueue,
        lhs: &mut Lhs
    ) -> Result<(Option<Snippet>, bool, bool, bool), Error> {
        debug_assert!(edge.upper_left() == self.point(), "Edge doesn't start at the position");
        debug_assert!(edge.lower_right().x() == self.point().x(), "Edge is not vertical");
        let x = self.point().x();
        let y = self.float_y();
        if let Some(ray) = self.vertical_ray_mut(lhs) {
            let snippet = ray.insert(edge, x, y, queued_edges)?;
            Ok((snippet, false, false, false))
        } else {
            let ray = Ray::new(edge, Pseudoangle::DOWN);
            self.insert_vertical_ray(ray, lhs);
            Ok((None, false, false, true))
        }
    }

    pub fn first_ray_mut<'scope>(&self, rhs: &'scope mut Rhs) -> Option<&'scope mut Ray> {
        if let Some((_, ray)) = self.right_hand_side_mut(rhs).nth(0) {
            Some(ray)
        } else {
            None
        }
    }
    pub fn last_ray_mut<'scope>(&self, rhs: &'scope mut Rhs) -> Option<&'scope mut Ray> {
        if let Some((_, ray)) = self.right_hand_side_mut(rhs).last() {
            Some(ray)
        } else {
            None
        }
    }
    pub fn first_ray<'scope>(&self, rhs: &'scope Rhs) -> Option<&'scope Ray> {
        if let Some((_, ray)) = self.right_hand_side(rhs).nth(0) {
            Some(ray)
        } else {
            None
        }
    }
    pub fn last_ray<'scope>(&self, rhs: &'scope Rhs) -> Option<&'scope Ray> {
        if let Some((_, ray)) = self.right_hand_side(rhs).last() {
            Some(ray)
        } else {
            None
        }
    }
}
#[cfg(test)]
mod test {
    use crate::units::{Pseudoangle, Float};
    use crate::{Point};
    use crate::edge::{Queue, Edge};
    use crate::intersection_algorithm::support::Support;
    use crate::operation::Operand;
    use crate::intersection_algorithm::scope::{Lhs, Rhs};

    #[test]
    fn left_side_test() {
        let mut lhs = Lhs::new();
        let point = Point::new(10, 5).unwrap();
        let support = Support::new(point.clone());
        let s0 = Point::new(0, 0).unwrap();
        let s1 = Point::new(0, 5).unwrap();
        let s2 = Point::new(0, 10).unwrap();
        let e3 = Point::new(10, 10).unwrap();
        let edge0 = Edge::original(0, Operand::Clipping, &s0, &point).unwrap();
        let edge1 = Edge::original(1, Operand::Clipping, &s1, &point).unwrap();
        let edge2 = Edge::original(2, Operand::Clipping, &s2, &point).unwrap();
        let edge3 = Edge::original(3, Operand::Clipping, &point, &e3).unwrap();
        let edge4 = Edge::original(4, Operand::Clipping, &s2, &point).unwrap();
        support.insert_to_left(edge0.clone(), &mut lhs);
        support.insert_to_left(edge1.clone(), &mut lhs);
        support.insert_to_left(edge2.clone(), &mut lhs);
        let mut queue = Queue::new();
        let _ = support.insert_to_vertical(edge3.clone(), &mut queue, &mut lhs);
        support.insert_to_left(edge4.clone(), &mut lhs);
        assert_eq!(support.left_hand_side(&lhs).count(), 3);
    }
    #[test]
    fn support_basics_test() {
        let mut lhs = Lhs::new();
        let mut rhs = Rhs::new();

        let point = Point::new(10, 5).unwrap();
        let support = Support::new(point);
        assert_eq!(support.in_scope(&rhs), false);
        assert_eq!(support.float_y(), Float::new(5.0).unwrap());

        let mut gb = Queue::new();

        let s0 = Point::new(0, 5).unwrap();
        let e0 = Point::new(10, 5).unwrap();
        let e0 = Edge::original(0, Operand::Clipping, &s0, &e0).unwrap();
        support.insert_to_left(e0, &mut lhs);
        assert_eq!(support.in_scope(&rhs), false);
        assert!(support.first_ray(&rhs).is_none());
        assert!(support.last_ray(&rhs).is_none());


        let s1 = Point::new(10, 5).unwrap();
        let e1 = Point::new(15, 5).unwrap();
        let e1 = Edge::original(0, Operand::Clipping, &s1, &e1).unwrap();
        let a1 = e1.pseudoangle_for_upper_left();
        let _ = support.insert_to_right(e1, &mut gb, &mut rhs);
        assert_eq!(support.right_hand_side(&rhs).count(), 1);
        assert_eq!(support.in_scope(&rhs), true);
        assert_eq!(support.first_ray(&rhs).unwrap().angle, a1);
        assert_eq!(support.last_ray(&rhs).unwrap().angle, a1);

        let s2 = Point::new(10, 5).unwrap();
        let e2 = Point::new(15, 0).unwrap();
        let e2 = Edge::original(0, Operand::Clipping, &s2, &e2).unwrap();
        let a2 = e2.pseudoangle_for_upper_left();
        let _ = support.insert_to_right(e2, &mut gb, &mut rhs);
        assert_eq!(support.right_hand_side(&rhs).count(), 2);
        assert_eq!(support.in_scope(&rhs), true);
        assert_eq!(support.first_ray(&rhs).unwrap().angle, a2);
        assert_eq!(support.last_ray(&rhs).unwrap().angle, a1);


        let s3 = Point::new(10, 5).unwrap();
        let e3 = Point::new(15, 10).unwrap();
        let e3 = Edge::original(0, Operand::Clipping, &s3, &e3).unwrap();
        let a3 = e3.pseudoangle_for_upper_left();
        let _ = support.insert_to_right(e3, &mut gb, &mut rhs);
        assert_eq!(support.right_hand_side(&rhs).count(), 3);
        assert_eq!(support.in_scope(&rhs), true);
        assert_eq!(support.first_ray(&rhs).unwrap().angle, a2);
        assert_eq!(support.last_ray(&rhs).unwrap().angle, a3);

        assert!(support.remove_from_right(&Pseudoangle::new(0.25), &mut rhs).is_none());
        let r0 = support.remove_from_right(&a1, &mut rhs).unwrap();
        assert_eq!(r0.angle, Pseudoangle::new(1.0));
        let r1 = support.remove_from_right(&a2, &mut rhs).unwrap();
        assert_eq!(r1.angle, Pseudoangle::new(0.5));
        let r2 = support.remove_from_right(&a3, &mut rhs).unwrap();
        assert_eq!(r2.angle, Pseudoangle::new(1.5));

        assert_eq!(support.in_scope(&rhs), false);
    }

    #[test]
    fn insert_edges_to_support() {
        let mut rhs = Rhs::new();

        let point = Point::new(0, 0).unwrap();
        let support = Support::new(point.clone());
        let end0 = Point::new(20, 10).unwrap();
        let end1 = Point::new(30, 15).unwrap();
        let end2 = Point::new(10, 5).unwrap();
        let edge0 = Edge::original(0, Operand::Clipping, &point, &end0).unwrap();
        let edge1 = Edge::original(1, Operand::Clipping, &point, &end1).unwrap();
        let edge2 = Edge::original(2, Operand::Clipping, &point, &end2).unwrap();

        let mut gb = Queue::new();
        let (snippet, td, bd, vd) = support.insert_to_right(edge0, &mut gb, &mut rhs).unwrap();
        assert!(td, "Should be top-dirty");
        assert!(bd, "Should be bottom-dirty");
        assert!(!vd, "Shouldn't be vertical-dirty");
        assert_eq!(support.right_hand_side(&rhs).count(), 1);
        assert!(snippet.is_none());
        let (snippet, td, bd, vd) = support.insert_to_right(edge1, &mut gb, &mut rhs).unwrap();
        assert!(!td, "Shouldn't be top-dirty");
        assert!(!bd, "Shouldn't be bottom-dirty");
        assert!(!vd, "Shouldn't be vertical-dirty");
        assert_eq!(support.right_hand_side(&rhs).count(), 1);
        assert!(snippet.is_some());
        assert_eq!(snippet.as_ref().unwrap().right.as_ref().unwrap().upper_left(), &end0);
        assert_eq!(snippet.as_ref().unwrap().right.as_ref().unwrap().lower_right(), &end1);
        let (snippet, td, bd, vd) = support.insert_to_right(edge2, &mut gb, &mut rhs).unwrap();
        assert!(!td, "Shouldn't be top-dirty");
        assert!(!bd, "Shouldn't be bottom-dirty");
        assert!(!vd, "Shouldn't be vertical-dirty");
        assert_eq!(support.right_hand_side(&rhs).count(), 1);
        assert!(snippet.is_some());
        assert_eq!(snippet.as_ref().unwrap().right.as_ref().unwrap().upper_left(), &end2);
        assert_eq!(snippet.as_ref().unwrap().right.as_ref().unwrap().lower_right(), &end0);
    }
}