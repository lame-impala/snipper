use crate::edge::{Queue, Edge};
use crate::{Point};
use crate::units::{Coordinate, Float};
use crate::intersection_algorithm::{Scope};
use crate::intersection_algorithm::support::Support;
use crate::intersection_algorithm::constraint::Constraint;
use crate::operation::{Operand};
use crate::intersection_algorithm::scope::{Rhs, Lhs};
use std::collections::btree_map::BTreeMap;
use crate::intersection_algorithm::stack::Stack;
use crate::intersection_algorithm::bentley_ottmann::BentleyOttmann;
use crate::intersection_algorithm::dirty_records::{DirtyRecords, Dirty};

fn test_dirty_recs(dr: &DirtyRecords, exp_td: Vec<f64>, exp_bd: Vec<f64>, exp_vd: Vec<i32>) {
    let (td, bd, vd) = dr.clone().explode();
    let td_vec: Vec<f64> = td.iter().map(|float| f64::from(*float)).collect();
    assert_eq!(td_vec, exp_td);
    let bd_vec: Vec<f64> = bd.iter().map(|float| f64::from(*float)).collect();
    assert_eq!(bd_vec, exp_bd);
    let vd_vec: Vec<i32> = vd.iter().map(|coo| coo.to_int()).collect();
    assert_eq!(vd_vec, exp_vd);
}

#[test]
fn initialize_dirty_recs_test() {
    let mut q = Queue::new();
    let p00 = Point::new(0, 0).unwrap();
    let p10 = Point::new(1, 0).unwrap();
    let p01 = Point::new(0, 1).unwrap();
    let p11 = Point::new(1, 1).unwrap();
    let p21 = Point::new(2, 1).unwrap();
    let p12 = Point::new(1, 2).unwrap();
    let p13 = Point::new(1, 3).unwrap();
    let p04 = Point::new(0, 4).unwrap();
    let p24 = Point::new(2, 4).unwrap();
    let p15 = Point::new(1, 5).unwrap();
    let p25 = Point::new(2, 5).unwrap();

    let left0 = Edge::original(0, Operand::Subject, &p00, &p10).unwrap();
    let left1 = Edge::original(1, Operand::Subject, &p01, &p11).unwrap();
    let in_scope0 = Edge::original(2, Operand::Subject, &p11, &p21).unwrap();
    let vertical = Edge::original(3, Operand::Subject, &p12, &p13).unwrap();
    let traverse = Edge::original(4, Operand::Subject, &p04, &p24).unwrap();
    let in_scope1 = Edge::original(5, Operand::Subject, &p15, &p25).unwrap();

    let mut scope = Scope::new(Coordinate::new(1));
    let stack = Stack::new();
    let (stack, _) = scope.insert_edge(left0, Constraint::LOOSE, stack, &mut q).unwrap();
    let (stack, _) = scope.insert_edge(left1, Constraint::LOOSE, stack, &mut q).unwrap();
    let (stack, _) = scope.insert_edge(in_scope0, Constraint::LOOSE, stack, &mut q).unwrap();
    let (stack, _) = scope.insert_edge(in_scope1, Constraint::LOOSE, stack, &mut q).unwrap();
    let (stack, _) = scope.insert_edge(vertical, Constraint::LOOSE, stack, &mut q).unwrap();
    let (_, _) = scope.insert_edge(traverse, Constraint::LOOSE, stack, &mut q).unwrap();

    let dr = scope.initialize_dirty_sets();
    test_dirty_recs(&dr, vec![1.0, 4.0, 5.0], vec![1.0, 4.0, 5.0], vec![2]);
}
#[test]
fn on_position_removed_test() {
    let mut q = Queue::new();
    let p00 = Point::new(0, 0).unwrap();
    let p32 = Point::new(3, 2).unwrap();
    let p01 = Point::new(0, 1).unwrap();
    let p31 = Point::new(3, 1).unwrap();
    let p02 = Point::new(0, 2).unwrap();
    let p30 = Point::new(3, 0).unwrap();

    let e0 = Edge::original(0, Operand::Subject, &p00, &p32).unwrap();
    let e1 = Edge::original(1, Operand::Subject, &p01, &p31).unwrap();
    let e2 = Edge::original(1, Operand::Subject, &p02, &p30).unwrap();

    let mut scope = Scope::new(Coordinate::new(0));
    let stack = Stack::new();
    let (stack, _) = scope.insert_edge(e0, Constraint::LOOSE, stack, &mut q).unwrap();
    let (stack, _) = scope.insert_edge(e1, Constraint::LOOSE, stack, &mut q).unwrap();
    let (_, _) = scope.insert_edge(e2, Constraint::LOOSE, stack, &mut q).unwrap();

    let mut dr = DirtyRecords::new();
    dr.set_dirty(Dirty::Top(Float::from(1)));
    dr.set_dirty(Dirty::Bottom(Float::from(1)));
    dr.set_dirty(Dirty::Vertical(Coordinate::new(1)));
    dr = scope.on_position_removed(&Float::from(1), None, None, dr);
    test_dirty_recs(&dr, vec![2.0], vec![0.0], vec![]);
}
#[test]
fn on_intersection_test() {
    let mut q = Queue::new();
    let p00 = Point::new(0, 0).unwrap();
    let p20 = Point::new(2, 0).unwrap();
    let p01 = Point::new(0, 1).unwrap();
    let p21 = Point::new(2, 1).unwrap();
    let p03 = Point::new(0, 3).unwrap();
    let p23 = Point::new(2, 3).unwrap();
    let p04 = Point::new(0, 4).unwrap();
    let p24 = Point::new(2, 4).unwrap();

    let e0 = Edge::original(0, Operand::Subject, &p00, &p20).unwrap();
    let e1 = Edge::original(1, Operand::Subject, &p01, &p23).unwrap();
    let e2 = Edge::original(2, Operand::Subject, &p03, &p21).unwrap();
    let e3 = Edge::original(3, Operand::Subject, &p04, &p24).unwrap();

    let mut scope = Scope::new(Coordinate::new(0));
    let stack = Stack::new();
    let (stack, _) = scope.insert_edge(e0, Constraint::LOOSE, stack, &mut q).unwrap();
    let (stack, _) = scope.insert_edge(e1, Constraint::LOOSE, stack, &mut q).unwrap();
    let (stack, _) = scope.insert_edge(e2, Constraint::LOOSE, stack, &mut q).unwrap();
    let (stack, _) = scope.insert_edge(e3, Constraint::LOOSE, stack, &mut q).unwrap();

    let mut dr = scope.initialize_dirty_sets();
    dr.undirty(Dirty::Bottom(Float::from(1)));
    dr.undirty(Dirty::Top(Float::from(3)));
    let (_, dr) = scope.intersect(
        &Float::from(1),
        &Float::from(3),
        stack,
        dr,
        &mut q,
    ).unwrap();
    test_dirty_recs(&dr, vec![0.0, 1.0, 4.0], vec![0.0, 3.0, 4.0], vec![]);
}
#[test]
fn on_removal_while_performing_intersection_test() {
    let mut q = Queue::new();
    let p00 = Point::new(0, 0).unwrap();
    let p20 = Point::new(2, 0).unwrap();
    let p01 = Point::new(0, 1).unwrap();
    let p31 = Point::new(3, 1).unwrap();
    let p03 = Point::new(0, 3).unwrap();
    let p33 = Point::new(3, 3).unwrap();
    let p04 = Point::new(0, 4).unwrap();
    let p24 = Point::new(2, 4).unwrap();

    let e0 = Edge::original(0, Operand::Subject, &p00, &p20).unwrap();
    let e1 = Edge::original(1, Operand::Subject, &p01, &p33).unwrap();
    let e2 = Edge::original(2, Operand::Subject, &p03, &p31).unwrap();
    let e3 = Edge::original(3, Operand::Subject, &p04, &p24).unwrap();

    let mut scope = Scope::new(Coordinate::new(0));
    let stack = Stack::new();
    let (stack, _) = scope.insert_edge(e0, Constraint::LOOSE, stack, &mut q).unwrap();
    let (stack, _) = scope.insert_edge(e1, Constraint::LOOSE, stack, &mut q).unwrap();
    let (stack, _) = scope.insert_edge(e2, Constraint::LOOSE, stack, &mut q).unwrap();
    let (stack, _) = scope.insert_edge(e3, Constraint::LOOSE, stack, &mut q).unwrap();

    let dr = DirtyRecords::new();
    let (stack, dr) = scope.intersect(
        &Float::from(1),
        &Float::from(3),
        stack,
        dr,
        &mut q,
    ).unwrap();
    assert_eq!(stack.len(), 2);
    test_dirty_recs(&dr, vec![4.0], vec![0.0], vec![]);
}
#[test]
fn iterator_test() {
    let l0 = Point::new(-10, -10).unwrap();
    let l1 = Point::new(-10, 0).unwrap();
    let l2 = Point::new(-10, 10).unwrap();

    let c0 = Point::new(0, 0).unwrap();
    let c1 = Point::new(0, 20).unwrap();


    let r0 = Point::new(10, -10).unwrap();
    let r1 = Point::new(10, 10).unwrap();

    let t0 = Edge::original(0, Operand::Subject, &l0, &r0).unwrap().clone();
    let s0 = Edge::original(1, Operand::Subject, &l0, &c0).unwrap().clone();
    let s1 = Edge::original(2, Operand::Subject, &l1, &c0).unwrap().clone();
    let t1 = Edge::original(3, Operand::Subject, &l1, &r1).unwrap().clone();
    let s2 = Edge::original(4, Operand::Subject, &l1, &c1).unwrap().clone();
    let s3 = Edge::original(5, Operand::Subject, &l2, &c1).unwrap().clone();

    let left = BTreeMap::new();
    let pos = BTreeMap::new();

    let mut q = Queue::new();
    let mut s = Scope::build(left, pos, &Coordinate::new(0), &mut q).unwrap();
    let mut stack = Stack::new();

    let r = s.insert_edge(t0, Constraint::LOOSE, stack, &mut q).unwrap();
    stack = r.0;
    let r = s.insert_edge(s0, Constraint::LOOSE, stack, &mut q).unwrap();
    stack = r.0;
    let r = s.insert_edge(s1, Constraint::LOOSE, stack, &mut q).unwrap();
    stack = r.0;
    let r = s.insert_edge(t1, Constraint::LOOSE, stack, &mut q).unwrap();
    stack = r.0;
    let r = s.insert_edge(s2, Constraint::LOOSE, stack, &mut q).unwrap();
    stack = r.0;
    let r = s.insert_edge(s3, Constraint::LOOSE, stack, &mut q).unwrap();
    stack = r.0;
    assert!(stack.is_empty());
    let i = s.iter();
    for (index, ray) in i.enumerate() {
        assert_eq!(index, ray.edge().index, "Expected {}, got {}", index, ray.edge().inspect());
    }
}
#[test]
fn insert_run_through_edge_to_support() {
    let mut gb = Queue::new();
    let mut lhs = Lhs::new();


    let point = Point::new(10, 5).unwrap();
    let support = Support::new(point.clone());
    let s = Point::new(0, 0).unwrap();
    let e = Point::new(20, 10).unwrap();

    let edge = Edge::original(0, Operand::Subject, &s, &e).unwrap();

    let snippet = support.insert_traverse(edge, &mut gb, &mut lhs).unwrap();
    assert!(snippet.is_some());
    assert_eq!(support.left_hand_side(&lhs).count(), 1);
    let (_, left) = support.left_hand_side(&lhs).last().unwrap();
    assert_eq!(left.edge().upper_left(), &s);
    assert_eq!(left.edge().lower_right(), &point);
    let right = snippet.as_ref().unwrap().right.as_ref().unwrap();
    assert_eq!(right.upper_left(), &point);
    assert_eq!(right.lower_right(), &e);
}
#[test]
fn dirty_by_inserting() {
    let mut lhs = Lhs::new();
    let mut rhs = Rhs::new();
    let point = Point::new(0, 0).unwrap();
    let support = Support::new(point.clone());

    let end0 = Point::new(10, 0).unwrap();
    let end1 = Point::new(10, -10).unwrap();
    let end2 = Point::new(10, 10).unwrap();
    let end3 = Point::new(0, 10).unwrap();

    let edge0 = Edge::original(0, Operand::Subject, &point, &end0).unwrap();
    let edge1 = Edge::original(1, Operand::Subject, &point, &end1).unwrap();
    let edge2 = Edge::original(2, Operand::Subject, &point, &end2).unwrap();
    let edge3 = Edge::original(3, Operand::Subject, &point, &end3).unwrap();

    let mut gb = Queue::new();

    let (snippet, td, bd, vd) = support.insert(edge0, &mut gb, &mut lhs, &mut rhs).unwrap();
    assert!(td, "Should be top-dirty");
    assert!(bd, "Should be bottom-dirty");
    assert!(!vd, "Shouldn't be vertical-dirty");
    assert_eq!(support.right_hand_side(&rhs).count(), 1);
    assert!(snippet.is_none());

    let (snippet, td, bd, vd) = support.insert(edge1, &mut gb, &mut lhs, &mut rhs).unwrap();
    assert!(td, "Should be top-dirty");
    assert!(!bd, "Shouldn't be bottom-dirty");
    assert!(!vd, "Shouldn't be vertical-dirty");
    assert_eq!(support.right_hand_side(&rhs).count(), 2);
    assert!(snippet.is_none());

    let (snippet, td, bd, vd) = support.insert(edge2, &mut gb, &mut lhs, &mut rhs).unwrap();
    assert!(!td, "Shouldn't be top-dirty");
    assert!(bd, "Should be bottom-dirty");
    assert!(!vd, "Shouldn't be vertical-dirty");
    assert_eq!(support.right_hand_side(&rhs).count(), 3);
    assert!(snippet.is_none());


    let (snippet, td, bd, vd) = support.insert(edge3, &mut gb, &mut lhs, &mut rhs).unwrap();
    assert!(!td, "Shouldn't be top-dirty");
    assert!(!bd, "Shouldn't be bottom-dirty");
    assert!(vd, "Should be vertical-dirty");
    assert_eq!(support.right_hand_side(&rhs).count(), 3);
    assert!(snippet.is_none());
}