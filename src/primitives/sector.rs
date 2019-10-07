use crate::Point;

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Mode {
    Closed,
    Open
}
pub trait Sector {
    fn contains(&self, point: &Point, mode: &Mode) -> bool;
}
