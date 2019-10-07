use crate::primitives::{Straight, Mode, Line, Point};
use crate::primitives::sector::Sector;

pub struct Intersection {}
impl Intersection {
    pub fn proper(s1: &Straight, s2: &Straight) -> Option<Point> {
        debug_assert!(!s1.contains_proper(&s2.start), "Segments not expected to intersect at start");
        debug_assert!(!s2.contains_proper(&s1.start), "Segments not expected to intersect at start");
        if s1.contains_proper(&s2.end) {
            debug_assert!(!s2.contains_proper(&s1.end), "Sengments not expected to overlap");
            Some(s2.end.clone())
        } else if s2.contains_proper(&s1.end) {
            debug_assert!(!s1.contains_proper(&s2.end), "Sengments not expected to overlap");
            Some(s1.end.clone())
        } else if Straight::may_cross(s1, s2) {
            let intersection = Intersection::intersection(s1, s2);
            if let Some(candidate) = intersection {
                if !s1.is_endpoint(&candidate) || !s2.is_endpoint(&candidate) {
                    Some(candidate)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    fn intersection(s1: &Straight, s2: &Straight) -> Option<Point> {
        let l1 = s1.to_line();
        let l2 = s2.to_line();
        let intersection = Line::intersection(&l1, &l2);
        if let Some(point) = intersection {
            if s1.bounds.contains(&point, &Mode::Closed) &&
                s2.bounds.contains(&point, &Mode::Closed) {
                Some(point)
            } else {
                None
            }
        } else {
            None
        }
    }
}