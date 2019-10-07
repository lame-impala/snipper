use crate::intersection_algorithm::ray::Ray;
use crate::intersection_algorithm::constraint::{Constraint};
use crate::{Point};
use itertools::{Either};
use crate::intersection_algorithm::traverse::Traverse;
use crate::intersection_algorithm::support::Support;
use crate::intersection_algorithm::scope::{Rhs, Lhs};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PositionType {
    None, Support, Traverse
}
#[derive(Debug)]
pub struct Position {
    pub either: Either<Support, Traverse>
}
impl Position {
    pub fn support(support: Support) -> Position {
        Position{
            either: Either::Left(support)
        }
    }
    pub fn traverse(traverse: Traverse) -> Position {
        Position{
            either: Either::Right(traverse)
        }
    }
    pub fn in_scope(&self, rhs: &Rhs) -> bool {
        match &self.either {
            Either::Left(support) => support.in_scope(rhs),
            _ => true
        }
    }
    pub fn constraint(&self) -> &Constraint {
        match &self.either {
            Either::Right(traverse) => &traverse.constraint,
            _ => &Constraint::LOOSE
        }
    }
    pub fn has_vertical_edge(&self, lhs: &Lhs) -> bool {
        match &self.either {
            Either::Left(support) => support.has_vertical_edge(lhs),
            _ => false
        }
    }
    pub fn first_ray_mut<'scope>(&'scope mut self, rhs: &'scope mut Rhs) -> Option<&'scope mut Ray> {
        match &mut self.either {
            Either::Left(support) => support.first_ray_mut(rhs),
            Either::Right(traverse) => Some(Traverse::ray_mut(traverse.float_y(), rhs))
        }
    }
    pub fn first_ray<'scope>(&'scope self, rhs: &'scope Rhs) -> Option<&'scope Ray> {
        match &self.either {
            Either::Left(support) => support.first_ray(rhs),
            Either::Right(traverse) => Some(&Traverse::ray(traverse.float_y(), rhs))
        }
    }
    pub fn last_ray_mut<'scope>(&'scope mut self, rhs: &'scope mut Rhs) -> Option<&'scope mut Ray> {
        match &mut self.either {
            Either::Left(support) => support.last_ray_mut(rhs),
            Either::Right(traverse) => Some(Traverse::ray_mut(traverse.float_y(), rhs))
        }
    }

    pub fn last_ray<'scope>(&'scope self, rhs: &'scope Rhs) -> Option<&'scope Ray> {
        match &self.either {
            Either::Left(support) => support.last_ray(rhs),
            Either::Right(traverse) => Some(Traverse::ray(traverse.float_y(), rhs))
        }
    }
    pub fn vertical_endpoint(&self, lhs: &Lhs) -> Option<Point> {
        match &self.either {
            Either::Left(support) => {
                if let Some(vertical) = support.vertical_ray(lhs) {
                    let end = vertical.edge().lower_right();
                    Some(end.clone())
                } else {
                    None
                }
            },
            Either::Right(_) => None
        }
    }
    pub fn position_type(&self) -> PositionType {
        match &self.either {
            Either::Left(_) => PositionType::Support,
            Either::Right(_) => PositionType::Traverse
        }
    }
    #[allow(dead_code)]
    pub fn inspect(&self, lhs: &Lhs, rhs: &Rhs) -> String {
        match &self.either {
            Either::Left(support) => support.inspect(lhs, rhs),
            Either::Right(traverse) => traverse.inspect(rhs)
        }
    }
}