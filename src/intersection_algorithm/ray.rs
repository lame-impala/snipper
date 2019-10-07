use crate::units::{Pseudoangle, Float};
use crate::primitives::Line;
use crate::edge::Edge;
use crate::{Error, Point, Coordinate};
use crate::primitives::{AbstractPoint};
use crate::intersection_algorithm::snippet::{Snippet};
use crate::intersection_algorithm::constraint::Constraint;
use crate::edge::queue::AbstractQueue;
use crate::operation::{Operand, Operation, Wrap};
use crate::drawing_algorithm::partition::Partition;


#[derive(Debug)]
pub struct Ray {
    pub angle: Pseudoangle,
    slope: Line,
    edge: Option<Edge>
}
impl Ray {
    #[allow(dead_code)]
    pub fn inspect(&self) -> String {
        format!("Ray: {:?}", self.edge())
    }
    pub fn new(edge: Edge, angle: Pseudoangle) -> Ray {
        let slope = edge.slope();
        Ray {
            angle,
            slope,
            edge: Some(edge)
        }
    }
    pub fn edge(&self) -> &Edge {
        self.edge.as_ref().expect("Empty ray not expected")
    }
    pub fn take_edge(mut self) -> Edge {
        self.edge.take().expect("Empty ray not expected")
    }
    pub fn reverse(mut self) -> Ray {
        self.angle = self.angle.reverse();
        self
    }
    pub fn count(&self, operand: Option<Operand>) -> usize {
        if let Some(edge) = self.edge.as_ref() {
            edge.count(operand)
        } else {
            0
        }
    }
    pub fn thickness(&self) -> usize {
        self.count(None)
    }
    pub fn slope(&self) -> &Line {
        &self.slope
    }
    pub fn insert(
        &mut self,
        edge: Edge,
        x: Coordinate,
        y: Float,
        queued_edges: &mut dyn AbstractQueue
    ) -> Result<Option<Snippet>, Error> {
        let point = self.endpoint().clone();
        if &point == edge.lower_right() {
            self.insert_unsafe(edge);
            Ok(None)
        } else if point.is_lower_right(&edge.lower_right()) {
            let snippet = self.snip_self(
                edge.lower_right(),
                &Constraint::LOOSE,
                Some((x, y)), queued_edges
            )?;
            self.insert_unsafe(edge);
            Ok(snippet)
        } else {
            let snippet = self.snip_edge(
                edge,
                &point,
                &Constraint::LOOSE,
                Some((x, y)), queued_edges
            )?;
            Ok(Some(snippet))
        }
    }
    pub fn insert_unsafe(&mut self, new: Edge) {
        if let Some(old) = self.edge.take() {
            let merged = old.merge(new);
            self.edge = Some(merged);
        } else {
            self.edge = Some(new);
        }
    }
    pub fn endpoint(&self) -> &Point {
        self.edge().lower_right()
    }
    pub fn is_empty(&self) -> bool {
        self.thickness() == 0
    }
    pub fn snip_self(
        &mut self,
        point: &Point,
        constraint: &Constraint,
        original_crossing: Option<(Coordinate, Float)>,
        queued_edges: &mut dyn AbstractQueue
    ) -> Result<Option<Snippet>, Error> {
        let snippet = if
        self.endpoint() == point {
            None
        } else {
            let edge = self.edge.take().expect("Empty ray not expected");
            Some(self.snip_edge(edge, point, constraint, original_crossing, queued_edges)?)
        };
        Ok(snippet)
    }
    pub fn snip_edge(
        &mut self,
        edge: Edge,
        point: &Point,
        constraint: &Constraint,
        original_crossing: Option<(Coordinate, Float)>,
        queued_edges: &mut dyn AbstractQueue
    ) -> Result<Snippet, Error> {
        let snippet = Snippet::snip(point, edge, constraint, queued_edges)?;
        let stripped = if let Some((ref left, _)) = snippet.left {
            if let Some((x, y)) = original_crossing {
                if left.pseudoangle_for_upper_left() == snippet.original_angle &&
                    left.straight.cross_with_vertical_or_upper(x).unwrap() == y {
                    let (left, snippet) = snippet.take_left();
                    self.insert_unsafe(left.unwrap().0);
                    snippet
                } else {
                    snippet
                }
            } else {
                snippet
            }
        } else {
            snippet
        };
        Ok(stripped)
    }
    pub fn yield_edge(
        &self,
        previous_partitions: (&Partition, &Partition),
        operation: &Operation,
    ) -> (Option<&Edge>, (&Partition, &Partition)) {
        let partitions = self.partitions(previous_partitions);
        let drawable = if operation.check_positions {
            let subject_drawable = self.drawable_edges(
                Operand::Subject,
                operation,
                partitions.0,
            partitions.1,
                Wrap::Outer
            );
            let clipping_drawable = self.drawable_edges(
                Operand::Clipping,
                operation,
            partitions.1,
            partitions.0,
                Wrap::Inner
            );
            subject_drawable + clipping_drawable
        } else {
            self.thickness()
        };
        let partitions = if self.angle == Pseudoangle::DOWN {
            (partitions.0.flip(), partitions.1.flip())
        } else {
            partitions
        };
        // this removes hairs
        if drawable % 2 == 0 {
            (None, partitions)
        } else {
            (Some(self.edge()), partitions)
        }
    }
    pub fn partitions(&self, previous: (&Partition, &Partition)) -> (&Partition, &Partition) {
        let subject = self.partition(Operand::Subject, previous.0);
        let clipping = self.partition(Operand::Clipping, previous.1);
        (subject, clipping)
    }
    pub fn partition(&self, operand: Operand, previous:(&Partition)) -> &Partition {
        let proper = match self.count(Some(operand)) %2 {
            0 => previous.previous_for_even(),
            1 => previous.previous_for_odd(),
            _ => panic!("Impossible")
        };
        proper
    }
    pub fn drawable_edges(
        &self,
        operand: Operand,
        operation: &Operation,
        proper_partition: &Partition,
        other_partition: &Partition,
        wrap: Wrap
    ) -> usize {
        let count = self.count(Some(operand));
        if count == 0 {
            count
        } else {
            let position = Partition::position_from_partition(
                proper_partition,
                other_partition,
                wrap
            );
            if (operation.test)(operand, position) {
                count
            } else {
                0
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::edge::{Queue, Edge};
    use crate::{AbstractPoint, Point};
    use crate::intersection_algorithm::ray::Ray;
    use crate::units::Float;
    use crate::operation::Operand;

    #[test]
    fn add_equal_edge_in_ray_test() {
        let mut gb = Queue::new();
        // should return no snippets
        let start = Point::new(0, 0).unwrap();
        let end = Point::new(10, 20).unwrap();
        let e0 = Edge::original(0, Operand::Clipping, &start, &end).unwrap();
        let e1 = Edge::original(1, Operand::Clipping, &start, &end).unwrap();
        let e2 = Edge::original(2, Operand::Clipping, &start, &end).unwrap();

        let mut r = Ray::new(e0.clone(), e0.pseudoangle);
        let x = start.x();
        let y = Float::from(start.y());
        let s = r.insert(e1, x, y, &mut gb).unwrap();
        assert!(s.is_none());
        let s = r.insert(e2, x, y, &mut gb).unwrap();
        assert!(s.is_none());
        assert_eq!(r.thickness(), 3);
    }
    #[test]
    fn add_shorter_edge_in_ray_test() {
        let mut gb = Queue::new();
        // should return right parts of the current edges
        let start = Point::new(0, 0).unwrap();
        let shorter = Point::new(5, 10).unwrap();
        let end = Point::new(10, 20).unwrap();

        let e0 = Edge::original(0, Operand::Clipping, &start, &end).unwrap();
        let e1 = Edge::original(1, Operand::Clipping, &start, &end).unwrap();
        let e2 = Edge::original(2, Operand::Clipping, &start, &shorter).unwrap();

        let mut r = Ray::new(e0.clone(), e0.pseudoangle);
        let x = start.x();
        let y = Float::from(start.y());

        let s = r.insert(e1, x, y,&mut gb).unwrap();
        assert!(s.is_none());
        let s = r.insert(e2, x, y,&mut gb).unwrap();
        assert!(s.is_some());
        assert_eq!(r.thickness(), 3);
    }
    #[test]
    fn add_longer_edge_in_ray_test() {
        let mut gb = Queue::new();
        // should return right part of the longer edge
        let start = Point::new(0, 0).unwrap();
        let longer = Point::new(20, 40).unwrap();
        let end = Point::new(10, 20).unwrap();

        let e0 = Edge::original(0, Operand::Clipping, &start, &end).unwrap();
        let e1 = Edge::original(1, Operand::Clipping, &start, &end).unwrap();
        let e2 = Edge::original(2, Operand::Clipping, &start, &longer).unwrap();

        let mut r = Ray::new(e0.clone(), e0.pseudoangle);
        let x = start.x();
        let y = Float::from(start.y());

        let s = r.insert(e1, x, y,&mut gb).unwrap();
        assert!(s.is_none());
        let s = r.insert(e2, x, y,&mut gb).unwrap();
        assert!(s.is_some());
        assert_eq!(r.thickness(), 3);
    }
}