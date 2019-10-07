mod units;
mod error;
mod edge;
mod primitives;
mod shape;
mod intersection_algorithm;
mod operation;
mod api;
mod drawing_algorithm;
mod helpers;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
mod test;

pub use api::{Snipper, Solution};
pub use primitives::{AbstractPoint, Point, Bounds};
pub use shape::{Shape, Path, Polygon, PathBuilder};
pub use error::Error;
pub use units::Coordinate;
pub use edge::Queue;
pub use intersection_algorithm::IntersectionAlgorithm;
