use super::primitives::Position;
use std::fmt::Display;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Operand {
    Subject, Clipping
}
impl Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::Subject => write!(f, "S"),
            Operand::Clipping => write!(f, "C")
        }
    }
}
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Wrap {
    Inner,
    Outer
}
impl Wrap {
    pub fn default(&self) -> Position {
        match self {
            &Wrap::Inner => Position::In,
            &Wrap::Outer => Position::Out
        }
    }
}
pub type EdgeTest = fn(
    operand: Operand,
    position: Position
) -> bool;
pub struct Operation {
    pub test: EdgeTest,
    pub check_positions: bool,
    id: usize
}
impl Operation {
    pub const UNION: Operation = Operation {
        test: Operation::union_test,
        check_positions: true, id: 0
    };
    pub const INTERSECTION: Operation = Operation {
        test: Operation::intersection_test,
        check_positions: true, id: 1
    };
    pub const DIFFERENCE: Operation = Operation {
        test: Operation::difference_test,
        check_positions: true, id: 2
    };
    pub const XOR: Operation = Operation {
        test: Operation::noop_test,
        check_positions: false, id: 3
    };
    pub fn id(&self) -> usize {
        self.id
    }
    fn noop_test(
        _: Operand,
        _: Position
    ) -> bool {
        true
    }
    fn union_test(
        _: Operand,
        position: Position
    ) -> bool {
        position == Position::Out
    }
    fn intersection_test(
        _: Operand,
        position: Position
    ) -> bool {
        position == Position::In
    }
    fn difference_test(
        operand: Operand,
        position: Position
    ) -> bool {
        match operand {
            Operand::Subject => position == Position::Out,
            Operand::Clipping => position == Position::In
        }
    }
}
impl PartialEq for Operation {
    fn eq(&self, other: &Operation) -> bool {
        self.id == other.id
    }
}
impl Eq for Operation {}
