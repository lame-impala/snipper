use crate::units::Float;
use std::collections::BTreeSet;
use crate::Coordinate;
use itertools::Either;

pub struct InsertionResult {
    affected_position: Either<(Coordinate, bool), Float>,
    top_dirty: bool,
    bottom_dirty: bool,
    removals: Option<Removals>,
}
impl InsertionResult {
    pub fn support_inserted(
        coordinate: Coordinate,
        top_dirty: bool,
        bottom_dirty: bool,
        vertical_dirty: bool,
        removals: Vec<Float>
    ) -> InsertionResult {
        let position = Either::Left((coordinate, vertical_dirty));
        let removals = Removals::from_vec(removals);
        InsertionResult::new(
            position,
            top_dirty,
            bottom_dirty,
            removals
        )
    }
    pub fn traverse_inserted(
        float: Float,
        top_dirty: bool,
        bottom_dirty: bool,
        removals: Option<Removals>
    ) -> InsertionResult {
        let position = Either::Right(float);
        InsertionResult::new(
            position,
            top_dirty,
            bottom_dirty,
            removals
        )
    }
    pub fn new(
        position: Either<(Coordinate, bool), Float>,
        top_dirty: bool,
        bottom_dirty: bool,
        removals: Option<Removals>,
    ) -> InsertionResult {
        InsertionResult {
            affected_position: position,
            top_dirty,
            bottom_dirty,
            removals,
        }
    }
    #[cfg(test)]
    pub fn report_removed(&mut self, position: Float) {
        let range = self.removals.get_or_insert(Removals::new());
        range.add(position);
    }
    pub fn coordinate_y(&self) -> Option<Coordinate> {
        match self.affected_position {
            Either::Left((coo, _)) => Some(coo),
            Either::Right(_) => None
        }
    }
    pub fn float_y(&self) -> Float {
        match self.affected_position {
            Either::Left((coo, _)) => Float::from(coo),
            Either::Right(float) => float
        }
    }
    pub fn top_dirty(&self) -> bool {
        self.top_dirty
    }
    pub fn bottom_dirty(&self) -> bool {
        self.bottom_dirty
    }
    pub fn vertical_dirty(&self) -> bool {
        match self.affected_position {
            Either::Left((_, vertical_dirty)) => vertical_dirty,
            Either::Right(_) => false
        }
    }
    pub fn removals(&self) -> Option<&Removals> {
        self.removals.as_ref()
    }
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Dirty {
    Top(Float),
    Bottom(Float),
    Vertical(Coordinate),
    None
}

#[derive(Clone)]
pub struct DirtyRecords {
    top_dirty: BTreeSet<Float>,
    bottom_dirty: BTreeSet<Float>,
    vertical_dirty: BTreeSet<Coordinate>
}
impl DirtyRecords {
    pub fn new() -> DirtyRecords {
        DirtyRecords{
            top_dirty: BTreeSet::new(),
            bottom_dirty: BTreeSet::new(),
            vertical_dirty: BTreeSet::new()
        }
    }
    pub fn set_dirty(&mut self, dirty: Dirty) {
        match dirty {
            Dirty::Top(position) => self.top_dirty.insert(position),
            Dirty::Bottom(position) => self.bottom_dirty.insert(position),
            Dirty::Vertical(position) => self.vertical_dirty.insert(position),
            Dirty::None => panic!("None not expected")
        };
    }
    pub fn undirty(&mut self, dirty: Dirty) {
        match dirty {
            Dirty::Top(ref position) => self.top_dirty.remove(position),
            Dirty::Bottom(ref position) => self.bottom_dirty.remove(position),
            Dirty::Vertical(ref position) => self.vertical_dirty.remove(position),
            Dirty::None => panic!("None not expected")
        };
    }
    pub fn remove(&mut self, position: &Float) {
        self.top_dirty.remove(position);
        self.bottom_dirty.remove(position);
        let float = f64::from(*position);
        if position.floor() == float {
            let coo = Coordinate::from_float(float).unwrap();
            self.vertical_dirty.remove(&coo);
        }
    }
    pub fn next(&self) -> Dirty {
        if !self.vertical_dirty.is_empty() {
            let coo = self.vertical_dirty.iter().next().unwrap();
            Dirty::Vertical(*coo)
        } else if !self.top_dirty.is_empty() {
            let float = self.top_dirty.iter().cloned().next().unwrap();
            Dirty::Top(float)
        } else if !self.bottom_dirty.is_empty() {
            let float = self.bottom_dirty.iter().cloned().next().unwrap();
            Dirty::Bottom(float)
        } else {
            Dirty::None
        }
    }
    pub fn has_next(&self) -> bool {
        !self.top_dirty.is_empty() ||
        !self.bottom_dirty.is_empty() ||
        !self.vertical_dirty.is_empty()
    }
    #[allow(dead_code)]
    pub fn inspect(&self) -> String {
        let td: Vec<String> = self.top_dirty.iter().map(|y| format!("{}", y)).collect();
        let bd: Vec<String> = self.bottom_dirty.iter().map(|y| format!("{}", y)).collect();
        let vd: Vec<String> = self.vertical_dirty.iter().map(|y| format!("{}", y)).collect();
        format!("TD: {} -- BD: {} -- VD: {}", td.join(", "), bd. join(", "), vd.join(", "))
    }
    #[cfg(test)]
    pub fn explode(self) -> (BTreeSet<Float>, BTreeSet<Float>, BTreeSet<Coordinate>) {
        (self.top_dirty, self.bottom_dirty, self.vertical_dirty)
    }
}

#[derive(Debug)]
pub struct Removals {
    min: Option<Float>,
    max: Option<Float>,
    vec: Vec<Float>
}
impl Removals {
    pub fn from_vec(vec: Vec<Float>) -> Option<Removals> {
        if vec.is_empty() { return None; }
        let mut removals = Removals::new();
        for float in vec {
            removals.add(float)
        }
        Some(removals)
    }
    pub fn new () -> Removals {
        Removals { min: None, max: None, vec: Vec::new() }
    }
    pub fn add(&mut self, float: Float) {
        if let Some(min) = self.min {
            self.min = Some(min.min(float));
        } else {
            self.min = Some(float);
        }
        if let Some(max) = self.max {
            self.max = Some(max.max(float));
        } else {
            self.max = Some(float);
        }
        self.vec.push(float);
    }
    pub fn min(&self) -> Option<&Float> {
        self.min.as_ref()
    }
    pub fn max(&self) -> Option<&Float> {
        self.max.as_ref()
    }
    pub fn is_empty(&self) -> bool { self.vec.is_empty() }
    pub fn iter(&self) -> std::slice::Iter<Float> {
        self.vec.iter()
    }
}

#[test]
fn insertion_result_test() {
    let ir = InsertionResult::new(
        Either::Left((Coordinate::new(5), true)),
        true,
        false,
        None,
    );
    assert_eq!(ir.float_y(), Float::from(5));
    assert!(ir.top_dirty());
    assert!(!ir.bottom_dirty());
    assert!(ir.vertical_dirty());
    let mut ir = InsertionResult::new(
        Either::Right(Float::from(10)),
        false,
        true,
        None,
    );
    assert_eq!(ir.float_y(), Float::from(10));
    assert!(!ir.top_dirty());
    assert!(ir.bottom_dirty());
    assert!(!ir.vertical_dirty());
    assert!(ir.removals().is_none());
    ir.report_removed(Float::from(30));
    ir.report_removed(Float::from(40));
    ir.report_removed(Float::from(35));
    let r = ir.removals.expect("Should be there");
    assert_eq!(r.vec.len(), 3);
    assert_eq!(r.min().unwrap(), &Float::from(30));
    assert_eq!(r.max().unwrap(), &Float::from(40));

}
#[test]
fn dirty_records_test() {
    let mut dr = DirtyRecords::new();
    assert!(!dr.has_next());
    dr.set_dirty(Dirty::Bottom(Float::from(5)));
    assert!(dr.has_next());
    assert_eq!(dr.next(), Dirty::Bottom(Float::from(5)));
    dr.set_dirty(Dirty::Top(Float::from(6)));
    assert_eq!(dr.next(), Dirty::Top(Float::from(6)));
    dr.set_dirty(Dirty::Vertical(Coordinate::new(7)));
    assert_eq!(dr.next(), Dirty::Vertical(Coordinate::new(7)));
    dr.undirty(Dirty::Vertical(Coordinate::new(7)));
    assert_eq!(dr.next(), Dirty::Top(Float::from(6)));
    dr.undirty(Dirty::Top(Float::from(6)));
    assert_eq!(dr.next(), Dirty::Bottom(Float::from(5)));
    dr.undirty(Dirty::Bottom(Float::from(5)));
    assert_eq!(dr.next(), Dirty::None);
    assert!(!dr.has_next());

    dr.set_dirty(Dirty::Bottom(Float::from(5)));
    dr.set_dirty(Dirty::Top(Float::from(5)));
    dr.set_dirty(Dirty::Vertical(Coordinate::new(5)));
    assert!(dr.has_next());
    dr.remove(&Float::from(5));
    assert!(!dr.has_next());

}

#[test]
fn range_test() {
    let mut r = Removals::new();
    assert!(r.is_empty());
    assert!(r.min.is_none());
    assert!(r.max.is_none());
    r.add(Float::new(5.0).unwrap());
    assert!(!r.is_empty());
    assert_eq!(r.min.unwrap(), Float::new(5.0).unwrap());
    assert_eq!(r.max.unwrap(), Float::new(5.0).unwrap());
    r.add(Float::new(3.0).unwrap());
    assert_eq!(r.min.unwrap(), Float::new(3.0).unwrap());
    assert_eq!(r.max.unwrap(), Float::new(5.0).unwrap());
    r.add(Float::new(10.0).unwrap());
    assert_eq!(r.min.unwrap(), Float::new(3.0).unwrap());
    assert_eq!(r.max.unwrap(), Float::new(10.0).unwrap());
}
