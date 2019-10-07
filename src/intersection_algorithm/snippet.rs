use crate::edge::{Edge};
use crate::edge::queue::AbstractQueue;
use crate::{Point, Error};
use crate::intersection_algorithm::constraint::Constraint;
use crate::units::Pseudoangle;

pub struct Snippets {
    pub vec: Vec<Snippet>
}
impl Snippets {
    pub fn new() -> Snippets {
        Snippets{ vec: Vec::new()}
    }
    pub fn push(&mut self, snippet: Snippet) {
        if !snippet.is_empty() {
            self.vec.push(snippet);
        }
    }
    pub fn push_option(&mut self, option: Option<Snippet>) {
        if let Some(snippet) = option {
            self.push(snippet);
        }
    }
    pub fn extend(&mut self, snippets: Snippets) {
        self.vec.extend(snippets.vec)
    }
    #[allow(dead_code)]
    pub fn inspect(&self) -> String {
        let strings: Vec<String> = self.vec.iter().map(|s| s.inspect()).collect();
        strings.join(", ")
    }
}
pub struct Snippet {
    pub original_angle: Pseudoangle,
    pub left: Option<(Edge, Constraint)>,
    pub right: Option<Edge>
}
impl Snippet {
    pub fn new(original_angle: Pseudoangle, left: Option<(Edge, Constraint)>, right: Option<Edge>) -> Snippet {
        Snippet{
            original_angle, left, right
        }
    }
    pub fn is_empty(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
    pub fn take_left(self) -> (Option<(Edge, Constraint)>, Snippet) {
        let left = self.left;
        let new = Snippet::new(self.original_angle, None, self.right);
        (left, new)
    }
    #[cfg(test)]
    pub fn take_right(self) -> (Option<Edge>, Snippet) {
        let right = self.right;
        let new = Snippet::new(self.original_angle, self.left, None);
        (right, new)
    }
    pub fn snip(point: &Point, edge: Edge, constraint: &Constraint, queued_edges: &mut dyn AbstractQueue) -> Result<Snippet, Error> {
        let original_angle = edge.pseudoangle_for_upper_left();
        let left_index = edge.index;
        let right_index = queued_edges.next_edge_index()?;
        let (left, right) = edge.left_split(point, left_index, right_index);
        let left_option = if let Some(left) = left {
            Some((left, *constraint))
        } else {
            None
        };
        Ok(Snippet::new(original_angle, left_option, right))
    }
    pub fn inspect(&self) -> String {
        let left = if let Some(left) = self.left.as_ref() {
            left.0.inspect()
        } else {
            "None".to_string()
        };
        let right = if let Some(right) = self.right.as_ref() {
            right.inspect()
        } else {
            "None".to_string()
        };
        format!(
            "Snippet -- {} / {}", left, right
        )
    }
}

#[cfg(test)]
mod test {
    use crate::intersection_algorithm::snippet::{Snippet};
    use crate::{Point};
    use crate::edge::{Queue, Edge};
    use crate::intersection_algorithm::constraint::Constraint;
    use crate::operation::Operand;

    #[test]
    fn take_left_test() {
        let o = Edge::original(0, Operand::Subject,
            &Point::new(0, 0).unwrap(), &Point::new(20, 0).unwrap()
        ).unwrap();
        let e = Edge::original(0, Operand::Subject,
            &Point::new(0, 0).unwrap(), &Point::new(10, 0).unwrap()
        ).unwrap();
        let s = Snippet::new(o.pseudoangle, Some((e, Constraint::LOOSE)), None);
        let (e, s) = s.take_left();
        assert!(e.is_some());
        assert!(s.is_empty());
    }
    #[test]
    fn right_test() {
        let o = Edge::original(0, Operand::Subject,
            &Point::new(0, 0).unwrap(), &Point::new(20, 0).unwrap()
        ).unwrap();
        let e = Edge::original(0, Operand::Subject,
            &Point::new(0, 0).unwrap(), &Point::new(10, 0).unwrap()
        ).unwrap();
        let s = Snippet::new(o.pseudoangle, None, Some(e));
        let (e, s) = s.take_right();
        assert!(e.is_some());
        assert!(s.is_empty());
    }
    #[test]
    fn snip_test() {
        let p00_00 = Point::new(0, 0).expect("!");
        let p10_10 = Point::new(10, 10).expect("!");
        let p30_00 = Point::new(30, 00).expect("!");
        let p00_30 = Point::new(00, 30).expect("!");
        let e = Edge::original(0, Operand::Clipping, &p00_00, &p30_00).unwrap();

        let mut gb = Queue::new();

        {
            let s = Snippet::snip(&p10_10, e, &Constraint::LOOSE,&mut gb).unwrap();
            assert!(s.left.is_some());
            assert!(s.right.is_some());
            let left = &s.left.unwrap().0;
            let right = &s.right.unwrap();
            assert_eq!(left.upper_left(), &p00_00);
            assert_eq!(left.lower_right(), &p10_10);
            assert_eq!(right.upper_left(), &p10_10);
            assert_eq!(right.lower_right(), &p30_00);
        }
        let e = Edge::original(0, Operand::Clipping, &p00_00, &p00_30).unwrap();
        {
            let s = Snippet::snip(&p00_00, e, &Constraint::LOOSE, &mut gb).unwrap();
            assert!(s.left.is_none());
            assert!(s.right.is_some());
            let right = &s.right.unwrap();
            assert_eq!(right.upper_left(), &p00_00);
            assert_eq!(right.lower_right(), &p00_30);
        }
    }
}
