use crate::primitives::{
    AbstractPoint,
    Point
};
use crate::shape::{Shape, Path};
use crate::edge::{Edge};
use crate::units::Coordinate;
use crate::error::Error;
use priority_queue::PriorityQueue;
use std::cmp::Ordering;
use crate::operation::Operand;

pub trait AbstractQueue {
    fn pop_edge(&mut self, at: Coordinate) -> Option<Edge>;
    fn push_edge(&mut self, edge: Edge);
    fn next_x(&self) -> Option<Coordinate>;
    fn create_edge(&mut self, start: &Point, end: &Point, operand: Operand) -> Result<Edge, Error>;
    fn next_edge_index(&mut self) -> Result<usize, Error>;
}
#[derive(PartialEq, Eq, Debug)]
pub struct Priority {
    x: Coordinate,
    y: Coordinate
}
impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Priority) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Priority {
    fn cmp(&self, other: &Priority) -> Ordering {
        if self.x > other.x {
            Ordering::Less
        } else if self.x < other.x {
            Ordering::Greater
        } else if self.y > other.y {
            Ordering::Less
        } else if self.y < other.y {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
pub struct Queue {
    num_edges: usize,
    queue: PriorityQueue<Edge, Priority>
}
impl AbstractQueue for Queue {
    fn pop_edge(&mut self, at: Coordinate) -> Option<Edge> {
        if let Some((edge, priority)) = self.queue.peek() {
            if priority.x < at {
                panic!("Not supposed to be there at {}: {}", at, edge.inspect());
            } else if priority.x == at {
                Some(self.queue.pop().unwrap().0)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn push_edge(&mut self, edge: Edge) {
        let point = edge.upper_left();
        let priority = Priority{x: point.x(), y: point.y()};
        self.queue.push(edge, priority);
    }
    fn next_x(&self) -> Option<Coordinate> {
        if let Some((_, priority)) = self.queue.peek() {
            Some(priority.x)
        } else {
            None
        }
    }
    fn create_edge(&mut self, start: &Point, end: &Point, operand: Operand) -> Result<Edge, Error> {
        let index = self.next_edge_index()?;
        if let Some(edge) = Edge::original(index, operand, start, end) {
            Ok(edge)
        } else {
            Err(Error::NullEdgeError)
        }
    }
    fn next_edge_index(&mut self) -> Result<usize, Error> {
        if self.num_edges == std::usize::MAX {
            Err(Error::TooManyEdgesError)
        } else {
            let edge_index = self.num_edges;
            self.num_edges += 1;
            Ok(edge_index)
        }
    }
}

impl Queue {
    pub fn build<T: Shape>(subject: T, clipping: T) -> Result<Queue, Error> {
        let mut queue = Queue::new();
        queue.add_operand(subject, Operand::Subject)?;
        queue.add_operand(clipping, Operand::Clipping)?;
        Ok(queue)
    }
    pub fn num_edges(&self) -> usize {
        self.num_edges
    }
    pub fn new() -> Queue {
        Queue {
            num_edges: 0,
            queue: PriorityQueue::new()
        }
    }
    pub fn add_operand<T: Shape>(&mut self, shape: T, operand: Operand) -> Result<(), Error> {
        for path in shape.paths() {
            self.insert_path(path, operand)?;
        }
        Ok(())
    }

    fn insert_path(&mut self, path: &Path, operand: Operand) -> Result<(), Error> {
        fn next_point_is_identical(point: &Point, index: usize, points: &Vec<Point>) -> bool {
            if index + 1 < points.len() {
                point == &points[index + 1]
            } else {
                point == &points[0]
            }
        }

        fn next_point<'path>(
            last_point: (&Point, usize), points: &'path Vec<Point>
        ) -> Option<(&'path Point, usize)> {
            let mut index = last_point.1 + 1;
            let mut next: Option<&Point> = None;
            while next.is_none() && index < points.len() {
                let candidate = &points[index];
                if candidate != last_point.0 && !next_point_is_identical(candidate, index, points) {
                    next = Some(candidate);
                } else {
                    index += 1;
                }
            }
            if let Some(next) = next {
                Some((next, index))
            } else if &points[0] != last_point.0 {
                Some((&points[0], points.len()))
            } else {
                None
            }
        }

        if path.points().len() < 2 { return Ok(()); }
        let mut last_point = Some((&path.points()[0], 0));
        while let Some((next_point, next_index)) = next_point(
            last_point.unwrap(), path.points()
        ) {

            self.insert_edge(
                last_point.unwrap().0, next_point, operand
            ).expect("Unexpected null edge");
            last_point = Some((next_point, next_index));
        }
        Ok(())
    }
    pub fn insert_edge(
        &mut self,
        point: &Point,
        next_point: &Point,
        operand: Operand) -> Result<(), Error>
    {
        let edge = self.create_edge(
            point,
            next_point,
            operand
        );
        if let Ok(edge) = edge {
            self.push_edge(edge);
            Ok(())
        } else {
            Err(Error::NullEdgeError)
        }
    }
}

#[test]
fn priority_queue_test() {
    let zero = Coordinate::new(0);
    let one = Coordinate::new(1);
    let mut pq: PriorityQueue<Point, Priority> = PriorityQueue::new();
    let p0 = Point::new(0, 0).unwrap();
    let p1 = Point::new(0, 1).unwrap();
    let p2 = Point::new(1, 0).unwrap();
    pq.push(p2.clone(), Priority{x: one, y: zero});
    pq.push(p0.clone(), Priority{x: zero, y: zero});
    pq.push(p1.clone(), Priority{x: zero, y: one});
    assert_eq!(pq.peek().as_ref().unwrap().0, &p0);
}