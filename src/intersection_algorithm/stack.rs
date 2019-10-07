use crate::edge::Edge;
use crate::intersection_algorithm::constraint::Constraint;

pub struct Stack {
    stack: Vec<Stacked>
}
impl Stack {
    pub fn new() -> Stack {
        Stack{ stack: Vec::new() }
    }
    pub fn push(&mut self, item: Stacked) {
        self.stack.push(item);
    }
    pub fn pop(&mut self) -> Option<(Stacked)> {
        self.stack.pop()
    }
    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
    pub fn len(&self) -> usize {
        self.stack.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[allow(dead_code)]
    pub fn inspect(&self) -> String {
        let strings: Vec<String> = self.stack.iter().map(|stacked| {
            stacked.edge.inspect()
        }).collect();
        format!("Stack: {}", strings.join(", "))
    }
}
pub struct Stacked {
    pub edge: Edge,
    pub constraint: Constraint
}
