use crate::primitives::{Bounds, AbstractPoint, Point, Position};
use crate::shape::{Shape};
use crate::shape::{Path, PathDirection};
use super::triangular_matrix::{TriangularMatrix};
use crate::Error;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Relation {
    Unknown,
    Unrelated,
    PartlyContained,
    Contains,
    Contained,
    Identical
}
impl Relation {
    pub fn invert(value: Relation) -> Relation {
        match value {
            Relation::Unknown => Relation::Unknown,
            Relation::Unrelated => Relation::Unrelated,
            Relation::PartlyContained => Relation::PartlyContained,
            Relation::Contained => Relation::Contains,
            Relation::Contains => Relation::Contained,
            Relation::Identical => Relation::Identical
        }
    }
}

#[derive(Clone, Debug)]
struct PolygonCount {
    bounds: Option<Bounds>,
    running_area: f64,
    running_centroid: Option<(f64, f64)>
}

impl PolygonCount {
    pub fn new() -> PolygonCount {
        PolygonCount{bounds: None, running_area: 0f64, running_centroid: None}
    }
    #[allow(dead_code)]
    fn inspect(&self) -> String {
        match self.centroid() {
            None => String::from("Polygon: 0/None"),
            Some(point) => {
                format!(
                    "Polygon: {:.2}/[{}, {}]",
                    self.area(),
                    point.x(),
                    point.y()
                )

            }
        }
    }
    pub fn area(&self) -> f64 {
        self.running_area
    }
    pub fn centroid(&self) -> Option<Point> {
        match self.running_centroid {
            None => None,
            Some(tuple) => {
                let x = (tuple.0 / self.running_area).round() as i32;
                let y = (tuple.1 / self.running_area).round() as i32;
                let result = Point::new(x, y);
                match result {
                    Ok(point) => Some(point),
                    _ => None
                }
            }
        }
    }
    fn update (&self, path: &Path, level: usize) -> PolygonCount {
        if path.is_null() { return self.clone(); }
        match &self.bounds {
            None => {
                if level > 0 { panic!("Zero level expected, got {}", level)}
                let bounds = Some(path.bounds().expect("Expected bounds to be there").clone());
                let running_area = path.area();
                let centroid = path.centroid().expect("Expected centroid to be there");
                let rcx = centroid.float_x() * running_area;
                let rcy = centroid.float_y() * running_area;
                let running_centroid = Some((rcx, rcy));
                PolygonCount{bounds, running_area, running_centroid}
            },
            Some(ref old) => {
                let bounds = if level == 0 {
                    let new = Bounds::union(old, &path.bounds().unwrap());
                    Some(new)
                } else {
                    Some(old.clone())
                };
                let rc = self.running_centroid.unwrap();
                let rcx_incr = path.centroid().unwrap().float_x() * path.area();
                let rcy_incr = path.centroid().unwrap().float_y() * path.area();
                let (running_area, running_centroid) = if level % 2 == 0 {
                    let ra = self.running_area + path.area();
                    let rcx = rc.0 + rcx_incr;
                    let rcy = rc.1 + rcy_incr;
                    (ra, Some((rcx, rcy)))
                } else {
                    let ra = self.running_area - path.area();
                    let rcx = rc.0 - rcx_incr;
                    let rcy = rc.1 - rcy_incr;
                    (ra, Some((rcx, rcy)))
                };
                PolygonCount{bounds, running_area, running_centroid}
            }
        }
    }
}
#[derive(Clone)]
pub struct Record {
    level: usize,
    parent: Option<usize>
}
impl Record {
    pub fn level(&self) -> usize {
        return self.level;
    }
    fn new() -> Record {
        Record{level: 0, parent: None}
    }
    pub fn parent(&self) -> Option<usize> {
        self.parent
    }
}
pub trait Comparator {
    fn compare(&mut self, a: &Path, b: &Path, a_id: usize, b_id: usize) -> Relation;
}
#[derive(Clone)]
struct PolygonStructure {
    table: TriangularMatrix<Relation>,
    records: Vec<Record>,
    levels: Vec< Vec<usize> >
}
impl PolygonStructure {
    pub fn trivial() -> PolygonStructure {
        let table = TriangularMatrix::new(
            1,
            Relation::Unrelated,
            Relation::invert
        ).unwrap();
        let record = Record::new();
        let records = vec![record];
        let zero = vec![0];
        let levels = vec![zero];
        PolygonStructure{table, records, levels}
    }
    pub fn build(paths: &Vec<Path>, comparator: &mut dyn Comparator) -> Result<PolygonStructure, &'static str> {
        let mut table = TriangularMatrix::new(
            paths.len(),
            Relation::Unknown,
            Relation::invert
        )?;
        let mut records = (0..paths.len()).map(|_: usize| Record::new()).collect();
        let mut levels: Vec< Vec<usize>> = Vec::new();
        for index in 0..paths.len() {
            PolygonStructure::build_record(
                index,
                paths,
                &mut table,
                &mut records,
                &mut levels,
                comparator
            );
        }
        PolygonStructure::build_levels(&mut table, &mut records, &mut levels);
        Ok(PolygonStructure{table, records, levels})
    }
    fn build_record(
        index: usize,
        paths: &Vec<Path>,
        table: &mut TriangularMatrix<Relation>,
        records: &mut Vec<Record>,
        levels: &mut Vec< Vec<usize> >,
        comparator: &mut dyn Comparator
    ) {
        let path = &paths[index];
        let mut unrelated: Vec<usize> = Vec::new();
        let mut contained: Vec<usize> = Vec::new();
        for other_index in (index + 1)..paths.len() {
            let mut relation = table.get(index, other_index).unwrap();
            if relation == Relation::Unknown {
                let other_path = &paths[other_index];
                relation = comparator.compare(path, other_path, index, other_index);
                table.set(index, other_index, relation);
            }
            match relation {
                Relation::Contains => {
                    records[other_index].level += 1;
                    contained.push(other_index);
                    PolygonStructure::set_relations(
                        other_index,
                        &unrelated,
                        Relation::Unrelated, table
                    );
                },
                Relation::Unrelated => {
                    unrelated.push(other_index);
                    PolygonStructure::set_relations(
                        other_index,
                        &contained,
                        Relation::Unrelated, table
                    );
                },
                Relation::Contained => {
                    records[index].level += 1;
                    PolygonStructure::set_relations(
                        other_index,
                        &contained,
                        Relation::Contains,
                        table
                    );
                },
                _ => panic!("Unexpected relation: {:?}", relation)
            }
        }
        let level = records[index].level;
        while levels.len() <= level {
            levels.push(Vec::new());
        }
        levels[level].push(index);
    }
    fn set_relations(
        index: usize,
        indices: &Vec<usize>,
        relation: Relation,
        table: &mut TriangularMatrix<Relation>
    ) {
        for other_index in indices {
            table.set(index, *other_index, relation);
        }
    }
    fn build_levels(
        table: &TriangularMatrix<Relation>,
        records: &mut Vec<Record>,
        levels: &Vec< Vec<usize>>
    ) {
        if levels.len() == 0 { return; }
        for level in 0..levels.len() - 1 {
            PolygonStructure::build_level(level, table, levels, records);
        }
    }
    fn build_level(
        index: usize,
        table: &TriangularMatrix<Relation>,
        levels: &Vec< Vec<usize> >,
        records: &mut Vec<Record>
    ) {
        let level = &levels[index];
        let child_indices = &levels[index + 1];
        for parent_index in level {
            PolygonStructure::register_parent(*parent_index, table, child_indices, records);
        }
    }
    fn register_parent(
        parent_index: usize,
        table: &TriangularMatrix<Relation>,
        child_indices: &Vec<usize>,
        records: &mut Vec<Record>
    ) {
        for child_index in child_indices {
            let relation = table.get(parent_index, *child_index).unwrap();
            if relation == Relation::Contains {
                records[*child_index].parent = Some(parent_index);
            }
        }
    }
}

pub struct FlatComparator {}
impl Comparator for FlatComparator {
    fn compare (&mut self, _: &Path, _: &Path, _: usize, _: usize) -> Relation {
        Relation::Unrelated
    }

}
#[derive(Clone)]
pub struct Polygon {
    paths: Vec<Path>,
    structure: Vec<Record>,
    levels: Vec<Vec<usize>>,
    count: PolygonCount
}
impl Polygon {
    pub unsafe fn flat(paths: Vec<Path>) -> Result<Polygon, Error> {
        let mut comparator = FlatComparator{};
        Polygon::build(paths, &mut comparator)
    }
    pub unsafe fn trivial(path: Path) -> Polygon {
        let count = PolygonCount::new().update(&path, 0);
        let normalized = Polygon::normalize(path, 0);
        let paths = vec![normalized];
        let structure = PolygonStructure::trivial();
        Polygon{
            paths,
            structure: structure.records,
            levels: structure.levels,
            count
        }
    }
    pub fn build(mut paths: Vec<Path>, comparator: &mut dyn Comparator) -> Result<Polygon, Error> {
        let result = PolygonStructure::build(&paths, comparator);
        if let Ok(structure) = result {
            let mut count = PolygonCount::new();
            let normalized: Vec<Path> = paths.drain(..).enumerate().map(|(path_index, path)| {
                let depth = structure.records[path_index].level;
                count = count.update(&path, depth);
                Polygon::normalize(path, depth)
            }).collect();
            Ok(Polygon{
                paths: normalized,
                structure: structure.records,
                levels: structure.levels,
                count
            })
        } else {
            Err(Error::TooManyPathsError)
        }
    }
    pub fn centroid(&self) -> Option<Point> {
        self.count.centroid()
    }
    pub fn area(&self) -> f64 {
        self.count.area()
    }

    fn position_at_level(
        &self, point: &Point,
        level: &Vec<usize>,
        parent_index: Option<usize>
    ) -> (Position, Option<usize>) {
        let mut position = Position::Out;
        let mut index = 0;
        let mut path_index = None;
        while index < level.len() && position == Position::Out {
            path_index = Some(level[index]);
            position = if
                parent_index.is_none() ||
                self.structure[path_index.unwrap()].parent == parent_index {
                let path: &Path = &self.paths[path_index.unwrap()];
                path.position(point)
            } else {
                position
            };
            if position == Position::Out {
                index += 1;
                path_index = None
            }
        }
        (position, path_index)
    }
    fn normalize(path: Path, depth: usize) -> Path {
        fn should_reverse(path: &Path, depth: usize) -> bool {
            if depth % 2 == 0 {
                path.direction() == PathDirection::Counterclockwise
            } else {
                path.direction() == PathDirection::Clockwise
            }
        }
        if should_reverse(&path, depth) {
            path.reverse()
        } else {
            path
        }
    }
    pub fn paths(&self) -> &Vec<Path> {
        &self.paths
    }
    pub fn inspect(&self) -> String {
        let strings: Vec<String> = self.paths.iter().map(|path| path.inspect()).collect();
        format!("{} paths: {}", strings.len(), strings.join("; "))
    }
    pub fn structure(&self) -> &Vec<Record> {
        &self.structure
    }
}
impl Shape for Polygon {
    fn position(&self, point: &Point) -> Position {
        let mut position = Position::Out;
        let levels = &self.levels;
        let mut depth = 0;
        let mut path_index: Option<usize> = None;
        while depth < levels.len() && position != Position::Edge {
            let level = &levels[depth];
            let tuple = self.position_at_level(point, level, path_index);

            if tuple.0 == Position::Edge {
                position = tuple.0;
            } else if tuple.0 == Position::Out {
                // break
                depth = levels.len()
            } else {
                path_index = tuple.1;
                position = Position::invert(&position);
                depth += 1;
            }
        }
        position
    }
    fn bounds(&self) -> Option<&Bounds> {
        self.count.bounds.as_ref()
    }
    fn paths(&self) -> Vec<&Path> {
        self.paths.iter().map(|path_ref: &Path| path_ref).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::primitives::{Sector, Mode};
    use super::Path;
//    use crate::position_count_algorithm::position_count::PositionCount;
    use super::Shape;
    use super::Position;
    use super::Relation;
    use super::Comparator;
    use super::Point;
    use super::Polygon;
    use super::PathDirection;
    use super::Bounds;
    use std::collections::HashMap;
    use crate::shape::polygon::PolygonStructure;

    pub struct PositionCount {
        inner: usize,
        outer: usize,
        edge: usize,
        edges: Option<HashMap<usize, Position>>
    }
    impl PositionCount {
        #[allow(dead_code)]
        pub fn inspect(&self) -> String {
            format!("IN: {}, OUT: {}, EDGE: {}", self.inner, self.outer, self.edge)
        }
        pub fn new(full: bool) -> PositionCount {
            let edges = if full {
                Some(HashMap::new())
            } else {
                None
            };
            PositionCount{inner: 0, outer: 0, edge: 0, edges}
        }
        pub fn check(&mut self, id: usize, position: Position) {
            match position {
                Position::In => self.inner += 1,
                Position::Out => self.outer += 1,
                Position::Edge => self.edge += 1,
                Position::Unknown => panic!("Expected definite position")
            }
            if self.edges.is_some() {
                self.set_edge_position(id, position);
            }
        }
        fn set_edge_position(&mut self, id: usize, position: Position) {
            let edges = self.edges.as_mut().unwrap();
            if edges.contains_key(&id) {
                panic!("Edge position already checked: {}", id);
            } else {
                edges.insert(id, position);
            }
        }
        pub fn compare(&self, other: &PositionCount) -> Relation {
            if self.outer > 0 {
                if self.inner == 0 {
                    if other.outer == 0 {
                        if other.inner > 0 {
                            Relation::Contains
                        } else {
                            Relation::Unrelated
                        }
                    } else {
                        Relation::Unrelated
                    }
                } else {
                    Relation::PartlyContained
                }
            } else if self.inner > 0 {
                Relation::Contained
            } else if self.edge > 0 {
                if other.edge != self.edge {
                    panic!(
                        "Expected number of edge hits to be identical, got: {}/{}",
                        self.edge,
                        other.edge
                    );
                } else {
                    Relation::Identical
                }
            } else {
                Relation::Unrelated
            }
        }
    }

    struct NaiveComparator {}
    impl NaiveComparator {
        fn count(&self, a: &Path, b: &Path) -> PositionCount {
            let mut count = PositionCount::new(false);
            for index in 0..a.points().len() {
                let segment = a.segment_at(index).unwrap();
                let start_position = b.position(&segment.start);
                let end_position = b.position(&segment.end);
                if start_position == Position::Edge {
                    count.check(index, end_position);
                } else if end_position == Position::Edge {
                    count.check(index, start_position);
                } else if start_position == end_position {
                    count.check(index ,start_position);
                } else {
                    // Assuming there are no intersections
                    panic!("Intersection detected");
                }
            }
            count
        }
    }
    impl Comparator for NaiveComparator {
        fn compare (&mut self, a: &Path, b: &Path, _: usize, _: usize) -> Relation {
            let a_count = self.count(a, b);
            let b_count = self.count(b, a);
            a_count.compare(&b_count)
        }
    }

    #[test]
    fn polygon_test() {
        let r0 = Path::new(&vec![
            Point::new(0, 0).expect("!"),
            Point::new(10, 0).expect("!"),
            Point::new(10, 8).expect("!"),
            Point::new(0, 8).expect("!")
        ]); // area 80, centroid {5, 4}
        let p00 = Path::new(&vec![
            Point::new(0, 0).expect("!"),
            Point::new(6, 0).expect("!"),
            Point::new(6, 5).expect("!"),
            Point::new(0, 5).expect("!")
        ]); // area 30, centroid {2.5, 3}
        let p010 = Path::new(&vec![
            Point::new(0, 0).expect("!"),
            Point::new(2, 0).expect("!"),
            Point::new(2, 2).expect("!"),
            Point::new(0, 2).expect("!")
        ]); // area 4, centroid {1, 1}
        let p001 = Path::new(&vec![
            Point::new(4, 0).expect("!"),
            Point::new(6, 0).expect("!"),
            Point::new(6, 3).expect("!"),
            Point::new(4, 3).expect("!")
        ]); // area 12, centroid {5, 1.5}
        let p01 = Path::new(&vec![
            Point::new(8, 0).expect("!"),
            Point::new(10, 0).expect("!"),
            Point::new(10, 4).expect("!"),
            Point::new(8, 4).expect("!")
        ]); // area 8, centroid {9, 2}

        let r1 = Path::new(&vec![
            Point::new(12, 0).expect("!"),
            Point::new(22, 0).expect("!"),
            Point::new(22, 6).expect("!"),
            Point::new(12, 6).expect("!")
        ]); // area 60, centroid {16, 3}
        let p10 = Path::new(&vec![
            Point::new(12, 0).expect("!"),
            Point::new(14, 0).expect("!"),
            Point::new(14, 5).expect("!"),
            Point::new(12, 5).expect("!")
        ]); // area 10, centroid {13, 2.5}
        let p11 = Path::new(&vec![
            Point::new(16, 0).expect("!"),
            Point::new(22, 0).expect("!"),
            Point::new(22, 5).expect("!"),
            Point::new(16, 5).expect("!")
        ]); // area 30, centroid {19, 2.5}
        let p110 = Path::new(&vec![
            Point::new(18, 0).expect("!"),
            Point::new(18, 0).expect("!"),
            Point::new(18, 4).expect("!"),
            Point::new(16, 4).expect("!")
        ]); // area 8, centroid {17, 2}
        let p111 = Path::new(&vec![
            Point::new(20, 0).expect("!"),
            Point::new(22, 0).expect("!"),
            Point::new(22, 2).expect("!"),
            Point::new(20, 2).expect("!")
        ]); // area 4, centroid {21, 1}

        let paths = vec![r0, p00, p010, p001, p01, r1, p10, p11, p110, p111];

        let mut comp = NaiveComparator{};
        let structure = PolygonStructure::build(&paths, &mut comp).unwrap();


        let t = &structure.table;
        assert_eq!(t.get(0, 1).unwrap(), Relation::Contains);
        assert_eq!(t.get(0, 2).unwrap(), Relation::Contains);
        assert_eq!(t.get(0, 3).unwrap(), Relation::Contains);
        assert_eq!(t.get(0, 4).unwrap(), Relation::Contains);
        assert_eq!(t.get(0, 5).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(0, 6).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(0, 7).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(0, 8).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(0, 9).unwrap(), Relation::Unrelated);

        assert_eq!(t.get(1, 2).unwrap(), Relation::Contains);
        assert_eq!(t.get(1, 3).unwrap(), Relation::Contains);
        assert_eq!(t.get(1, 4).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(1, 5).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(1, 6).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(1, 7).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(1, 8).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(1, 9).unwrap(), Relation::Unrelated);

        assert_eq!(t.get(2, 3).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(2, 4).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(2, 5).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(2, 6).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(2, 7).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(2, 8).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(2, 9).unwrap(), Relation::Unrelated);

        assert_eq!(t.get(3, 4).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(3, 5).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(3, 6).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(3, 7).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(3, 8).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(3, 9).unwrap(), Relation::Unrelated);

        assert_eq!(t.get(4, 5).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(4, 6).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(4, 7).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(4, 8).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(4, 9).unwrap(), Relation::Unrelated);

        assert_eq!(t.get(5, 6).unwrap(), Relation::Contains);
        assert_eq!(t.get(5, 7).unwrap(), Relation::Contains);
        assert_eq!(t.get(5, 8).unwrap(), Relation::Contains);
        assert_eq!(t.get(5, 9).unwrap(), Relation::Contains);

        assert_eq!(t.get(6, 7).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(6, 8).unwrap(), Relation::Unrelated);
        assert_eq!(t.get(6, 9).unwrap(), Relation::Unrelated);

        assert_eq!(t.get(7, 8).unwrap(), Relation::Contains);
        assert_eq!(t.get(7, 9).unwrap(), Relation::Contains);

        assert_eq!(t.get(8, 9).unwrap(), Relation::Unrelated);


        assert_eq!(structure.levels.len(), 3);
        assert_eq!(structure.levels[0], vec![0, 5]);
        assert_eq!(structure.levels[1], vec![1, 4, 6, 7]);
        assert_eq!(structure.levels[2], vec![2, 3, 8, 9]);

        let poly = Polygon::build(paths, &mut comp).unwrap();
        assert!(poly.structure[0].parent.is_none());
        assert_eq!(poly.paths[0].direction(), PathDirection::Clockwise);
        assert_eq!(poly.structure[1].parent, Some(0));
        assert_eq!(poly.paths[1].direction(), PathDirection::Counterclockwise);
        assert_eq!(poly.structure[2].parent, Some(1));
        assert_eq!(poly.paths[2].direction(), PathDirection::Clockwise);
        assert_eq!(poly.structure[3].parent, Some(1));
        assert_eq!(poly.paths[3].direction(), PathDirection::Clockwise);
        assert_eq!(poly.structure[4].parent, Some(0));
        assert_eq!(poly.paths[4].direction(), PathDirection::Counterclockwise);
        assert!(poly.structure[5].parent.is_none());
        assert_eq!(poly.paths[5].direction(), PathDirection::Clockwise);
        assert_eq!(poly.structure[6].parent, Some(5));
        assert_eq!(poly.paths[6].direction(), PathDirection::Counterclockwise);
        assert_eq!(poly.structure[7].parent, Some(5));
        assert_eq!(poly.paths[7].direction(), PathDirection::Counterclockwise);
        assert_eq!(poly.structure[8].parent, Some(7));
        assert_eq!(poly.paths[8].direction(), PathDirection::Clockwise);
        assert_eq!(poly.structure[9].parent, Some(7));
        assert_eq!(poly.paths[9].direction(), PathDirection::Clockwise);

        assert_eq!(poly.count.bounds.as_ref().unwrap(), &Bounds::new(0, 0, 8, 22));
        assert_eq!(poly.count.centroid().unwrap(), Point::new(9, 4).expect("!"));
        assert_eq!(poly.count.area(), 80f64);

        assert!(!poly.contains(&Point::new(-1, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(0, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(1, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(2, 1).expect("!"), &Mode::Closed));
        assert!(!poly.contains(&Point::new(3, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(4, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(5, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(6, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(7, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(8, 1).expect("!"), &Mode::Closed));
        assert!(!poly.contains(&Point::new(9, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(10, 1).expect("!"), &Mode::Closed));
        assert!(!poly.contains(&Point::new(11, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(12, 1).expect("!"), &Mode::Closed));
        assert!(!poly.contains(&Point::new(13, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(14, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(15, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(16, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(17, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(18, 1).expect("!"), &Mode::Closed));
        assert!(!poly.contains(&Point::new(19, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(20, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(21, 1).expect("!"), &Mode::Closed));
        assert!(poly.contains(&Point::new(22, 1).expect("!"), &Mode::Closed));
        assert!(!poly.contains(&Point::new(23, 1).expect("!"), &Mode::Closed));
    }
}