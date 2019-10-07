use crate::primitives::Position;
use crate::operation::Wrap;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Partition {
    pub counterclockwise: Position,
    pub clockwise: Position
}
impl Partition {
    pub fn overlap(a: &Partition, b: &Partition) -> bool {
        a.clockwise == b.clockwise || a.counterclockwise == b.counterclockwise
    }
    pub fn is_even(&self) -> bool {
        self.counterclockwise == self.clockwise
    }
    #[cfg(test)]
    pub fn previous_for_ray(&self, even: bool) -> &'static Partition {
        match even {
            true => self.previous_for_even(),
            false => self.previous_for_odd()
        }
    }
    pub fn adjacent_for_even(position: Position) -> &'static Partition {
        match position {
            Position::Out  => &Partition::OUT_OUT,
            Position::In => &Partition::IN_IN,
            _ => panic!("Unexpected position: {:?}", position)
        }
    }
    pub fn previous_for_even(&self) -> &'static Partition {
        let previous = self.counterclockwise;
        Partition::adjacent_for_even(previous)
    }
    pub fn previous_for_odd(&self) -> &'static Partition {
        let previous = self.counterclockwise;
        match previous {
            Position::Out  => &Partition::IN_OUT,
            Position::In => &Partition::OUT_IN,
            _ => panic!("Unexpected position: {:?}", self.counterclockwise)
        }
    }
    pub fn flip(&self) -> &'static Partition {
        match self {
            &Partition::IN_IN => &Partition::IN_IN,
            &Partition::IN_OUT => &Partition::OUT_IN,
            &Partition::OUT_IN => &Partition::IN_OUT,
            &Partition::OUT_OUT => &Partition::OUT_OUT,
            _ => panic!("Unknown partition type: {:?}", self)
        }
    }
    pub fn position_from_partition(
        proper: &Partition,
        other: &Partition,
        wrap: Wrap
    ) -> Position {
        match wrap {
            Wrap::Outer => {
                if other == &Partition::IN_IN {
                    Position::In
                } else {
                    Position::Out
                }
            },
            Wrap::Inner => {
                if other.is_even() {
                    other.clockwise
                } else if proper.is_even() {
                    wrap.default()
                } else {
                    if Partition::overlap(proper, other) {
                        wrap.default()
                    } else {
                        Position::Out
                    }
                }
            },
        }
    }
    #[allow(dead_code)]
    pub fn inspect(&self) -> String {
        match self {
            &Partition::IN_IN => "IN-IN".to_owned(),
            &Partition::IN_OUT => "IN-OUT".to_owned(),
            &Partition::OUT_IN => "OUT-IN".to_owned(),
            &Partition::OUT_OUT => "OUT-OUT".to_owned(),
            _ => panic!("Unknown partition type: {:?}", self)
        }
    }
    pub const IN_IN: Partition = Partition{counterclockwise: Position::In, clockwise: Position::In};
    pub const IN_OUT: Partition = Partition{counterclockwise: Position::In, clockwise: Position::Out};
    pub const OUT_IN: Partition = Partition{counterclockwise: Position::Out, clockwise: Position::In};
    pub const OUT_OUT: Partition = Partition{counterclockwise: Position::Out, clockwise: Position::Out};
}

#[test]
fn partition_test() {
    let ii = &Partition::IN_IN;
    assert_eq!(ii.previous_for_ray(true), &Partition::IN_IN);
    assert_eq!(ii.previous_for_ray(false), &Partition::OUT_IN);
    assert_eq!(ii.flip(), &Partition::IN_IN);
    let oo = &Partition::OUT_OUT;
    assert_eq!(oo.previous_for_ray(true), &Partition::OUT_OUT);
    assert_eq!(oo.previous_for_ray(false), &Partition::IN_OUT);
    assert_eq!(oo.flip(), &Partition::OUT_OUT);
    let io = &Partition::IN_OUT;
    assert_eq!(io.previous_for_ray(true), &Partition::IN_IN);
    assert_eq!(io.previous_for_ray(false), &Partition::OUT_IN);
    assert_eq!(io.flip(), &Partition::OUT_IN);
    let oi = &Partition::OUT_IN;
    assert_eq!(oi.previous_for_ray(true), &Partition::OUT_OUT);
    assert_eq!(oi.previous_for_ray(false), &Partition::IN_OUT);
    assert_eq!(oi.flip(), &Partition::IN_OUT);
}