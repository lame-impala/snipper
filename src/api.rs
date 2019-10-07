use super::shape::Polygon;
use crate::shape::{Path, Shape};
use super::error::Error;
use crate::operation::{Operation};
use crate::intersection_algorithm::{Scope};
use crate::{Queue, Coordinate, Bounds};
use std::collections::btree_map::BTreeMap;
use crate::units::Float;
use crate::intersection_algorithm::position::Position;
use crate::intersection_algorithm::scope::Key;
use crate::intersection_algorithm::ray::Ray;
use crate::edge::queue::AbstractQueue;
use crate::shape::polygon::{Comparator, Relation};
use crate::drawing_algorithm::drawing_algorithm::DrawingAlgorithm;
use crate::drawing_algorithm::partition::Partition;
use crate::drawing_algorithm::builder::BuilderIndex;
use crate::drawing_algorithm::routes::FirstIndex;

pub struct Solution {
    data: Option<(Vec<Path>, PathComparator)>
}
impl Solution {
    fn new(data: Option<(Vec<Path>, PathComparator)>) -> Solution {
        Solution { data }
    }
    pub fn paths(self) -> Vec<Path> {
        if let Some((paths, _)) = self.data {
            paths
        } else {
            Vec::new()
        }
    }
    pub fn polygon(self) -> Result<Polygon, Error> {
        if let Some((paths, mut comparator)) = self.data {
            Polygon::build(paths, &mut comparator)
        } else {
            unsafe { Polygon::flat(vec![]) }
        }
    }
}

pub struct Snipper {}
impl Snipper {
    pub fn perform_operation<T: Shape>(
        subject: T,
        clipping: T,
        operation: &'static Operation
    ) -> Result<Solution, Error> {
        let mut queue = Queue::build(subject, clipping)?;
        let mut next: Option<Coordinate> = queue.next_x();
        let mut left: BTreeMap<Key, Ray> = BTreeMap::new();
        let mut positions: BTreeMap<Float, Position> = BTreeMap::new();
        if let Some(x) = next {
            let mut state = DrawingAlgorithm::initial_state(x);
            while let Some(x) = next {
                let scope = Scope::build(left, positions, &x, &mut queue)?;
                let mut partitions = (&Partition::OUT_OUT, &Partition::OUT_OUT);
                for ray in scope.iter() {
                    let tuple = ray.yield_edge(partitions, operation);
                    partitions = tuple.1;
                    if let Some(edge) = tuple.0 {
                        state.draw_edge(edge);
                    }
                }
                let (new_left, new_positions, next_scope) = scope.pass_over();
                left = new_left;
                positions = new_positions;
                let next_batch = queue.next_x();
                next = match (next_scope, next_batch) {
                    (Some(a), Some(b)) => Some(a.min(b)),
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (None, None) => None,
                };
                if let Some(next_x) = next {
                    state = state.next_state(next_x);
                }
            }
            let pic = state.terminate_all();
            let (paths, routes) = pic.build_paths();
            let comparator = PathComparator {
                routes
            };
            Ok(Solution::new(Some((paths, comparator))))

        } else {
            Ok(Solution::new(None))
        }
    }

    pub fn union(subject: Polygon, clipping: Polygon) -> Result<Solution, Error> {
        Snipper::perform_operation(subject, clipping, &Operation::UNION)
    }
    pub fn intersection(subject: Polygon, clipping: Polygon) -> Result<Solution, Error> {
        Snipper::perform_operation(subject, clipping, &Operation::INTERSECTION)
    }
    pub fn xor(subject: Polygon, clipping: Polygon) -> Result<Solution, Error> {
        Snipper::perform_operation(subject, clipping, &Operation::XOR)
    }
    pub fn difference(minuend: Polygon, subtrahend: Polygon) -> Result<Solution, Error> {
        Snipper::perform_operation(minuend, subtrahend, &Operation::DIFFERENCE)
    }
    pub fn normalize(paths: Vec<Path>) -> Result<Solution, Error> {
        let non_normal = unsafe { Polygon::flat(paths)? };
        let null = unsafe { Polygon::flat(vec![])? };
        Snipper::perform_operation(non_normal, null, &Operation::XOR)
    }
}
pub struct PathComparator {
    routes: crate::drawing_algorithm::routes::Routes
}
impl PathComparator {
    pub fn count_chains_above(&mut self, a: usize, b: usize) -> usize {
        let mut first_index = self.routes.first(&a);
        let before_first = self.routes.structure().get(first_index.expect("Expected to be there"));
        debug_assert!(before_first.is_some(), "Index expected to be there");
        let mut count = 0;
        let mut option = before_first;
        while let Some(previous) = option {
            if let BuilderIndex::Some(index) = previous {
                let chain_index = previous.unwrap();
                if self.routes.belongs_to_path(&chain_index, &b) {
                    count += 1;
                } else if let FirstIndex::Random(index) = first_index {
                    if self.routes.belongs_to_path(&chain_index, &a) {
                        first_index = FirstIndex::Random(index);
                    }
                }
                option = Some(self.routes.structure().get(*index).expect("Previous index expected to be there"));
            } else {
                option = None;
            }
        }
        self.routes.confirm_first(first_index.expect("Expected to be there"), &a);
        count
    }
}
impl Comparator for PathComparator {
    fn compare(&mut self, a: &Path, b: &Path, a_index: usize, b_index: usize) -> Relation {
        if Bounds::have_collision(a.bounds().unwrap(), b.bounds().unwrap()) {
            let a_cnt = self.count_chains_above(a_index, b_index) % 2;
            let b_cnt = self.count_chains_above(b_index, a_index) % 2;
            debug_assert!(!(a_cnt != 0 && b_cnt != 0), "Both paths can't have odd number of neighbors");
            if a_cnt == 1 { return Relation::Contained; }
            if b_cnt == 1 { return Relation::Contains; }
            Relation::Unrelated
        } else {
            Relation::Unrelated
        }
    }
}
