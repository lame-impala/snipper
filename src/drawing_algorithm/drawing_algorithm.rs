use crate::drawing_algorithm::builder_list::{BuilderList, Chunk};
use crate::drawing_algorithm::builder::{PathsInConstruction, BuilderIndex, ChainBuilder, Location};
use crate::{Coordinate, Point, AbstractPoint};
use crate::edge::Edge;
use crate::units::Pseudoangle;
use std::collections::btree_map::BTreeMap;

pub struct DrawingAlgorithm {
    x: Coordinate,
    current_builder: BuilderIndex,
    state: PathsInConstruction,
    candidates: BuilderList,
    promises: BuilderList,
    created: BTreeMap<Point, usize>
}
impl DrawingAlgorithm {
    pub fn new(x: Coordinate, state: PathsInConstruction, candidates: BuilderList) -> DrawingAlgorithm {
        DrawingAlgorithm {
            x,
            current_builder: BuilderIndex::BeforeFirst,
            state,
            candidates,
            promises: BuilderList::new(),
            created: BTreeMap::new()
        }
    }
    pub fn initial_state(x: Coordinate) -> DrawingAlgorithm {
        let state = PathsInConstruction::new();
        let candidates = BuilderList::new();
        DrawingAlgorithm::new(x, state, candidates)
    }
    pub fn next_state(self, new_x: Coordinate) -> DrawingAlgorithm {
        debug_assert!(self.created.is_empty());
        let old_x = self.x();
        let (terminated, passed) = self.candidates.split_off(old_x);
        let candidates = passed.extend(self.promises);
        let state = DrawingAlgorithm::terminate(terminated, self.state);
        DrawingAlgorithm::new(new_x, state, candidates)
    }
    pub fn terminate_all(self) -> PathsInConstruction {
        debug_assert!(self.created.is_empty());
        DrawingAlgorithm::terminate(self.candidates, self.state)
    }
    fn terminate(terminated: BuilderList, mut state: PathsInConstruction) -> PathsInConstruction {
        for chunk in terminated.chunks() {
            match chunk {
                Chunk::Complete(a, b) => {
                    state.connect(a, b, Location::End, Location::End);
                },
                Chunk::Partial(_, point) => {
                    panic!("Partial chunk at {}", point.inspect());
                }
            }
        }
        state
    }
    pub fn x(&self) -> Coordinate {
        self.x
    }
    pub fn draw_edge(&mut self, edge: &Edge) {
        let candidate= if let Some(candidate) = self.candidate(edge) {
            self.current_builder = BuilderIndex::Some(candidate);
            candidate
        } else {
            let point = edge.upper_left();
            if edge.pseudoangle == Pseudoangle::DOWN {
                // For every vertical without appropriate builder
                // there is one more edge to the right of the scope
                let index = self.create_builder_after_current(point);
                self.candidates.push_back(point.clone(), index);
            }
            self.create_builder_after_current(point)
        };
        self.append_to(candidate, edge.lower_right().clone());
    }
    pub fn candidate(&mut self, edge: &Edge) -> Option<usize> {
        let point = edge.upper_left();
        let result = if edge.pseudoangle == Pseudoangle::DOWN {
            // There might be edges to the right of the scope
            // we have no information about at this point
            self.candidates.pop_back(point)
        } else {
            self.candidates.pop_front(point)
        };
        result
    }
    pub fn append_to(&mut self, index: usize, point: Point) {
        let x = self.x();
        debug_assert!(point.x() >= x);
        let builder = self.state
            .get_builder_mut(index)
            .expect("Builder expected to be there");
        if point.x() == x {
            builder.append(point.clone());
            self.candidates.push_back(point, index);
        } else {
            self.promises.push_back(builder.loose_end().clone(), index);
        }
    }
    pub fn create_builder_after_current(&mut self, point: &Point) -> usize {
        let builder = ChainBuilder::new(point);
        let new_index = self.state.insert_after(builder, self.current_builder);
        self.current_builder = new_index;
        if let Some(last_created) = self.created.remove(point) {
            self.state.connect(
                last_created,
                new_index.unwrap(),
                Location::Start,
                Location::Start
            );
        } else {
            self.created.insert(point.clone(), new_index.unwrap());
        }
        new_index.unwrap()
    }
    #[allow(dead_code)]
    pub fn inspect(&mut self) -> String {
        let builders = self.state.inspect();
        let candidates = self.candidates.inspect();
        let promises = self.promises.inspect();
        let created = format!("{:?}", self.created);
        format!("Drawing scope at {}, last created: {}\n{}\nCandidates -- {}\nPromises -- {}", self.x(), created, builders, candidates, promises)
    }
}
#[cfg(test)]
mod test {
    use crate::drawing_algorithm::drawing_algorithm::DrawingAlgorithm;
    use crate::{Coordinate, Point};
    use crate::edge::Edge;
    use crate::operation::Operand;
    use crate::drawing_algorithm::builder::BuilderIndex;

    #[test]
    fn create_builder_test() {
        let p = Point::new(-10, 0).unwrap();
        let mut state = DrawingAlgorithm::initial_state(Coordinate::new(0));
        let index = state.create_builder_after_current(&p);
        assert_eq!(index, 0);
        assert_eq!(state.created.get(&p).unwrap(), &0);
        let index = state.create_builder_after_current(&p);
        assert_eq!(index, 1);
        assert_eq!(state.created.get(&p), None);
    }
    #[test]
    fn append_to_test() {
        let p = Point::new(-10, 0).unwrap();
        let p1 = Point::new(0, 0).unwrap();
        let p2 = Point::new(10, 10).unwrap();
        let mut state = DrawingAlgorithm::initial_state(Coordinate::new(0));
        let index = state.create_builder_after_current(&p);
        state.append_to(index, p1.clone());
        let index = state.create_builder_after_current(&p);
        state.append_to(index, p2.clone());
        assert_eq!(state.candidates.pop_front(&p1), Some(0));
        assert_eq!(state.promises.pop_front(&p), Some(1));
    }
    #[test]
    fn draw_edge_test() {
        let p = Point::new(-10, 0).unwrap();
        let p1 = Point::new(0, 0).unwrap();
        let p2 = Point::new(10, 10).unwrap();
        let e1 = Edge::original(0, Operand::Subject, &p, &p1).unwrap();
        let e2 = Edge::original(1, Operand::Subject, &p, &p2).unwrap();
        let mut state = DrawingAlgorithm::initial_state(Coordinate::new(0));
        state.draw_edge(&e1);
        state.draw_edge(&e2);
        assert_eq!(state.candidates.points().get(&p1).unwrap().iter().peekable().peek(), Some(&&0));
        assert_eq!(state.promises.points().get(&p).unwrap().iter().peekable().peek(), Some(&&1));
        let mut state = state.next_state(Coordinate::new(10));
        assert_eq!(state.candidates.points().get(&p1).unwrap().iter().peekable().peek(), Some(&&0));
        assert_eq!(state.candidates.points().get(&p).unwrap().iter().peekable().peek(), Some(&&1));
        let e3 = Edge::original(0, Operand::Subject, &p1, &p2).unwrap();
        let e4 = Edge::original(1, Operand::Subject, &p, &p2).unwrap();
        state.draw_edge(&e3);
        state.draw_edge(&e4);
        assert_eq!(state.candidates.points().get(&p2).unwrap().iter().peekable().peek(), Some(&&0));
        assert_eq!(state.candidates.points().get(&p2).unwrap().iter().peekable().nth(1), Some(&1));
        let pic = state.terminate_all();
        let (paths, routes) = pic.build_paths();
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].points().len(), 3);
        assert!(routes.belongs_to_path(&0, &0));
        assert!(routes.belongs_to_path(&1, &0));
        assert_eq!(routes.structure()[0], BuilderIndex::BeforeFirst);
        assert_eq!(routes.structure()[1], BuilderIndex::Some(0));
    }
}