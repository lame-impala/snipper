use crate::{Coordinate, Point, AbstractPoint};
use std::collections::vec_deque::VecDeque;
use std::collections::btree_map::BTreeMap;

pub struct BuilderList {
    points: BTreeMap<Point, VecDeque<usize>>
}
impl BuilderList {
    #[cfg(test)]
    pub fn points(&self) -> &BTreeMap<Point, VecDeque<usize>> {
        &self.points
    }
    pub fn inspect(&self) -> String {
        let strings: Vec<String> = self.points.iter().map(|(point, vec)| {
            let strings: Vec<String> = vec.iter().map(|chain| format!("{}", chain)).collect();
            format!("At {} --> [{}]", point.inspect(), strings.join(", "))
        }).collect();
        format!("BuilderList: {}", strings.join("; "))
    }
    pub fn new() -> BuilderList {
        BuilderList {
            points: BTreeMap::new()
        }
    }
    pub fn pop_front(&mut self, key: &Point) -> Option<usize> {
        if let Some(deque) = self.points.get_mut(key) {
            deque.pop_front()
        } else {
            None
        }
    }
    pub fn pop_back(&mut self, key: &Point) -> Option<usize> {
        if let Some(deque) = self.points.get_mut(key) {
            deque.pop_back()
        } else {
            None
        }
    }
    pub fn push_back(&mut self, point: Point, chain_index: usize) {
        self.points.entry(point).or_insert(VecDeque::new()).push_back(chain_index);
    }
    #[cfg(test)]
    pub fn push_front(&mut self, point: Point, chain_index: usize) {
        self.points.entry(point).or_insert(VecDeque::new()).push_front(chain_index);
    }
    pub fn chunks(&self) -> ChunksIterator {
        ChunksIterator::new(self)
    }
    pub fn split_off(mut self, x: Coordinate) -> (BuilderList, BuilderList) {
        let min = Coordinate::new(std::i32::MIN);
        let rim = Point::unchecked(x, min);
        let right = self.points.split_off(&rim);
        let terminated = BuilderList { points: self.points };
        let active = BuilderList { points: right };
        (terminated, active)
    }
    pub fn extend(mut self, other: BuilderList) -> BuilderList {
        self.points.extend(other.points);
        self
    }
}
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Chunk<'list> {
    Complete(usize, usize),
    Partial(usize, &'list Point)
}
pub struct ChunksIterator<'list> {
    map_iter: std::collections::btree_map::Iter<'list, Point, VecDeque<usize>>,
    vec_iter: Option<(std::collections::vec_deque::Iter<'list, usize>, &'list Point)>
}
impl <'list> ChunksIterator<'list>{
    pub fn new(list: &'list BuilderList) -> ChunksIterator {
        let map_iter = list.points.iter();
        ChunksIterator{
            map_iter, vec_iter: None
        }
    }
}
impl <'list> Iterator for ChunksIterator<'list> {
    type Item = Chunk<'list>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut stop = false;
        let mut item: Option<Chunk> = None;
        while !stop {
            if let Some((vec_iter, point)) = &mut self.vec_iter {
                if let Some(next) = vec_iter.next() {
                    if let Some(second_next) = vec_iter.next() {
                        item = Some(Chunk::Complete(*next, *second_next));
                    } else {
                        item = Some(Chunk::Partial(*next, point));
                    }
                    stop = true;
                } else {
                    if let Some((point, deque)) = self.map_iter.next() {
                        self.vec_iter = Some((deque.iter(), point));
                    } else {
                        stop = true;
                    }
                }
            } else {
                if let Some((point, deque)) = self.map_iter.next() {
                    self.vec_iter = Some((deque.iter(), point));
                } else {
                    stop = true;
                }
            }
        }
        item
    }
}
#[cfg(test)]
mod test {
    use crate::drawing_algorithm::builder_list::{BuilderList, Chunk};
    use crate::{Point, Coordinate};

    #[test]
    fn push_pop_test() {
        let mut bl = BuilderList::new();
        let p00 = Point::new(0, 0).unwrap();
        let p01 = Point::new(0, 1).unwrap();

        bl.push_back(p00.clone(), 1);
        bl.push_front(p00.clone(), 0);
        bl.push_back(p01.clone(), 3);
        bl.push_front(p01.clone(), 2);

        let first = bl.chunks().nth(0).unwrap();
        assert_eq!(first, Chunk::Complete(0, 1));
        let second = bl.chunks().nth(1).unwrap();
        assert_eq!(second, Chunk::Complete(2, 3));

        let f = bl.pop_front(&p00).unwrap();
        assert_eq!(f, 0);
        let f = bl.pop_back(&p01).unwrap();
        assert_eq!(f, 3);

    }
    #[test]
    fn split_off_test() {
        let mut bl = BuilderList::new();
        let p00 = Point::new(0, 0).unwrap();
        let p01 = Point::new(0, 1).unwrap();
        let p10 = Point::new(1, 0).unwrap();
        let p11 = Point::new(1, 1).unwrap();

        bl.push_back(p00.clone(), 0);
        bl.push_back(p00.clone(), 1);
        bl.push_back(p01.clone(), 2);
        bl.push_back(p01.clone(), 3);
        bl.push_back(p10.clone(), 4);
        bl.push_back(p10.clone(), 5);
        bl.push_back(p11.clone(), 6);
        bl.push_back(p11.clone(), 7);

        let (t, a) = bl.split_off(Coordinate::new(1));
        let first = t.chunks().nth(0).unwrap();
        assert_eq!(first, Chunk::Complete(0, 1));
        let second = t.chunks().nth(1).unwrap();
        assert_eq!(second, Chunk::Complete(2, 3));

        let first = a.chunks().nth(0).unwrap();
        assert_eq!(first, Chunk::Complete(4, 5));
        let second = a.chunks().nth(1).unwrap();
        assert_eq!(second, Chunk::Complete(6, 7));

    }
}