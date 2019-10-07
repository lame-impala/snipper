use std::collections::{HashMap, HashSet};
use crate::drawing_algorithm::builder::BuilderIndex;
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum FirstIndex {
    Unkwnown,
    Random(usize),
    Confirmed(usize)
}
impl FirstIndex {
    pub fn expect(&self, msg: &'static str) -> usize {
        match self {
            FirstIndex::Unkwnown => panic!(msg),
            FirstIndex::Random(index) => *index,
            FirstIndex::Confirmed(index) => *index
        }
    }
    pub fn is_none(&self) -> bool {
        match self {
            FirstIndex::Unkwnown => true,
            FirstIndex::Random(_) => false,
            FirstIndex::Confirmed(_) => false
        }
    }
}
pub struct Routes {
    chains: Vec<Option<usize>>,
    paths: Vec<FirstIndex>,
    structure: Vec<BuilderIndex>
}
impl Routes {
    pub fn new (structure: Vec<BuilderIndex>) -> Routes {
        Routes{
            chains: Vec::new(), paths: Vec::new(), structure
        }
    }
    pub fn structure(&self) -> &Vec<BuilderIndex> {
        &self.structure
    }
    pub fn confirm_first(&mut self, index: usize, path_index: &usize) {
        self.paths[*path_index] = FirstIndex::Confirmed(index)
    }
    pub fn first(&self, path_index: &usize) -> FirstIndex {
        self.paths.get(*path_index).expect("Path not there").clone()

    }
    pub fn add_chain_to_path(&mut self, chain_index: usize, path_index: usize) {
        if self.chains.len() <= chain_index {
            self.chains.resize(chain_index + 1, None);
        }
        debug_assert!(self.chains[chain_index] == None, "Can't reassign");
        self.chains[chain_index] = Some(path_index);
        if self.paths.len() <= path_index {
            self.paths.resize(path_index + 1, FirstIndex::Unkwnown);
        }
        if self.paths[path_index].is_none() {
            self.paths[path_index] = FirstIndex::Random(chain_index);
        }
    }
    pub fn belongs_to_path(&self, chain_index: &usize, candidate: &usize) -> bool {
        if let Some(path_index) = &self.chains[*chain_index] {
            path_index == candidate
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn inspect(&self) -> String {
        let groups: HashMap<usize, HashSet<usize>> = HashMap::new();
        let acc = self.chains
            .iter()
            .enumerate()
            .fold(groups, |mut acc, (index, path_index)| {
                if let Some(path_index) = path_index {
                    acc.entry(*path_index).or_insert(HashSet::new()).insert(index);
                }
                acc
            });
        let vec: Vec<String> = acc.iter().map(|(path_index, chains)| {
            let strings: Vec<String> = chains.iter().map(|chain_index| format!("{}", chain_index)).collect();
            format!("[{}] {}", path_index, strings.join(", "))
        }).collect();
        vec.join("; ")
    }
}
