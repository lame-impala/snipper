use crate::units::{Coordinate, Float};
use crate::{Error, Shape, Queue};
use crate::edge::{Edge};
use crate::edge::queue::{AbstractQueue};
use std::collections::btree_map::BTreeMap;
use crate::intersection_algorithm::ray::Ray;
pub use scope::Scope;
use crate::intersection_algorithm::scope::Key;
use crate::intersection_algorithm::position::Position;


pub struct IntersectionAlgorithm {}
impl IntersectionAlgorithm {
    pub fn perform<T: Shape>(a: T, b: T) -> Result<Vec<Edge>, Error> {
        let mut queued_edges = Queue::build(a, b)?;

        let mut left: BTreeMap<Key, Ray> = BTreeMap::new();
        let mut positions: BTreeMap<Float, Position> = BTreeMap::new();
        let mut next: Option<Coordinate> = queued_edges.next_x();
        let mut vec: Vec<Edge> = Vec::new();
        while let Some(x) = next {
            let scope = Scope::build(left, positions, &x, &mut queued_edges)?;
            let edges: Vec<Edge> = scope.left_hand_edges();
            vec.extend(edges);
            let (new_left, new_positions, next_scope) = scope.pass_over();
            left = new_left;
            positions = new_positions;
            let next_batch = queued_edges.next_x();
            next = match (next_scope, next_batch) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            };
        }
        Ok(vec)
    }
}
pub mod scope;
pub mod traverse;
pub mod support;
pub mod position;
pub mod ray;
pub mod snippet;
pub mod constraint;
pub mod stack;
pub mod dirty_records;
pub mod bentley_ottmann;
#[cfg(test)]
mod test;