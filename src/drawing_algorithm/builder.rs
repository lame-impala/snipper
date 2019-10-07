use crate::{AbstractPoint, Point, Path, PathBuilder};
use std::cmp::Ordering;
use crate::drawing_algorithm::routes::Routes;
use std::collections::btree_set::BTreeSet;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum BuilderIndex {
    BeforeFirst,
    Some(usize),
    AfterLast
}
impl BuilderIndex {
    pub fn option(&self) -> Option<usize> {
        match self {
            BuilderIndex::BeforeFirst => None,
            BuilderIndex::Some(index) => Some(*index),
            BuilderIndex::AfterLast => None
        }
    }
    pub fn is_some(&self) -> bool {
        match self {
            BuilderIndex::BeforeFirst => false,
            BuilderIndex::Some(_) => true,
            BuilderIndex::AfterLast => false
        }
    }
    pub fn unwrap(&self) -> usize {
        match self {
            BuilderIndex::BeforeFirst => panic!("Index is empty"),
            BuilderIndex::Some(index) => *index,
            BuilderIndex::AfterLast => panic!("Index is empty")
        }
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.x() < other.x() {
            Ordering::Less
        } else if self.x() > other.x() {
            Ordering::Greater
        } else if self.y() < other.y() {
            Ordering::Less
        } else if self.y() > other.y() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Location {
    Start, End
}

pub struct PathsInConstruction {
    head: BuilderIndex,
    nodes: Vec<ChainBuilderNode>,
    structure: Vec<BuilderIndex>
}
impl PathsInConstruction {
    pub fn inspect(&self) -> String {
        let vec = self.to_vec();
        let strings: Vec<String> = vec.iter().map(|b| {
            b.inspect()
        }).collect();
        format!("{} paths:\n{}", strings.len(), strings.join("\n"))
    }
    pub fn new() -> PathsInConstruction {
        PathsInConstruction{
            head: BuilderIndex::BeforeFirst,
            nodes: Vec::new(),
            structure: Vec::new()
        }
    }
    pub fn insert_after(&mut self, builder: ChainBuilder, index: BuilderIndex) -> BuilderIndex {
        let new_index = self.nodes.len();
        let new_node = match index {
            BuilderIndex::BeforeFirst => {
                let old_head = self.head;
                self.head = BuilderIndex::Some(new_index);
                ChainBuilderNode::new(builder, new_index, old_head.option())
            },
            BuilderIndex::Some(old_index) => {
                self.create_node_after_index(builder, new_index, &old_index)
            }
            BuilderIndex::AfterLast => {
                panic!("Expected index or before first position, got after last");
            }
        };
        debug_assert!(self.nodes.len() == self.structure.len());
        self.structure.push(index);
        self.nodes.push(new_node);
        BuilderIndex::Some(new_index)
    }
    fn create_node_after_index(&mut self, builder: ChainBuilder, new_index: usize, old_index: &usize) -> ChainBuilderNode {
        let mut previous_node: &mut ChainBuilderNode = &mut self.nodes[*old_index];
        let next = previous_node.next();
        previous_node.next = Some(new_index);
        ChainBuilderNode::new(builder, new_index, next)
    }
    #[cfg(test)]
    fn head_index(&self) -> BuilderIndex {
        self.head
    }
    fn next_after(&self, index: BuilderIndex) -> BuilderIndex {
        match index {
            BuilderIndex::BeforeFirst => {
                self.head
            },
            BuilderIndex::Some(last_index) => {
                let node = self.get(last_index).unwrap();
                match node.next() {
                    None => BuilderIndex::AfterLast,
                    Some(next_index) => BuilderIndex::Some(next_index)
                }
            },
            BuilderIndex::AfterLast => {
                panic!("Expected index or before first position, got after last");
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&ChainBuilderNode>{
        self.nodes.get(index)
    }
    pub fn get_builder_mut(&mut self, index: usize) -> Option<&mut ChainBuilder>{
        if let Some(node) = self.nodes.get_mut(index) {
            Some(&mut node.child)
        } else {
            None
        }
    }
    pub fn build_paths(mut self) -> (Vec<Path>, Routes) {
        let mut paths = Vec::new();
        let mut routes = Routes::new(self.structure.drain(..).collect());
        let mut visited: BTreeSet<usize> = BTreeSet::new();
        for node in &self.nodes {
            if !visited.contains(&node.index) {
                let mut next = Some(node);
                let mut previous: Option<usize> = None;
                let mut path_builder = PathBuilder::new();
                let path_index = paths.len();
                while let Some(node) = next {
                    visited.insert(node.index);
                    routes.add_chain_to_path(node.index, path_index);

                    let connection = if let Some(previous) = previous {
                        if node.end_connection.unwrap() == previous {
                            Location::Start
                        } else if node.start_connection.unwrap() == previous {
                            Location::End
                        } else {
                            panic!("Expected connection to previous");
                        }
                    } else {
                        Location::End
                    };
                    let tuple = self.trace(
                        &node.child,
                        connection,
                        path_builder,
                        routes
                    );
                    path_builder = tuple.0;
                    routes = tuple.1;

                    let candidate = match connection {
                        Location::End => node.end_connection.unwrap(),
                        Location::Start => node.start_connection.unwrap()
                    };
                    previous = Some(node.index);
                    next = if visited.contains(&candidate) {
                        None
                    } else {
                        let next_chain = Some(&self.nodes[candidate]);
                        next_chain
                    };
                }
                let path = path_builder.build();
                if !path.is_null() {
                    paths.push(path);
                }
            }
        }
        (paths, routes)
    }
    pub fn connect(&mut self, a: usize, b: usize, location_a: Location, location_b: Location) {
        {
            let node_a = &mut self.nodes[a];
            ChainBuilderNode::connect(node_a, b, location_a);
        }
        {
            let node_b = &mut self.nodes[b];
            ChainBuilderNode::connect(node_b, a, location_b);
        }
    }
    fn trace(
        &self,
        chain: &ChainBuilder,
        towards: Location,
        mut path: PathBuilder,
        routes: Routes
    ) -> (PathBuilder, Routes) {
        match towards {
            Location::End => {
                for (_, point) in chain.chain
                    .iter()
                    .take(chain.chain.len() - 1)
                    .enumerate() {
                    path.add(point);
                }
            },
            Location::Start => {
                for (_, point) in chain.chain
                    .iter()
                    .rev()
                    .take(chain.chain.len() - 1)
                    .enumerate() {
                    path.add(point);
                }
            }
        }

        (path, routes)
    }
    fn to_vec(&self) -> Vec<&ChainBuilderNode> {
        let mut vec = Vec::new();
        let mut current = self.head;
        while current.is_some() {
            vec.push(&self.nodes[current.unwrap()]);
            current = self.next_after(current);
        }
        vec
    }
}
pub struct ChainBuilderNode {
    child: ChainBuilder,
    index: usize,
    next: Option<usize>,
    start_connection: Option<usize>,
    end_connection: Option<usize>
}
impl ChainBuilderNode {
    fn new (child: ChainBuilder, index: usize, next: Option<usize>) -> ChainBuilderNode {
        ChainBuilderNode{
            child,
            index,
            next,
            start_connection: None,
            end_connection: None }
    }
    fn connect(a: &mut ChainBuilderNode, b_index: usize, location: Location) {
        if location == Location::Start {
            debug_assert!(a.start_connection.is_none(), "Can't reassign start");
            a.start_connection = Some(b_index);
        } else {
            debug_assert!(a.end_connection.is_none(), "Can't reassign end");
            a.end_connection = Some(b_index);
        }
    }
    fn inspect(&self) -> String {
        let start = if let Some(index) = self.start_connection {
            index.to_string()
        } else {
            "?".to_string()
        };
        let end = if let Some(index) = self.end_connection {
            index.to_string()
        } else {
            "?".to_string()
        };
        format!("{}<{}>{}#{}", start, self.index, end, self.child.inspect())
    }
    fn next(&self) -> Option<usize> {
        self.next
    }
}

pub struct ChainBuilder {
    chain: Vec<Point>
}
impl ChainBuilder {
    pub fn inspect(&self) -> String {
        let strings: Vec<String> = self.chain.iter().map(|p| p.inspect()).collect();

        format!("{}", strings.join(", "))
    }
    pub fn new(start: &Point) -> ChainBuilder {
        ChainBuilder {
            chain: vec![start.clone()]
        }
    }
    pub fn append(&mut self, point: Point) {
        self.chain.push(point);
    }
    pub fn loose_end(&self) -> &Point {
        self.chain.iter().last().expect("Builder expected to have at least one point")
    }
}

#[cfg(test)]
mod test {
    use crate::{Point};
    use crate::drawing_algorithm::builder::{ChainBuilder, PathsInConstruction, BuilderIndex};

    #[test]
    fn list_mutable_access_test() {
        let p_0 = Point::new(-1, 0).expect("!");
        let p_1 = Point::new(0, 0).expect("!");
        let p_2 = Point::new(-1, 1).expect("!");
        let p0 = Point::new(0, 0).expect("!");
        let p1 = Point::new(1, 0).expect("!");
        let p2 = Point::new(0, 1).expect("!");
        let pb0 = ChainBuilder::new(&p_0);
        let pb1 = ChainBuilder::new(&p_1);
        let pb2 = ChainBuilder::new(&p_2);
        let mut pic = PathsInConstruction::new();
        assert_eq!(pic.next_after(BuilderIndex::BeforeFirst), BuilderIndex::BeforeFirst);
        let i0 = pic.insert_after(pb2, BuilderIndex::BeforeFirst);
        assert_eq!(i0.unwrap(), 0);
        assert_eq!(pic.next_after(BuilderIndex::BeforeFirst), i0);
        assert_eq!(pic.structure[i0.unwrap()], BuilderIndex::BeforeFirst);
        let i1 = pic.insert_after(pb0, BuilderIndex::BeforeFirst);
        assert_eq!(i1.unwrap(), 1);
        assert_eq!(pic.structure[i1.unwrap()], BuilderIndex::BeforeFirst);
        let head_index = pic.head_index().unwrap();
        assert_eq!(head_index, i1.unwrap());
        let builder_mut = pic.get_builder_mut(head_index).unwrap();
        builder_mut.append(p0.clone());
        let current_index = pic.get(head_index).unwrap().next().unwrap();
        assert_eq!(current_index, 0);
        let builder_mut = pic.get_builder_mut(current_index).unwrap();
        builder_mut.append(p2.clone());

        let i2 = pic.insert_after(pb1, BuilderIndex::Some(head_index));
        assert_eq!(i2.unwrap(), 2);
        assert_eq!(pic.structure[i2.unwrap()], BuilderIndex::Some(head_index));

        let builder_mut = pic.get_builder_mut(i2.unwrap()).unwrap();
        builder_mut.append(p1.clone());
        let points: Vec<Point> = pic.to_vec().iter()
            .map(|pb| {
                pb.child.loose_end().clone()
            })
            .collect();
        assert_eq!(&points[0usize], &p0);
        assert_eq!(&points[1usize], &p1);
        assert_eq!(&points[2usize], &p2);
    }

    #[test]
    fn chain_builder_append_test() {
        let p0 = Point::new(0, 0).expect("!");
        let p1 = Point::new(1, 0).expect("!");
        let mut pb = ChainBuilder::new(&p0);
        assert_eq!(pb.loose_end(), &p0);
        pb.append(p1.clone());
        assert_eq!(pb.loose_end(), &p1);
    }
}