use crate::units::Pseudoangle;
use crate::{Point, Coordinate};
use crate::primitives::{AbstractPoint, Straight};
use crate::intersection_algorithm::scope::Direction;

#[derive(Copy, Clone, Debug)]
pub struct Constraint {
    min: Pseudoangle,
    max: Pseudoangle
}
impl Constraint {
    pub const LOOSE: Constraint = Constraint{ min: Pseudoangle::UP, max: Pseudoangle::DOWN };
    pub const VERTICAL: Constraint = Constraint{ min: Pseudoangle::DOWN, max: Pseudoangle::DOWN };

    pub fn merge(a: &Constraint, b: &Constraint) -> Constraint {
        Constraint {
            min: a.min.max(b.min),
            max: a.max.min(b.max)
        }
    }
    pub fn constrain_direction(&self, angle: &Pseudoangle, dir: Direction) -> Constraint {
        match dir {
            Direction::Upwards => self.constrain_minimum(angle),
            Direction::Downwards => self.constrain_maximum(angle)
        }
    }
    pub fn constrain_minimum(&self, angle: &Pseudoangle) -> Constraint {
        if &self.min > angle {
            self.clone()
        } else {
            Constraint{ min: *angle, max: self.max }
        }
    }
    pub fn constrain_maximum(&self, angle: &Pseudoangle) -> Constraint {
        if &self.max < angle {
            self.clone()
        } else {
            Constraint{ min: self.min, max: *angle }
        }
    }
    pub fn y_at_pseudoangle(angle: &Pseudoangle, start: &Point, end_x: &Coordinate) -> f64 {
        let float_x = f64::from(end_x.to_int());
        let start_x = start.float_x();
        let delta_x = float_x - start_x;
        debug_assert!(delta_x >= 0.0, "Expected new point not to be to the left");
        if delta_x == 0.0 {
            start.float_y()
        } else {
            let start_y = start.float_y();
            let end_y = if angle == &Pseudoangle::ONE {
                start_y
            } else {
                let p = if angle < &Pseudoangle::ONE {
                    angle.to_float()
                } else {
                    2.0 - angle.to_float()
                };
                if p == 0.0 {
                    if angle < &Pseudoangle::ONE {
                        std::f64::NEG_INFINITY
                    } else {
                        std::f64::INFINITY
                    }
                } else {
                    let abs_y = (delta_x - (p * delta_x)) / p;
                    let delta_y = if angle < &Pseudoangle::ONE {
                        -abs_y
                    } else {
                        abs_y
                    };
                    start_y + delta_y
                }
            };
            end_y
        }

    }
    pub fn upper_allowance(&self, upper_left: &Point, angle: &Pseudoangle, at: &Coordinate) -> f64 {
        if &self.min > angle {
            0.0
        } else {
            Constraint::allowance(upper_left, angle, at, &self.min)
        }
    }
    pub fn lower_allowance(&self, upper_left: &Point, angle: &Pseudoangle, at: &Coordinate) -> f64 {
        if &self.max < angle {
            0.0
        } else {
            Constraint::allowance(upper_left, angle, at, &self.max)
        }
    }
    pub fn allowance(upper_left: &Point, angle: &Pseudoangle, at: &Coordinate, limit: &Pseudoangle) -> f64 {
        let cross_point = Constraint::y_at_pseudoangle(
            angle,
            upper_left,
            at
        );
        let other_point = Constraint::y_at_pseudoangle(
            limit,
            upper_left,
            at
        );
        if other_point.is_finite() {
            (cross_point - other_point).abs()
        } else {
            std::f64::INFINITY
        }
    }
    pub fn is_too_tight(&self, straight: &Straight, angle: &Pseudoangle, at: &Coordinate) -> bool {
        if at == &straight.upper_left().x() {
            false // edge starts here so it has total freedom
        } else if straight.start.y() == straight.end.y() {
            false // horizontals never move
        } else if self.upper_allowance(straight.upper_left(), angle, at) < 1.001 {
            true
        } else if self.lower_allowance(straight.upper_left(), angle, at) < 1.001 {
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn inspect(&self) -> String {
        format!("Constraint -- min: {}, max: {}", self.min.display_value(), self.max.display_value())
    }
    pub fn allows(&self, straight: &Straight, angle: &Pseudoangle, at: &Coordinate) -> bool {
        if angle > &self.min && angle < &self.max {
            // Constraint is not stepped over
            // but we need to be sure there's enough allowance
            !self.is_too_tight(straight, angle, at)
        } else {
            false
        }
    }
}
#[cfg(test)]
mod test {
    use crate::primitives::{Vector};
    use crate::intersection_algorithm::constraint::Constraint;
    use crate::{Point, Coordinate};
    use crate::edge::Edge;
    use crate::units::Pseudoangle;
    use crate::operation::Operand;

    #[test]
    fn merge_test() {
        let a = Constraint{min: Pseudoangle::new(0.2), max: Pseudoangle::new(0.6)};
        let b = Constraint{min: Pseudoangle::new(0.4), max: Pseudoangle::new(0.8)};
        let m = Constraint::merge(&a, &b);
        assert_eq!(m.min, Pseudoangle::new(0.4));
        assert_eq!(m.max, Pseudoangle::new(0.6));
        let m = Constraint::merge(&b, &a);
        assert_eq!(m.min, Pseudoangle::new(0.4));
        assert_eq!(m.max, Pseudoangle::new(0.6));
    }
    #[test]
    fn y_at_pseudoangle_test() {
        let v0 = Vector::from(&Point::new(3, 2).unwrap());
        let pa0 = v0.pseudoangle().unwrap();
        let s0 = Point::new(0, 0).unwrap();
        let s1 = Point::new(3, -1).unwrap();
        assert_eq!(Constraint::y_at_pseudoangle(&pa0, &s0, &v0.x()), 1.9999999999999993);
        assert_eq!(Constraint::y_at_pseudoangle(&pa0, &s0, &Coordinate::new(6)), 3.9999999999999987);
        assert_eq!(Constraint::y_at_pseudoangle(&pa0, &s1, &Coordinate::new(6)), 0.9999999999999993);

        let v0 = Vector::from(&Point::new(3, -2).unwrap());
        let pa0 = v0.pseudoangle().unwrap();
        let s0 = Point::new(0, 0).unwrap();
        let s1 = Point::new(3, -1).unwrap();
        assert_eq!(Constraint::y_at_pseudoangle(&pa0, &s0, &v0.x()), -2.0000000000000004);
        assert_eq!(Constraint::y_at_pseudoangle(&pa0, &s0, &Coordinate::new(6)), -4.000000000000001);
        assert_eq!(Constraint::y_at_pseudoangle(&pa0, &s1, &Coordinate::new(6)), -3.0000000000000004);

        let v0 = Vector::from(&Point::new(3, 0).unwrap());
        let pa0 = v0.pseudoangle().unwrap();
        let s0 = Point::new(0, 0).unwrap();
        let s1 = Point::new(3, -1).unwrap();
        assert_eq!(Constraint::y_at_pseudoangle(&pa0, &s0, &v0.x()), 0.0);
        assert_eq!(Constraint::y_at_pseudoangle(&pa0, &s0, &Coordinate::new(6)), 0.0);
        assert_eq!(Constraint::y_at_pseudoangle(&pa0, &s1, &Coordinate::new(6)), -1.0);
    }
    #[test]
    fn allowance_test() {
        let e0 = Edge::original(0, Operand::Subject,
            &Point::new(1, 1).unwrap(),
            &Point::new(4, -1).unwrap()
        ).unwrap();
        let a0 = Vector::from(&Point::new(3, -1).unwrap()).pseudoangle().unwrap();
        let a1 = Vector::from(&Point::new(3, -2).unwrap()).pseudoangle().unwrap();
        let a2 = Vector::from(&Point::new(3, -3).unwrap()).pseudoangle().unwrap();
        assert_eq!(Constraint::allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &Coordinate::new(5), &a0),
           1.3333333333333337
        );
        assert_eq!(Constraint::allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &Coordinate::new(5), &a1),
             0.0
        );
        assert_eq!(Constraint::allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &Coordinate::new(5), &a2),
             1.333333333333333
        );
    }
    #[test]
    fn constrain_test() {
        let e0 = Edge::original(0, Operand::Subject,
            &Point::new(1, 1).unwrap(),
            &Point::new(4, -1).unwrap()
        ).unwrap();


        let a0 = Vector::from(&Point::new(3, -4).unwrap()).pseudoangle().unwrap();
        let a1 = Vector::from(&Point::new(3, -3).unwrap()).pseudoangle().unwrap();
        let a2 = Vector::from(&Point::new(3, 0).unwrap()).pseudoangle().unwrap();
        let a3 = Vector::from(&Point::new(3, 1).unwrap()).pseudoangle().unwrap();
        let mut c = Constraint::LOOSE;
        let coo = Coordinate::new(5);
        assert_eq!(c.min, Pseudoangle::UP);
        assert_eq!(c.max, Pseudoangle::DOWN);
        assert_eq!(c.upper_allowance(
            &e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), std::f64::INFINITY);
        assert_eq!(c.lower_allowance(
            &e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), std::f64::INFINITY);
        assert!(!c.is_too_tight(&e0.straight, &e0.pseudoangle_for_upper_left(), &coo));
        c = c.constrain_minimum(&a0);
        assert_eq!(c.min, a0);
        assert_eq!(c.max, Pseudoangle::DOWN);
        assert_eq!(c.upper_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 2.666666666666666);
        assert_eq!(c.lower_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), std::f64::INFINITY);
        assert!(!c.is_too_tight(&e0.straight, &e0.pseudoangle_for_upper_left(), &coo));
        c = c.constrain_maximum(&a3);
        assert_eq!(c.min, a0);
        assert_eq!(c.max, a3);
        assert_eq!(c.upper_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 2.666666666666666);
        assert_eq!(c.lower_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 4.0);
        assert!(!c.is_too_tight(&e0.straight, &e0.pseudoangle_for_upper_left(), &coo));
        c = c.constrain_minimum(&a1);
        assert_eq!(c.min, a1);
        assert_eq!(c.max, a3);
        assert_eq!(c.upper_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 1.333333333333333);
        assert_eq!(c.lower_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 4.0);
        c = c.constrain_maximum(&a2);
        assert_eq!(c.min, a1);
        assert_eq!(c.max, a2);
        assert_eq!(c.upper_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 1.333333333333333);
        assert_eq!(c.lower_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 2.666666666666667);
        assert!(!c.is_too_tight(&e0.straight, &e0.pseudoangle_for_upper_left(), &coo));
        c = c.constrain_maximum(&a3);
        assert_eq!(c.min, a1);
        assert_eq!(c.max, a2);
        assert_eq!(c.upper_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 1.333333333333333);
        assert_eq!(c.lower_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 2.666666666666667);
        c = c.constrain_minimum(&a1);
        assert_eq!(c.min, a1);
        assert_eq!(c.max, a2);
        assert_eq!(c.upper_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 1.333333333333333);
        assert_eq!(c.lower_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 2.666666666666667);
        let cu = Vector::from(&Point::new(5, -4).unwrap());
        let a_cu = cu.pseudoangle().unwrap();
        let c_ttfu = c.constrain_minimum(&a_cu);
        assert_eq!(c_ttfu.min, a_cu);
        assert_eq!(c_ttfu.max, a2);
        assert_eq!(c_ttfu.upper_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 0.5333333333333328);
        assert_eq!(c_ttfu.lower_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 2.666666666666667);
        assert!(c_ttfu.is_too_tight(&e0.straight, &e0.pseudoangle_for_upper_left(), &coo));
        let cl= Vector::from(&Point::new(4, -2).unwrap());
        let a_cl = cl.pseudoangle().unwrap();
        assert_eq!(a_cl, Pseudoangle::new(0.6666666666666666));
        let c_ttfl = c.constrain_maximum(&a_cl);
        assert_eq!(c_ttfl.min, a1);
        assert_eq!(c_ttfl.max, a_cl);
        assert_eq!(c_ttfl.upper_allowance(
            &e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 1.333333333333333);
        assert_eq!(c_ttfl.lower_allowance(
            e0.upper_left(),
            &e0.pseudoangle_for_upper_left(),
            &coo
        ), 0.6666666666666665);
        assert!(c_ttfl.is_too_tight(&e0.straight, &e0.pseudoangle_for_upper_left(), &coo));


    }
    #[test]
    fn random_polygon_fiasco_test() {
        let p0 = Point::new(11, 31).unwrap();
        let p1 = Point::new(19, 1).unwrap();

        let edge = Edge::original(0, Operand::Subject, &p0, &p1).unwrap();
        let vec = Vector::from(&Point::new(4, -14).unwrap());
        let c = Constraint::LOOSE;
        let c = c.constrain_maximum(vec.pseudoangle().as_ref().unwrap());
        let a = c.lower_allowance(edge.upper_left(), &edge.pseudoangle_for_upper_left(), &Coordinate::new(15));
        assert!(a < 1.001, "Actually: {}", a);
        assert!(c.is_too_tight(&edge.straight, &edge.pseudoangle_for_upper_left(), &Coordinate::new(15)));
    }
}
