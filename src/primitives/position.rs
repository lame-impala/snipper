#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Position {
    Unknown,
    Out,
    Edge,
    In
}
impl Position {
    pub fn invert(value: &Position) -> Position {
        match value {
            &Position::Unknown => Position::Unknown,
            &Position::Out => Position::In,
            &Position::In => Position::Out,
            &Position::Edge => Position::Edge
        }
    }
}
