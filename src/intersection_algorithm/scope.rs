use crate::units::{Float, Pseudoangle};
use crate::primitives::{Intersection, Vector, Line};
use crate::intersection_algorithm::position::{PositionType, Position};
use crate::edge::queue::AbstractQueue;
use crate::{Point, AbstractPoint, Error, Coordinate};
use crate::intersection_algorithm::snippet::{Snippets, Snippet};
use std::collections::btree_map::BTreeMap;
use crate::edge::Edge;
use crate::intersection_algorithm::constraint::Constraint;
use crate::intersection_algorithm::stack::{Stack, Stacked};
use itertools::{Either};
use crate::intersection_algorithm::support::Support;
use crate::intersection_algorithm::traverse::Traverse;
use crate::intersection_algorithm::ray::Ray;
use std::iter::{Peekable, Filter};
use std::cmp::Ordering;
use crate::intersection_algorithm::bentley_ottmann::BentleyOttmann;
use crate::intersection_algorithm::dirty_records::{InsertionResult, Removals};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Removed {
    Nothing, Ray, Position
}
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Upwards, Downwards
}
impl Direction {
    fn invert(&self) -> Direction {
        match self {
            Direction::Upwards => Direction::Downwards,
            Direction::Downwards => Direction::Upwards
        }
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Key {
    y: Float,
    angle: Pseudoangle
}
impl Key {
    pub fn new(y: Float, angle: Pseudoangle) -> Key {
        Key{ y, angle }
    }
    pub fn reversed(self) -> ReversedKey {
        ReversedKey { y: self.y, angle: self.angle }
    }
}
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ReversedKey {
    y: Float,
    angle: Pseudoangle
}
impl ReversedKey {
    pub fn new(y: Float, angle: Pseudoangle) -> ReversedKey {
        ReversedKey{ y, angle }
    }
    pub fn y(&self) -> Float {
        self.y
    }
}
impl PartialOrd for ReversedKey {
    fn partial_cmp(&self, other: &ReversedKey) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ReversedKey {
    fn cmp(&self, other: &ReversedKey) -> Ordering {
        if self.y == other.y {
            other.angle.cmp(&self.angle)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

pub type Lhs = BTreeMap<ReversedKey, Ray>;
pub type Rhs = BTreeMap<Key, Ray>;
pub struct Scope {
    line: Line,
    positions: BTreeMap<Float, Position>,
    lhs: Lhs,
    rhs: Rhs,
    next_x: Option<Coordinate>
}
impl Scope {
    pub fn new(
        x: Coordinate
    ) -> Scope{
        let line = Scope::create_base(x);
        Scope{
            line,
            positions: BTreeMap::new(),
            lhs: BTreeMap::new(),
            rhs: BTreeMap::new(),
            next_x: None
        }
    }
    fn pop_batch(at: Coordinate, queued_edges: &mut dyn AbstractQueue) -> Result<(Scope, Stack), Error> {
        let mut scope = Scope::new(at);
        let mut stack = Stack::new();
        while let Some(edge) = queued_edges.pop_edge(at) {
            let tuple = scope.insert_edge(
                edge,
                Constraint::LOOSE,
                stack,
                queued_edges
            )?;
            stack = tuple.0;
        }
        Ok((scope, stack))
    }
    fn edge_to_stack(&self, edge: Edge, constraint: Constraint, mut stack: Stack, queued_edges: &mut dyn AbstractQueue) -> Stack {
        if edge.upper_left().x() <= self.x() {
            let stacked = Stacked{
                edge,
                constraint
            };
            stack.push(stacked)
        } else {
            queued_edges.push_edge(edge); // Not in scope yet
        }
        stack
    }
    fn check_minimum(&mut self, candidate: Coordinate) {
        if candidate > self.x() {
            if let Some(min) = self.next_x {
                if min > candidate {
                    self.next_x = Some(candidate);
                }
            } else {
                self.next_x = Some(candidate);
            }
        }
    }
    fn insert_edge_from_left(&mut self, edge: Edge, constraint: Constraint, queued_edges: &mut dyn AbstractQueue) -> Result<Snippets, Error> {
        self.check_minimum(edge.lower_right().x());
        debug_assert!(edge.upper_left().x() < self.x(), "Expected edge start to the left of the sweep line");
        if edge.lower_right().x() == self.x() {
            let y = edge.lower_right().y();
            let (snippets, _) = self.safe_create_support(
                y, queued_edges
            ).unwrap();
            let support = self.get_support(&y).unwrap();
            support.insert_to_left(edge, &mut self.lhs);
            Ok(snippets)
        } else {
            let (snippets, _) = self.insert_traverse(edge, constraint, queued_edges)?;
            Ok(snippets)
        }
    }
    pub fn build(
        inbound: BTreeMap<Key, Ray>,
        positions: BTreeMap<Float, Position>,
        x: &Coordinate,
        queued_edges: &mut dyn AbstractQueue
    ) -> Result<Scope, Error> {
        let (mut scope, mut stack) = Scope::pop_batch(*x, queued_edges)?;
        stack = scope.insert_rays_from_left(inbound, positions, stack, queued_edges)?;
        scope.find_intersections(stack, queued_edges)?;
        Ok(scope)
    }
    fn insert_rays_from_left(
        &mut self,
        rays: BTreeMap<Key, Ray>,
        positions: BTreeMap<Float, Position>,
        mut stack: Stack,
        queued_edges: &mut dyn AbstractQueue
    ) -> Result<Stack, Error> {
        for (key, ray) in rays {
            let constraint = match positions.get(&key.y) {
                None => &Constraint::LOOSE,
                Some(position) => match &position.either {
                    Either::Left(_) => &Constraint::LOOSE,
                    Either::Right(traverse) => &traverse.constraint
                }
            };
            let point = ray.edge().straight.lower_right();
            debug_assert!(point.x() >= self.x());
            if point.x() == self.x() {
                let (snippets, _) = self.safe_create_support(point.y(), queued_edges)?;
                let support = self.get_support(&point.y()).unwrap();
                let reverse = ray.reverse();
                debug_assert!(!support.contains_left_hand_side_ray(&reverse.angle, &self.lhs), "Ray already there: {}", support.inspect(&self.lhs, &self.rhs));
                support.insert_ray_to_left(reverse.angle, reverse, &mut self.lhs);
                stack = self.enqueue_snippets(snippets, stack, queued_edges);
            } else {
                let snippets = self.insert_edge_from_left(ray.take_edge(), *constraint, queued_edges)?;
                stack = self.enqueue_snippets(snippets, stack, queued_edges);
            }
        }
        Ok(stack)
    }

    pub fn left_hand_edges(&self) -> Vec<Edge> {
        self.lhs.iter().map(|(_, ray)| {
            ray.edge().clone()
        }).collect()
    }
    pub fn pass_over(self) -> (
        Rhs,
        BTreeMap<Float, Position>,
        Option<Coordinate>
    ) {
        (self.rhs, self.positions, self.next_x)
    }
    fn remove_traverse(&mut self, y: &Float) {
        let position = self.positions
            .remove(y)
            .expect("Position expected to be there");
        if let Either::Right(traverse) = position.either {
            let key = traverse.key(&self.rhs);
            self.rhs.remove(&key).expect("Ray expected to exist");
        } else {
            panic!("Traverse expected");
        }
    }


    fn insert_traverse(
        &mut self,
        edge: Edge,
        constraint: Constraint,
        queued_edges: &mut dyn AbstractQueue
    ) -> Result<(Snippets, InsertionResult), Error> {
        debug_assert!(edge.lower_right().x() > self.x());
        let y = edge.straight.cross_with_vertical(self.x()).unwrap();
        let x = self.x();
        if let Some(position) = self.positions.get_mut(&y) {
            match &mut position.either {
                Either::Left(support) => {
                    Scope::insert_traverse_to_support(
                        edge,
                        support as &Support,
                        &mut self.lhs,
                        queued_edges
                    )
                },
                Either::Right(traverse) => {
                    let ray = Traverse::ray(traverse.float_y(), &self.rhs);
                    if traverse.can_take(&edge, ray) {
                        let merged = Constraint::merge(&constraint, &traverse.constraint);
                        if merged.is_too_tight(
                            &edge.straight,
                            &edge.pseudoangle_for_upper_left(),
                            &x) {
                            let preferred = traverse.preferred_point(&x);
                            let y = traverse.float_y();
                            self.snip_too_tightly_constrained_traverse(
                                edge,
                                preferred,
                                y,
                                merged,
                                queued_edges,
                            )
                        } else {
                            Scope::insert_to_existing_traverse(
                                edge,
                                x,
                                y,
                                merged,
                                traverse,
                                &mut self.rhs,
                                queued_edges,
                            )
                        }
                    } else {
                        self.snip_intersecting_traverses(
                            edge,
                            y,
                            constraint,
                            queued_edges,
                        )
                    }
                }
            }
        } else {
            let constraint = self.constraint(&constraint, &edge, &y);
            if constraint.is_too_tight(
                &edge.straight, &edge.pseudoangle_for_upper_left(),
                &(self.x())
            ) {
                self.snip_too_tightly_constrained_edge(
                    edge,
                    y,
                    constraint,
                    queued_edges,
                )
            } else {
                self.insert_to_fresh_traverse(
                    edge,
                    y,
                    constraint
                )
            }
        }
    }

    fn insert_to_fresh_traverse(
        &mut self,
        edge: Edge,
        y: Float,
        constraint: Constraint
    ) -> Result<(Snippets, InsertionResult), Error> {
        let traverse = Traverse::new(y, constraint);
        let angle = edge.pseudoangle_for_upper_left();
        let ray = Ray::new(edge, angle);
        let key = Key::new(y, ray.angle);
        self.rhs.insert(key, ray);
        self.positions.insert(y, Position::traverse(traverse));
        let result = InsertionResult::traverse_inserted(
            y, true, true, None
        );
        Ok((Snippets::new(), result))
    }

    fn snip_too_tightly_constrained_edge(
        &mut self,
        edge: Edge,
        y: Float,
        constraint: Constraint,
        queued_edges: &mut dyn AbstractQueue,
    ) -> Result<(Snippets, InsertionResult), Error> {
        let preferred = Traverse::preferred_position(y);
        let point = Point::unchecked(self.x(), preferred);
        let snippet = Snippet::snip(&point, edge, &constraint, queued_edges)?;
        let mut snippets = Snippets::new();
        snippets.push(snippet);
        let removals = if let Some(obstacles) = self.obstacles(&y, &Float::from(preferred)) {
            let tuple = self.wipe_range(obstacles, &point, queued_edges)?;
            snippets.extend(tuple.0);
            Some(tuple.1)
        } else {
            None
        };
        let result = InsertionResult::traverse_inserted(
            Float::from(preferred), false, false, removals
        );
        Ok((snippets, result))
    }

    fn snip_intersecting_traverses(
        &mut self,
        edge: Edge,
        y: Float,
        constraint: Constraint,
        queued_edges: &mut dyn AbstractQueue,
    ) -> Result<(Snippets, InsertionResult), Error> {
        let mut snippets = Snippets::new();
        let point = Intersection::proper(
            &edge.straight,
            &Traverse::edge(y, &self.rhs).straight
        ).expect("Intersection point expected to be there");
        let (snippet, removed_element) = self.snip_ray(
            &y,
            Direction::Downwards,
            &point,
            queued_edges
        )?;
        let removals = match removed_element {
            Removed::Position => {
                let mut removals = Removals::new();
                removals.add(y);
                Some(removals)
            },
            Removed::Nothing | Removed::Ray => panic!("Expected Position")
        };
        snippets.push_option(snippet);
        let snippet = Snippet::snip(&point, edge, &constraint, queued_edges)?;
        snippets.push(snippet);
        let result = InsertionResult::traverse_inserted(
            y, false, false, removals
        );
        Ok((snippets, result))
    }

    fn insert_to_existing_traverse(
        edge: Edge,
        x: Coordinate,
        y: Float,
        merged: Constraint,
        traverse: &mut Traverse,
        rhs: &mut Rhs,
        queued_edges: &mut dyn AbstractQueue,
    ) -> Result<(Snippets, InsertionResult), Error> {
        let ray = Traverse::ray_mut(y, rhs);
        traverse.insert(edge, x, queued_edges, ray);
        traverse.constraint = merged;
        let result = InsertionResult::traverse_inserted(
            y, false, false, None
        );
        Ok((Snippets::new(), result))
    }

    fn snip_too_tightly_constrained_traverse(
        &mut self,
        edge: Edge,
        preferred: Point,
        y: Float,
        merged: Constraint,
        queued_edges: &mut dyn AbstractQueue,
    ) -> Result<(Snippets, InsertionResult), Error> {
        let mut snippets = Snippets::new();
        let (snippet, removed_element) = self.snip_ray(
            &y,
            Direction::Downwards,
            &preferred,
            queued_edges
        )?;
        debug_assert!(removed_element == Removed::Position);
        snippets.push(snippet.expect("Snipping traverse must produce snippet"));
        let snippet = Snippet::snip(&preferred, edge, &merged, queued_edges)?;
        snippets.push(snippet);
        let mut removals = if let Some(obstacles) = self.obstacles(&y, &Float::from(preferred.y())) {
            let tuple = self.wipe_range(obstacles, &preferred, queued_edges)?;
            snippets.extend(tuple.0);
            tuple.1
        } else {
            Removals::new()
        };
        removals.add(y);
        let result = InsertionResult::traverse_inserted(
            y, false, false, Some(removals)
        );
        Ok((snippets, result))
    }

    fn insert_traverse_to_support(
        edge: Edge,
        support: &Support,
        lhs: &mut Lhs,
        queued_edges: &mut dyn AbstractQueue,
    ) -> Result<(Snippets, InsertionResult), Error> {
        let snippet = support.insert_traverse(edge, queued_edges, lhs)?;
        let mut snippets = Snippets::new();
        snippets.push(snippet.expect("Traverse to support must produce snippet"));
        let result = InsertionResult::traverse_inserted(
            support.float_y(), false, false, None
        );
        Ok((snippets, result))
    }
    fn support_in_direction(&self, y: &Float, dir: Direction) -> Option<Float> {
        match dir {
            Direction::Upwards => self.support_above(y),
            Direction::Downwards => self.support_below(y)
        }
    }
    fn support_above(&self, y: &Float) -> Option<Float> {
        if let Some(above) = self.neighbor_above(y) {
            let position = self.positions.get(&above).unwrap();
            match position.either {
                Either::Left(_) => Some(above),
                _ => None
            }
        } else {
            None
        }
    }

    fn neighbor(&self, y: &Float, dir: Direction) -> Option<Float> {
        match dir {
            Direction::Upwards => self.neighbor_above(y),
            Direction::Downwards => self.neighbor_below(y)
        }
    }
    fn neighbor_above(&self, y: &Float) -> Option<Float> {
        if let Some((float, _)) = self.positions.range(..y).next_back() {
            Some(*float)
        } else {
            None
        }
    }
    fn support_below(&self, y: &Float) -> Option<Float> {
        if let Some(below) = self.neighbor_below(y) {
            let position = self.positions.get(&below).unwrap();
            match position.either {
                Either::Left(_) => Some(below),
                _ => None
            }
        } else {
            None
        }
    }
    fn constraint_in_direction(
        &self,
        mut constraint: Constraint,
        edge: &Edge,
        cross: &Float,
        dir: Direction
    ) -> Constraint {
        let start = edge.upper_left();
        if let Some(neighbor) = self.support_in_direction(cross, dir) {
            let point = Point::unchecked(
                self.x(),
                Coordinate::from_float(f64::from(neighbor)).unwrap());
            let vector = Vector::new(start, &point);
            let angle = vector.pseudoangle().unwrap();
            constraint = constraint.constrain_direction(&angle, dir);
        }
        constraint
    }
    fn constraint(&self, constraint: &Constraint, edge: &Edge, cross: &Float) -> Constraint {
        let mut constraint = constraint.clone();
        constraint = self.constraint_in_direction(constraint, edge, cross, Direction::Upwards);
        constraint = self.constraint_in_direction(constraint, edge, cross, Direction::Downwards);
        constraint
    }
    fn constrain_neighbor(&mut self, y: &Coordinate, dir: Direction, queued_edges: &mut dyn AbstractQueue) -> Result<(Snippets, Vec<Float>), Error> {
        let float = Float::from(y);
        let point = Point::unchecked(self.x(), *y);
        let mut snippets = Snippets::new();
        let mut removed = Vec::new();
        let x = self.x();
        loop {
            if let Some(neighbor) = self.neighbor(&float, dir) {
                if let Some(traverse) = self.get_traverse(&neighbor) {
                    let edge = Traverse::edge(traverse.float_y(), &self.rhs);
                    let start = edge.upper_left();
                    let vector = Vector::new(start, &point);
                    let constraint = traverse.constraint.constrain_direction(
                        vector.pseudoangle().as_ref().unwrap(), dir.invert()
                    );
                    if constraint.is_too_tight(
                        &edge.straight,
                        &edge.pseudoangle_for_upper_left(),
                        &x
                    ) {
                        let snip_point = traverse.preferred_point(&x);
                        let (snippet, what) = self.snip_ray(
                            &neighbor,
                            dir,
                            &snip_point,
                            queued_edges
                        )?;
                        debug_assert!(what == Removed::Position);
                        snippets.push(snippet.unwrap());
                        removed.push(neighbor);
                    } else {
                        let traverse = self.get_traverse_mut(&neighbor).unwrap();
                        traverse.constraint = constraint;
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok((snippets, removed))
    }
    fn constrain_neighbors(&mut self, y: &Coordinate, queued_edges: &mut dyn AbstractQueue) -> Result<(Snippets, Vec<Float>), Error> {
        let (mut snippets, mut removed) = self.constrain_neighbor(y, Direction::Upwards, queued_edges)?;
        let (new_snippets, new_removed) = self.constrain_neighbor(y, Direction::Downwards, queued_edges)?;
        snippets.extend(new_snippets);
        removed.extend(new_removed);
        Ok((snippets, removed))
    }
    fn create_support(&mut self, y: Coordinate) -> Position {
        let x = self.x();
        let position = Position::support(Support::new(Point::unchecked(x, y)));
        position
    }
    fn safe_create_support(&mut self, y: Coordinate, queued_edges: &mut dyn AbstractQueue) -> Result<(Snippets, Vec<Float>), Error> {
        let float = Float::from(y);
        let (snippets, removed) = if self.positions.contains_key(&float) {
            match self.position_type(&float) {
                PositionType::Support => {
                    (Snippets::new(), Vec::new())
                },

                PositionType::Traverse => {
                    let point = Point::unchecked(self.x(), y);
                    let mut snippets = Snippets::new();
                    let (snippet, removed_position) = self.snip_ray(&float, Direction::Downwards, &point, queued_edges)?;
                    debug_assert!(removed_position == Removed::Position);
                    snippets.push(snippet.unwrap());
                    let mut removed = Vec::new();
                    removed.push(float);
                    let position = self.create_support(y);
                    self.positions.insert(float, position);
                    let (new_snippets, new_removed) = self.constrain_neighbors(&y, queued_edges)?;
                    snippets.extend(new_snippets);
                    removed.extend(new_removed);
                    (snippets, removed)
                }
                _ => {
                    panic!("Expected position here")
                }
            }
        } else {
            let position = self.create_support(y);
            self.positions.insert(float, position);
            let (snippets, removed) = self.constrain_neighbors(&y, queued_edges)?;
            (snippets, removed)
        };
        Ok((snippets, removed))

    }
    fn get_traverse(&self, y: &Float) -> Option<&Traverse> {
        if let Some(position) = self.positions.get(y) {
            match &position.either {
                Either::Left(_) => {
                    None
                },
                Either::Right(traverse) => {
                    Some(traverse)
                }
            }
        } else {
            None
        }
    }
    fn get_traverse_mut(&mut self, y: &Float) -> Option<&mut Traverse> {
        if let Some(position) = self.positions.get_mut(y) {
            match &mut position.either {
                Either::Left(_) => {
                    None
                },
                Either::Right(traverse) => {
                    Some(traverse)
                }
            }
        } else {
            None
        }
    }
    fn create_base(position: Coordinate) -> Line {
        let zero = Coordinate::new(0);
        let delta_y = Coordinate::new(1000000);
        let start = Point::unchecked(position, zero.clone());
        let end = Point::unchecked(zero, delta_y);
        let vector = Vector::from(&end);
        Line::new(&start, &vector)
    }
    pub fn iter(&self) -> ScopeIterator {
        ScopeIterator::new(self)
    }
    #[allow(dead_code)]
    pub fn inspect(&self) -> String {
        let mut strings: Vec<String> = Vec::new();
        for (_, position) in &self.positions {
            strings.push(position.inspect(&self.lhs, &self.rhs));
        }
        strings.join("\n")
    }
}
pub struct ScopeIterator<'scope> {
    scope: &'scope Scope,
    lhs: Peekable<std::collections::btree_map::Iter<'scope, ReversedKey, Ray>>,
    rhs: Peekable<Filter<std::collections::btree_map::Iter<'scope, Float, Position>, fn(&(& Float, &Position)) -> bool>>
}
impl <'scope> ScopeIterator<'scope> {
    pub fn new(scope: &'scope Scope) -> ScopeIterator<'scope> {
        let lhs = scope.lhs.iter().peekable();
        let rhs = scope.positions
            .iter()
            .filter(ScopeIterator::traverse as fn(&(& Float, &Position)) -> bool)
            .peekable();
        ScopeIterator { scope, lhs, rhs }
    }
    fn traverse((_, position): &(&Float, &Position)) -> bool {
        position.either.is_right()
    }
    fn draw_from_left(&mut self) -> Option<&'scope Ray> {
        let (_, ray) = self.lhs.next().unwrap();
        Some(ray)
    }
    fn draw_from_right(&mut self) -> Option<&'scope Ray> {
        let (y, _) = self.rhs.next().unwrap();
        Some(Traverse::ray(*y, &self.scope.rhs))
    }
}
impl <'scope> Iterator for ScopeIterator<'scope> {
    type Item = (&'scope Ray);
    fn next(&mut self) -> Option<Self::Item> {
        let l = self.lhs.peek();
        let r = self.rhs.peek();
        match (l, r) {
            (None, None) => None,
            (None, Some(_)) => {
                self.draw_from_right()
            },
            (Some(_), None) => {
                self.draw_from_left()
            },
            (Some((key, _)), Some((y, _))) => {
                if key.y() < **y {
                    self.draw_from_left()
                } else if key.y() > **y {
                    self.draw_from_right()
                } else {
                    panic!("Equal y not expected");
                }
            }
        }
    }
}
impl BentleyOttmann for Scope {
    fn x(&self) -> Coordinate {
        self.line.point.x()
    }
    fn line(&self) -> &Line {
        &self.line
    }
    fn positions(&self) -> &BTreeMap<Float, Position> {
        &self.positions
    }
    fn position_type(&self, float: &Float) -> PositionType {
        if let Some(position) = self.positions.get(float) {
            position.position_type()
        } else {
            PositionType::None
        }
    }
    fn get_support(&self, y: &Coordinate) -> Option<Support> {
        // TODO create lightweight support
        if let Some(position) = self.positions.get(&Float::from(y)) {
            match &position.either {
                Either::Left(_) => {
                    let point = Point::unchecked(self.x(), y.clone());
                    Some(Support::new(point))
                },
                Either::Right(_) => {
                    panic!("Traverse not expected");
                }
            }
        } else {
            None
        }
    }
    fn lhs(&self) -> &Lhs {
        &self.lhs
    }
    fn lhs_mut(&mut self) -> &mut Lhs {
        &mut self.lhs
    }
    fn rhs(&self) -> &Rhs {
        &self.rhs
    }
    fn rhs_mut(&mut self) -> &mut Rhs {
        &mut self.rhs
    }
    fn in_scope_above(&self, y: &Float) -> Option<Float> {
        let mut range = self.positions.range(..y).into_iter().rev();
        range.find(|(_, position)| {
            position.in_scope(&self.rhs)
        }).map(|(float, _)| *float)
    }
    fn in_scope_or_vertical_above(&self, y: &Float) -> Option<Float> {
        let mut range = self.positions.range(..y).into_iter().rev();

        range.find(|(_, position)| {
            if position.in_scope(&self.rhs) {
                true
            } else {
                let vertical_endpoint = position.vertical_endpoint(&self.lhs);
                if let Some(vertical_endpoint) = vertical_endpoint {
                    vertical_endpoint.float_y() > f64::from(*y)
                } else {
                    false
                }
            }
        }).map(|(float, _)| *float)
    }
    fn in_scope_below(&self, y: &Float) -> Option<Float> {
        let mut range = self.positions.range(y..).into_iter();
        range.find(|(float, position)| {
            float != &y && position.in_scope(&self.rhs)
        }).map(|(float, _)| *float)
    }
    fn vertical_endpoint_option(&self, float: &Float) -> Option<Point> {
        let position = self.positions
            .get(float)
            .expect("Expected position to be there");
        position.vertical_endpoint(&self.lhs)
    }
    fn vertical_endpoint(&self, position: &Coordinate) -> Point {
        let support = self.get_support(position).unwrap();
        let vertical = support.vertical_ray(&self.lhs).unwrap();
        let end = vertical.edge().lower_right();
        end.clone()
    }
    fn neighbor_below(&self, y: &Float) -> Option<Float> {
        let mut range = self.positions.range(y..);
        let candidate = range.next();
        if let Some((other_y, _)) = candidate {
            if y == other_y {
                if let Some((float, _)) = range.next() {
                    Some(*float)
                } else {
                    None
                }
            } else {
                Some(*other_y)
            }
        } else {
            None
        }
    }
    fn insert_edge(
        &mut self,
        edge: Edge,
        constraint: Constraint,
        mut stack: Stack,
        queued_edges: &mut dyn AbstractQueue
    ) -> Result<(Stack, InsertionResult), Error> {
        self.check_minimum(edge.lower_right().x());
        let (
            snippets,
            result
        ) = if edge.upper_left().x() < self.x() {
            let end = edge.lower_right();
            if end.x() < self.x() {
                panic!("Edge to the left of the sweep line");
            } else if end.x() == self.x() {
                let (mut snippets, removed) = self.safe_create_support(end.y(), queued_edges)?;
                let support = self.get_support(&end.y()).unwrap();
                let (snippet, td, bd, vd) = support.insert(
                    edge,
                    queued_edges,
                    &mut self.lhs,
                    &mut self.rhs
                )?;
                snippets.push_option(snippet);
                let result = InsertionResult::support_inserted(
                    support.point().y(), td, vd, bd, removed
                );
                (snippets, result)
            } else {
                let (
                    snippets,
                    result
                ) = self.insert_traverse(edge, constraint, queued_edges)?;
                (snippets, result)
            }
        } else {
            let point = edge.upper_left();
            let float = Float::from(point.y());
            let mut snippets = Snippets::new();
            if self.position_type(&float) == PositionType::Traverse {
                let (snippet, _) = self.snip_ray(&float, Direction::Downwards, point, queued_edges)?;
                snippets.push_option(snippet);
            };

            let y = edge.upper_left().y();
            let (new_snippets, removed) = self.safe_create_support(
                y, queued_edges
            )?;
            snippets.extend(new_snippets);
            let support = self.get_support(&y).unwrap();
            let (snippet, td, bd, vd) = support.insert(
                edge,
                queued_edges,
                &mut self.lhs,
                &mut self.rhs
            )?;
            snippets.push_option(snippet);
            let result = InsertionResult::support_inserted(
                support.point().y(), td, bd, vd, removed
            );
            (snippets, result)
        };
        stack = self.enqueue_snippets(snippets, stack, queued_edges);
        Ok((stack, result))
    }
    fn snip_ray(
        &mut self,
        float: &Float,
        direction: Direction,
        point: &Point,
        queued_edges: &mut dyn AbstractQueue) -> Result<(Option<Snippet>, Removed), Error> {
        let original_crossing = if point.x() == self.x() {
            None
        } else {
            Some((self.x(), *float))
        };
        let position = self.positions
            .get_mut(float)
            .expect("Expected position to be there");
        let constraint = position.constraint().clone();
        let ray = match direction {
            Direction::Upwards => position.first_ray_mut(&mut self.rhs).unwrap(),
            Direction::Downwards => position.last_ray_mut(&mut self.rhs).unwrap()
        };
        let snippet = ray.snip_self(point, &constraint, original_crossing, queued_edges)?;
        let angle = ray.angle;

        let (removed, candidate) = if ray.is_empty() {
            let removed = match &mut position.either {
                Either::Left(support) => {
                    let _ = support.remove_from_right(&angle, &mut self.rhs);
                    if support.in_scope(&self.rhs) {
                        Removed::Ray
                    } else {
                        Removed::Position
                    }
                },
                Either::Right(_) => {
                    let _ = self.remove_traverse(float);
                    Removed::Position

                }
            };
            (removed, None)
        } else {
            (Removed::Nothing, Some(ray.edge().lower_right().x()))
        };
        if let Some(candidate) = candidate {
            self.check_minimum(candidate)
        }
        Ok((snippet, removed))
    }
    fn enqueue_snippets(&self, snippets: Snippets, mut stack: Stack, queued_edges: &mut dyn AbstractQueue) -> Stack {
        for snippet in snippets.vec {
            if let Some((left, constraint)) = snippet.left {
                stack = self.edge_to_stack(left, constraint, stack, queued_edges);
            }
            if let Some(right) = snippet.right {
                stack = self.edge_to_stack(right, Constraint::LOOSE, stack, queued_edges);
            }
        }
        stack
    }
    fn wipe_range(
        &mut self,
        obstacles: Vec<Float>,
        point: &Point,
        queued_edges: &mut dyn AbstractQueue
    ) -> Result<(Snippets, Removals), Error> {
        let mut snippets = Snippets::new();
        let mut removals = Removals::new();
        for obstacle in obstacles {
            let neighbor = self.positions()
                .get(&obstacle)
                .expect("Expected obstacle to be there");
            let traverse = neighbor
                .either
                .as_ref()
                .right()
                .expect("Expected obstacle to be traverse");
            let constraint = traverse.constraint;
            let ray = Traverse::ray_mut(traverse.float_y(), self.rhs_mut());
            let snippet = ray.snip_self(&point, &constraint, None, queued_edges)?;
            if ray.is_empty() {
                let _ = self.remove_traverse(&obstacle);
                removals.add(obstacle);
            }
            snippets.push_option(snippet);
        }
        Ok((snippets, removals))
    }
}