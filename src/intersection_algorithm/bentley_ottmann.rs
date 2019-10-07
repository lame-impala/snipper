use crate::intersection_algorithm::constraint::Constraint;
use crate::intersection_algorithm::snippet::{Snippets, Snippet};
use crate::units::{Float};
use crate::edge::queue::AbstractQueue;
use crate::{Coordinate, Error, Point, AbstractPoint};
use crate::intersection_algorithm::stack::Stack;
use crate::intersection_algorithm::position::{PositionType, Position};
use crate::intersection_algorithm::traverse::Traverse;
use crate::primitives::{Line, Straight, Intersection};
use crate::intersection_algorithm::scope::{Direction, Removed, Lhs, Rhs};
use itertools::Either;
use std::collections::btree_map::BTreeMap;
use crate::edge::Edge;
use crate::intersection_algorithm::support::Support;
use crate::intersection_algorithm::dirty_records::{DirtyRecords, Dirty, InsertionResult, Removals};

pub trait BentleyOttmann {
    fn initialize_dirty_sets(&self) -> DirtyRecords {
        let mut records = DirtyRecords::new();
        for (float, position) in self.positions() {
            if position.in_scope(&self.rhs()) {
                records.set_dirty(Dirty::Top(*float));
                records.set_dirty(Dirty::Bottom(*float));
            }
            if position.has_vertical_edge(&self.lhs()) {
                let support = position.either.as_ref().left().unwrap();
                records.set_dirty(Dirty::Vertical(support.point.y()));
            }
        }
        records
    }
    fn on_position_removed(
        &self, float: &Float,
        above: Option<Float>,
        below: Option<Float>,
        mut dirty_records: DirtyRecords
    ) -> DirtyRecords {
        if let Some(above) = above {
            dirty_records.set_dirty(Dirty::Bottom(above));
        } else  if let Some(above) = self.in_scope_above(float) {
            dirty_records.set_dirty(Dirty::Bottom(above));
        }
        if let Some(below) = below {
            dirty_records.set_dirty(Dirty::Top(below));
        } else if let Some(below) = self.in_scope_below(float) {
            dirty_records.set_dirty(Dirty::Top(below));
        }
        dirty_records.remove(float);
        dirty_records
    }
    fn undirty_removed(
        &self,
        removals: Option<&Removals>,
        mut dirty_records: DirtyRecords
    ) -> DirtyRecords {
        if let Some(removals) = removals {
            if !removals.is_empty() {
                for float in removals.iter() {
                    dirty_records.remove(float);
                }
                if let Some(above) = self.in_scope_above(removals.min().unwrap()) {
                    dirty_records.set_dirty(Dirty::Bottom(above));
                }
                if let Some(below) = self.in_scope_above(removals.max().unwrap()) {
                    dirty_records.set_dirty(Dirty::Top(below));
                }
            }
        }
        dirty_records
    }

    fn find_intersections(&mut self, mut stack: Stack, queued_edges: &mut dyn AbstractQueue) -> Result<(), Error> {
        let mut dirty_records = self.initialize_dirty_sets();
        while !stack.is_empty() || dirty_records.has_next() {
            if stack.has_next() {
                let stacked = stack.pop().unwrap();
                let tuple = self.insert_edge(
                    stacked.edge, stacked.constraint, stack, queued_edges
                )?;
                // top_dirty, bottom_dirty, vertical_dirty, final_position, was_removed?, removed
                stack = tuple.0;
                let result = tuple.1;
                if result.top_dirty() {
                    dirty_records.set_dirty(Dirty::Top(result.float_y()));
                }
                if result.bottom_dirty() {
                    dirty_records.set_dirty(Dirty::Bottom(result.float_y()));
                }
                if result.vertical_dirty() {
                    dirty_records.set_dirty(Dirty::Vertical(result.coordinate_y().unwrap()));
                }
                dirty_records = self.undirty_removed(result.removals(), dirty_records);

            } else {
                let next = dirty_records.next();
                let tuple = match next {
                    Dirty::Vertical(coo) => {
                        dirty_records.undirty(Dirty::Vertical(coo));
                        self.handle_vertical(
                            &coo,
                            stack,
                            queued_edges,
                            dirty_records
                        )?
                    },
                    Dirty::Top(float) => {
                        self.handle_top_dirty(
                            float,
                            stack,
                            dirty_records,
                            queued_edges,
                        )?
                    },
                    Dirty::Bottom(float) => {
                        self.handle_bottom_dirty(
                            float,
                            stack,
                            dirty_records,
                            queued_edges
                        )?
                    },
                    Dirty::None => {
                        panic!("Some branch expected to execute");
                    }
                };
                stack = tuple.0;
                dirty_records = tuple.1
            }
        }
        Ok(())
    }

    fn handle_bottom_dirty(
        &mut self,
        float: Float,
        stack: Stack,
        mut dirty_records: DirtyRecords,
        queued_edges: &mut dyn AbstractQueue,
    ) -> Result<(Stack, DirtyRecords), Error> {
        let _ = dirty_records.undirty(Dirty::Bottom(float));
        let below = self.in_scope_below(&float);
        if let Some(below) = below {
            let _ = dirty_records.undirty(Dirty::Top(below));
            self.intersect(
                &float,
                &below,
                stack,
                dirty_records,
                queued_edges,
            )
        } else {
            Ok((stack, dirty_records))
        }
    }
    fn handle_top_dirty(
        &mut self,
        float: Float,
        stack: Stack,
        mut dirty_records: DirtyRecords,
        queued_edges: &mut dyn AbstractQueue,
    ) -> Result<(Stack, DirtyRecords), Error> {
        let above = self.in_scope_or_vertical_above(&float);
        if let Some(above) = above {
            // The check for verticals but it is actually necessary
            // given how positions are marked as dirty. Maybe this
            // could be done in some more consistent way
            let vertical_endpoint = self.vertical_endpoint_option(&above);
            if vertical_endpoint.is_some() && vertical_endpoint.as_ref().unwrap().float_y() > f64::from(float) {
                let coo = Coordinate::from_float(f64::from(above)).unwrap();
                dirty_records.undirty(Dirty::Vertical(coo));
                self.handle_vertical(
                    &coo,
                    stack,
                    queued_edges,
                    dirty_records
                )
            } else {
                let _ = dirty_records.undirty(Dirty::Top(float));
                let _ = dirty_records.undirty(Dirty::Bottom(above));
                self.intersect(
                    &above,
                    &float,
                    stack,
                    dirty_records,
                    queued_edges,
                )
            }
        } else {
            dirty_records.undirty(Dirty::Top(float));
            Ok((stack, dirty_records))
        }
    }
    fn handle_vertical(
        &mut self,
        support_position: &Coordinate,
        mut stack: Stack,
        queued_edges: &mut dyn AbstractQueue,
        mut dirty_records: DirtyRecords
    ) -> Result<(Stack, DirtyRecords), Error> {
        if let Some(neighbor_position) = self.neighbor_below(&Float::from(*support_position)) {
            let end = self.vertical_endpoint(support_position);
            let snippets = if neighbor_position > Float::new(end.float_y()).unwrap() {
                Snippets::new()
            } else {
                let neighbor_type = self.position_type(&neighbor_position);
                let (mut snippets, point) = match neighbor_type {
                    PositionType::Support => {
                        let point = self.positions()
                            .get(&neighbor_position)
                            .expect("Expected neighbor to be there")
                            .either
                            .as_ref()
                            .left()
                            .expect("Expected neighbor to be support")
                            .point.clone();
                        (Snippets::new(), point)
                    },
                    PositionType::Traverse => {
                        let neighbor = self.positions()
                            .get(&neighbor_position)
                            .expect("Expected neighbor to be there");
                        let traverse = neighbor
                            .either
                            .as_ref()
                            .right()
                            .expect("Expected neighbor to be traverse");
                        let ray = Traverse::ray(traverse.float_y(), &self.rhs());
                        let line = &self.line();
                        let point = Line::intersection(line, &ray
                            .slope())
                            .expect("Edge not expected to be collinear");
                        let mut obstacles = self.downward_obstacles(
                            &Float::from(*support_position),
                            &traverse.float_y()
                        );
                        obstacles.push(traverse.float_y());
                        let tuple = self.wipe_obstacles(
                            obstacles,
                            &point,
                            queued_edges,
                            dirty_records
                        )?;
                        let snippets = tuple.0;
                        dirty_records = tuple.1;
                        (snippets, point)
                    },
                    _ => panic!("Expected position at {}", neighbor_position)
                };
                let x = self.x();
                let support = self.get_support(support_position).unwrap();
                let mut ray = support.take_vertical_ray(self.lhs_mut()).unwrap();
                let original_crossing = Some((x, Float::from(support_position)));
                let snippet = ray.snip_self(&point, &Constraint::VERTICAL, original_crossing, queued_edges)?;
                snippets.push_option(snippet);
                if !ray.is_empty() {
                    support.insert_vertical_ray(ray, self.lhs_mut());
                }
                snippets
            };
            stack = self.enqueue_snippets(snippets, stack, queued_edges);
        }
        Ok((stack, dirty_records))
    }

    fn is_allowed(&self, y: &Float, point: &Point) -> bool {
        if let Either::Right(traverse) = self.positions()
            .get(y)
            .unwrap()
            .either
            .as_ref() {
            let start = Traverse::edge(traverse.float_y(), self.rhs())
                .straight
                .upper_left();
            let new = Straight::new(&start, point);
            let new_angle = new.vector().pseudoangle().unwrap();
            let constraint = &traverse.constraint;
            constraint.allows(&new, &new_angle, &self.x())
        } else {
            true
        }
    }
    fn disallowed_or_has_obstacles(
        &self,
        current_y: &Float,
        other_y: &Float,
        point: &Point,
        direction: Direction
    ) -> bool {
        debug_assert!(
            current_y < other_y && direction == Direction::Downwards ||
            current_y > other_y && direction == Direction::Upwards);
        if self.is_allowed(current_y, point) {
            self.has_obstacles(
                current_y,
                other_y,
                &self.wiping_range_endpoint(
                    current_y,
                    point,
                    direction
                )
            )
        } else {
            true
        }
    }
    fn intersect(
        &mut self,
        upper: &Float,
        lower: &Float,
        mut stack: Stack,
        dirty_records: DirtyRecords,
        queued_edges: &mut dyn AbstractQueue,
    ) -> Result<(Stack, DirtyRecords), Error> {
        let intersection = self.intersect_rays(upper, lower);
        if let Some(point) = intersection {
            let (snippets, dirty_records) = if self.disallowed_or_has_obstacles(
                upper,
                lower,
                &point,
                Direction::Downwards
            ) {
                self.snip_disallowed_or_having_obstacles(
                    upper,
                    lower,
                    Direction::Downwards,
                    dirty_records,
                    queued_edges,
                )?
            } else if self.disallowed_or_has_obstacles(
                lower,
                upper,
                &point,
                Direction::Upwards
            ) {
                self.snip_disallowed_or_having_obstacles(
                    lower,
                    upper,
                    Direction::Upwards,
                    dirty_records,
                    queued_edges,
                )?
            } else {
                self.perform_allowed_intersection(
                    upper,
                    lower,
                    &point,
                    queued_edges,
                    dirty_records)?
            };

            stack = self.enqueue_snippets(snippets, stack, queued_edges);
            Ok((stack, dirty_records))
        } else {
            Ok((stack, dirty_records))
        }
    }

    fn perform_allowed_intersection(
        &mut self,
        upper: &Float,
        lower: &Float,
        point: &Point,
        queued_edges: &mut dyn AbstractQueue,
        dirty_records: DirtyRecords,
    ) -> Result<(Snippets, DirtyRecords), Error> {
        let snippets = Snippets::new();
        let (snippets, dirty_records) = self.perform_allowed_intersection_on_ray(
            upper,
            point,
            Direction::Downwards,
            dirty_records,
            snippets,
            queued_edges,
        )?;
        self.perform_allowed_intersection_on_ray(
            lower,
            point,
            Direction::Upwards,
            dirty_records,
            snippets,
            queued_edges,
        )
    }

    fn perform_allowed_intersection_on_ray(
        &mut self,
        current_y: &Float,
        point: &Point,
        direction: Direction,
        mut dirty_records: DirtyRecords,
        mut snippets: Snippets,
        queued_edges: &mut dyn AbstractQueue,
    ) -> Result<(Snippets, DirtyRecords), Error>{
        let (snippet, removed) = self.snip_ray(
            current_y, direction, point, queued_edges
        )?;
        dirty_records = match removed {
            Removed::Position => {
                self.on_position_removed(current_y, None, None, dirty_records)
            },
            Removed::Ray => {
                let dirty = match direction {
                    Direction::Downwards => Dirty::Bottom(*current_y),
                    Direction::Upwards => Dirty::Top(*current_y),
                };
                dirty_records.set_dirty(dirty);
                dirty_records
            },
            _ => dirty_records
        };
        snippets.push_option(snippet);
        Ok((snippets, dirty_records))

    }

    fn snip_disallowed_or_having_obstacles(
        &mut self,
        current_y: &Float,
        other_y: &Float,
        direction: Direction,
        mut dirty_records: DirtyRecords,
        queued_edges: &mut dyn AbstractQueue,
    ) -> Result<(Snippets, DirtyRecords), Error> {
        debug_assert!(
            current_y < other_y && direction == Direction::Downwards ||
                current_y > other_y && direction == Direction::Upwards);

        let mut snippets = Snippets::new();
        let traverse = self.positions().get(current_y)
            .unwrap()
            .either
            .as_ref()
            .right()
            .expect("Traverse expected");
        let preferred = traverse.preferred_point(&self.x());
        let (current_snippet, current_removed) = self.snip_ray(
            current_y, direction, &preferred, queued_edges
        )?;
        debug_assert!(current_removed == Removed::Position);
        let (above, below) = match direction {
            Direction::Downwards => (None, Some(*other_y)),
            Direction::Upwards => (Some(*other_y), None)
        };
        dirty_records = self.on_position_removed(
            current_y,
            above,
            below,
            dirty_records
        );
        snippets.push(current_snippet.unwrap());
        let obstacles = self.obstacles(current_y, &Float::from(preferred.y()));
        if let Some(obstacles) = obstacles {
            let tuple = self.wipe_obstacles(
                obstacles,
                &preferred,
                queued_edges,
                dirty_records
            )?;
            snippets.extend(tuple.0);
            dirty_records = tuple.1
        }
        Ok((snippets, dirty_records))
    }
    fn wiping_range_endpoint(&self, current: &Float, point: &Point, dir: Direction) -> Float {
        let position = self.positions().get(current).unwrap();
        let start = match dir {
            Direction::Downwards => {
                position.last_ray(&self.rhs()).unwrap().edge().upper_left()
            },
            Direction::Upwards => {
                position.first_ray(&self.rhs()).unwrap().edge().upper_left()
            }
        };
        let straight = Straight::new(start, point);
        if straight.is_null() {
            *current
        } else {
            if straight.start.x() == straight.end.x() {
                // verticals are handled separately
                *current
            } else {
                straight.cross_with_vertical(self.x()).unwrap()
            }
        }
    }
    fn has_obstacles(&self, current: &Float, other: &Float, new: &Float) -> bool {
        let (from, to) = if current > new {
            (new, current)
        } else {
            (current, new)
        };
        let result = self.positions()
            .range(from..)
            .into_iter()
            .take_while(|(y, _)| y <= &to)
            .filter_map(|(y, _)| {
                if y == current || y == other {
                    None
                } else if self.position_type(y) == PositionType::Support {
                    None
                } else {
                    Some(y)
                }
            }).nth(0);
        result.is_some()
    }
    fn downward_obstacles(&self, from: &Float, to: &Float) -> Vec<Float> {
        self.positions()
            .range(from..)
            .into_iter()
            .skip_while(|(y, _)| {
                y <= &from
            }).take_while(|(y, _)| {
            if self.position_type(y) == PositionType::Support {
                false
            } else if y < &to {
                true
            } else {
                false
            }
        }).map(|(y, _)| *y).collect()
    }
    fn upward_obstacles(&self, from: &Float, to: &Float) -> Vec<Float> {
        self.positions()
            .range(..from)
            .into_iter()
            .rev()
            .skip_while(|(y, _)| {
                y >= &from
            }).take_while(|(y, _)| {
            if self.position_type(y) == PositionType::Support {
                false
            } else if y >= &to {
                true
            } else {
                false
            }
        }).map(|(y, _)| *y).collect()
    }
    fn obstacles(&self, start: &Float, end: &Float) -> Option<Vec<Float>> {
        if start == end {
            None
        } else {
            let obstacles = if start < end {
                self.downward_obstacles(&start, &end)
            } else {
                self.upward_obstacles(&start, &end)
            };
            Some(obstacles)
        }
    }

    fn intersect_rays(&self, upper: &Float, lower: &Float) -> Option<Point> {
        let upper_position = self.positions()
            .get(upper)
            .expect("Expected upper position to be there");
        let lower_position = self.positions()
            .get(lower)
            .expect("Expected lower position to be there");
        let top_ray_option = lower_position.first_ray(&self.rhs());
        let bottom_ray_option = upper_position.last_ray(&self.rhs());
        if top_ray_option.is_some() && bottom_ray_option.is_some() {
            let top_ray = top_ray_option.unwrap();
            let bottom_ray = bottom_ray_option.unwrap();
            let intersection = Intersection::proper(
                &top_ray.edge().straight, &bottom_ray.edge().straight
            );
           intersection
        } else {
            None
        }
    }
    fn wipe_obstacles(
        &mut self,
        obstacles: Vec<Float>,
        point: &Point,
        queued_edges: &mut dyn AbstractQueue,
        mut dirty_records: DirtyRecords
    ) -> Result<(Snippets, DirtyRecords), Error> {
        let (snippets, removals) = self.wipe_range(
            obstacles,
            point,
            queued_edges
        )?;
        dirty_records = self.undirty_removed(Some(&removals), dirty_records);
        Ok((snippets, dirty_records))
    }

    fn x(&self) -> Coordinate;
    fn line(&self) -> &Line;
    fn positions(&self) -> &BTreeMap<Float, Position>;
    fn position_type(&self, float: &Float) -> PositionType;
    fn get_support(&self, y: &Coordinate) -> Option<Support>;
    fn lhs(&self) -> &Lhs;
    fn lhs_mut(&mut self) -> &mut Lhs;
    fn rhs(&self) -> &Rhs;
    fn rhs_mut(&mut self) -> &mut Rhs;
    fn in_scope_above(&self, y: &Float) -> Option<Float>;
    fn in_scope_or_vertical_above(&self, y: &Float) -> Option<Float>;
    fn in_scope_below(&self, y: &Float) -> Option<Float>;
    fn vertical_endpoint_option(&self, float: &Float) -> Option<Point>;
    fn vertical_endpoint(&self, position: &Coordinate) -> Point;
    fn neighbor_below(&self, y: &Float) -> Option<Float>;

    fn insert_edge(
        &mut self,
        edge: Edge,
        constraint: Constraint,
        stack: Stack,
        queued_edges: &mut dyn AbstractQueue
    ) -> Result<(Stack, InsertionResult), Error>;

    fn snip_ray(
        &mut self,
        float: &Float,
        direction: Direction,
        point: &Point,
        queued_edges: &mut dyn AbstractQueue
    ) -> Result<(Option<Snippet>, Removed), Error>;

    fn enqueue_snippets(&self, snippets: Snippets, stack: Stack, queued_edges: &mut dyn AbstractQueue) -> Stack;
    fn wipe_range(
        &mut self,
        obstacles: Vec<Float>,
        point: &Point,
        queued_edges: &mut dyn AbstractQueue
    ) -> Result<(Snippets, Removals), Error>;
}