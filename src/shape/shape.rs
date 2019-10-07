use crate::primitives::{Point, Position, Mode, Bounds, Sector};
use super::path::Path;

pub trait Shape {
    fn position(&self, point: &Point) -> Position;
    fn bounds(&self) -> Option<&Bounds>;
    fn paths(&self) -> Vec<&Path>;
}
impl <T: Shape> Sector for T {
    fn contains(&self, point: &Point, mode: &Mode) -> bool {
        match self.bounds() {
            None => return false,
            Some(ref bounds) => {
                if !bounds.contains(point, mode) {
                    return false
                }
            }
        }
        let position = self.position(point);
        match position {
            Position::In => true,
            Position::Out => false,
            Position::Edge => *mode == Mode::Closed,
            Position::Unknown => panic!("Definite position expected")
        }
    }
}