pub mod path;
pub mod polygon;
pub mod shape;
mod triangular_matrix;
pub use path::{Path, PathDirection, PathBuilder};
pub use polygon::Polygon;
pub use shape::Shape;