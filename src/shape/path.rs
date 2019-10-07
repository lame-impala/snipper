use crate::primitives::{AbstractPoint, Point, Position, Mode, Bounds, Straight, Sector, Line, Vector};
use crate::helpers::approx_eq;
use crate::shape::{Shape};
use crate::units::Coordinate;

#[derive(Debug, PartialEq, Clone)]
pub enum PathDirection {
    Unknown,
    Clockwise,
    Counterclockwise
}

#[derive(Clone)]
struct PathCount {
    bounds: Option<Bounds>,
    running_area: f64,
    running_centroid: Option<(f64, f64)>,
    perimeter: f64,
    perimeter_centroid: Option<(f64, f64)>

}
impl PathCount {
    fn new() -> PathCount {
        PathCount{
            bounds: None,
            running_area: 0.0,
            running_centroid: None,
            perimeter: 0.0,
            perimeter_centroid: None
        }
    }
    fn update(&self, next: &Point, last: Option<&Point>) -> PathCount {
        let bounds = Some(self.update_bounds(next));
        let new = match last {
            None => {
                let running_area = 0.0;
                let running_centroid = Some((next.float_x(), next.float_y()));
                let perimeter = 0.0;
                let perimeter_centroid = None;
                PathCount{bounds, running_area, running_centroid, perimeter, perimeter_centroid}
            },
            Some(last_point) => {
                let (
                    running_area,
                    running_centroid
                ) = self.update_area_and_centroid(next, last_point);
                let (
                    perimeter,
                    perimeter_centroid
                ) = self.update_perimeter(next, last_point);
                PathCount{bounds, running_area, running_centroid, perimeter, perimeter_centroid}
            }
        };
        new
    }
    fn update_perimeter(&self, next: &Point, last: &Point) -> (f64, Option<(f64, f64)>) {
        let segment = Straight::new(next, last);
        let midpoint = segment.midpoint();
        let length = segment.length();
        let perimeter = self.perimeter + length;
        let (old_x, old_y) = match self.perimeter_centroid {
            None => (0f64, 0f64),
            Some((x, y)) => (x, y)
        };
        let new_x = old_x + midpoint.float_x() * length;
        let new_y = old_y + midpoint.float_y() * length;
        (perimeter, Some((new_x, new_y)))
    }
    fn update_area_and_centroid(&self, next: &Point, last: &Point) -> (f64, Option<(f64, f64)>) {
        let increment = (last.float_x() * next.float_y()) - (last.float_y() * next.float_x());
        let running_area = self.running_area + increment;
        let x_shift = (last.float_x() + next.float_x()) * increment;
        let y_shift = (last.float_y() + next.float_y()) * increment;
        let (x, y) = self.running_centroid.unwrap();
        let running_centroid: (f64, f64) = (x + x_shift, y + y_shift);
        (running_area, Some(running_centroid))
    }
    fn update_bounds(&self, next: &Point) -> Bounds {
        match &self.bounds {
            None => {
                Bounds::from_extremes(next.y(), next.x(), next.y(), next.x())
            },
            Some(old) => {
                let top = old.top().min(next.y());
                let left = old.left().min(next.x());
                let bottom = old.bottom().max(next.y());
                let right = old.right().max(next.x());
                Bounds::from_extremes(top, left, bottom, right)
            }
        }
    }
    pub fn area(&self) -> f64 {
        (self.running_area / 2.0).abs()
    }
    pub fn centroid(&self) -> Option<Point> {
        match self.running_centroid {
            None => None,
            Some((rcx, rcy)) => {
                let ra = self.running_area;
                let result = if approx_eq(0f64, ra, 0.001) {
                    if approx_eq(0f64, self.perimeter, 0.001) {
                        Point::new(rcx.round() as i32, rcy.round() as i32)
                    } else {
                        let pc = self.perimeter_centroid.unwrap();
                        let pcx = (pc.0 / self.perimeter).round() as i32;
                        let pcy = (pc.1 / self.perimeter).round() as i32;
                        Point::new(pcx, pcy)
                    }
                } else {
                    let ratio = 1.0 / (3.0 * ra);
                    let x = (rcx * ratio).round() as i32;
                    let y = (rcy * ratio).round() as i32;
                    Point::new(x, y)
                };
                match result {
                    Ok(point) => Some(point),
                    _ => None
                }
            }
        }
    }
}
pub struct PathBuilder {
    points: Vec<Point>,
    count: PathCount
}
impl PathBuilder {
    pub fn new() -> PathBuilder {
        PathBuilder{
            points: Vec::new(),
            count: PathCount::new()
        }
    }
    pub fn add(&mut self, point: &Point) {
        self.count = self.count.update(point, self.points.last());
        self.points.push(point.clone());
    }
    pub fn build(mut self) -> Path {
        if self.points.len() > 0 {
            self.count = self.count.update(self.points.first().unwrap(), self.points.last());
        }
        Path { points: self.points, count: self.count }
    }
}
pub struct Path {
    points: Vec<Point>,
    count: PathCount
}
impl Path {
    pub fn is_empty(&self) -> bool { self.points.is_empty() }
    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }
    pub fn new(source: &Vec<Point>) -> Path {
        let mut builder = PathBuilder::new();
        for point in source {
            builder.add(point);
        }
        builder.build()
    }
    pub fn direction(&self) -> PathDirection {
        let ra = self.count.running_area;
        if ra == 0f64 {
            PathDirection::Unknown
        } else if ra < 0f64 {
            PathDirection::Counterclockwise
        } else {
            PathDirection::Clockwise
        }
    }
    pub fn is_null(&self) -> bool {
        self.points.len() == 0
    }
    pub fn area(&self) -> f64 { self.count.area() }
    pub fn centroid(&self) -> Option<Point> { self.count.centroid() }
    pub fn bounds(&self) -> Option<&Bounds> {
        match &self.count.bounds {
            None => None,
            Some(bounds) => {
                Some(&bounds)
            }
        }
    }
    pub fn point_at(&self, idx: &usize) -> Option<&Point> {
        if self.is_null() { return None }
        let modulo = idx % self.points.len();
        self.points.get(modulo)
    }
    pub fn segment_at(&self, idx: usize) -> Option<Straight> {
        if self.is_null() { return None }
        let start = self.point_at(&idx);
        let end = self.point_at(&(idx + 1));
        if start == None || end == None { return None }
        let straight = Straight::new(&start.unwrap(), &end.unwrap());
        Some(straight)
    }
    pub fn reverse(mut self) -> Path {
        let points = self.points.drain(..).rev().collect();
        Path::new(&points)
    }
    pub fn inspect(&self) -> String {
        let strings: Vec<String> = self.points()
            .iter()
            .map(|point| point.inspect())
            .collect();
        strings.join(", ")
    }
}
impl Clone for Path {
    fn clone(&self) -> Path {
        Path{
            points: self.points.iter().cloned().collect(),
            count: self.count.clone()
        }
    }

}
impl Shape for Path {
    fn position(&self, point: &Point) -> Position {
        fn is_to_the_right(p: &Point, s: &Straight) -> bool {
            s.start.x() <= p.x() || s.end.x() <= p.x()
        }
        fn is_within_points_on_y(p: &Point, s: &Straight) -> bool {
            if s.start.y() < p.y() && s.end.y() >= p.y() {return true;}
            if s.end.y() < p.y() && s.start.y() >= p.y() {return true;}
            return false;
        }
        fn intersects_with(l1: &Line, s: &Straight) -> bool {
            if l1.contains(&s.start) {return true};
            if l1.contains(&s.end) {return true};
            let l2 = s.to_line();
            let i1 = Line::intersection(l1, &l2);
            match i1 {
                None => false,
                Some(intersection) => s.bounds.contains(&intersection, &Mode::Closed)
            }
        }
        let mut position = Position::Out;
        let mut i: usize = 0;

        let base = Vector::from(&Point::unchecked(
            Coordinate::new(1000),
            Coordinate::new(0)
        ));

        let line = Line::new(&point, &base);
        while position != Position::Edge && i < self.points.len() {
            let segment = self.segment_at(i).unwrap();
            if segment.contains(point, &Mode::Closed) {
                position = Position::Edge;
            } else if
                is_to_the_right(&point, &segment) &&
                    is_within_points_on_y(&point, &segment) &&
                    intersects_with(&line, &segment)
                {
                    position = Position::invert(&position);
                }
            i += 1;
        }
        position
    }
    fn bounds(&self) -> Option<&Bounds> {
        self.count.bounds.as_ref()
    }
    fn paths(&self) -> Vec<&Path> {
        vec![&self]
    }
}

#[test]
fn centroid_test() {
    let points1  = vec![
        Point::new(1, 1).expect("!"),
        Point::new(3, 1).expect("!"),
        Point::new(3, 3).expect("!"),
        Point::new(1, 3).expect("!")
    ];
    let path1 = Path::new(&points1);
    assert_eq!(path1.centroid().unwrap(), Point::new(2, 2).expect("!"));
    let points2  = vec![
        Point::new(1000, 1000).expect("!"),
        Point::new(2000, 1000).expect("!"),
        Point::new(3000, 1000).expect("!"),
        Point::new(2000, 1000).expect("!")
    ];
    let path2 = Path::new(&points2);
    assert_eq!(path2.centroid().unwrap(), Point::new(2000, 1000).expect("!"));
    let points3  = vec![
        Point::new(0, 0).expect("!"),
        Point::new(10, 0).expect("!"),
        Point::new(10, 10).expect("!"),
        Point::new(10, 0).expect("!")
    ];
    let path3 = Path::new(&points3);
    assert_eq!(path3.centroid().unwrap(), Point::new(8, 3).expect("!"));
    let points4  = vec![
        Point::new(100, 100).expect("!"),
        Point::new(300, 100).expect("!"),
        Point::new(300, 500).expect("!"),
        Point::new(300, 100).expect("!")
    ];
    let path4 = Path::new(&points4);
    assert_eq!(path4.centroid().unwrap(), Point::new(267, 233).expect("!"));
    let points5  = vec![
        Point::new(100, 100).expect("!"),
        Point::new(400, 100).expect("!"),
        Point::new(400, 500).expect("!"),
        Point::new(100, 100).expect("!"),
        Point::new(400, 500).expect("!"),
        Point::new(400, 100).expect("!")
    ];
    let path5 = Path::new(&points5);
    assert_eq!(path5.centroid().unwrap(), Point::new(300, 250).expect("!"));
}
#[test]
fn direction_test() {
    let points1 = vec![
        Point::new(0, 0).expect("!"),
        Point::new(2, 0).expect("!"),
        Point::new(1, 2).expect("!")
    ];
    let points2 = vec![
        Point::new(0, 0).expect("!"),
        Point::new(1, 2).expect("!"),
        Point::new(2, 0).expect("!")
    ];
    let points3 = vec![
        Point::new(0, 0).expect("!"),
        Point::new(1, 2).expect("!")
    ];
    let cw = Path::new(&points1);
    let ccw = Path::new(&points2);
    let ukwn = Path::new(&points3);
    assert_eq!(cw.direction(), PathDirection::Clockwise);
    assert_eq!(ccw.direction(), PathDirection::Counterclockwise);
    assert_eq!(ukwn.direction(), PathDirection::Unknown);
    assert_eq!(cw.reverse().direction(), PathDirection::Counterclockwise);
    assert_eq!(ccw.reverse().direction(), PathDirection::Clockwise);
    assert_eq!(ukwn.reverse().direction(), PathDirection::Unknown);
}
#[test]
fn path_test() {
    let points = vec![
        Point::new(0, 0).expect("!"),
        Point::new(30, 0).expect("!"),
        Point::new(30, 10).expect("!"),
        Point::new(40, 10).expect("!"),
        Point::new(40, 20).expect("!"),
        Point::new(10, 20).expect("!"),
        Point::new(10, 10).expect("!"),
        Point::new(0, 10).expect("!")
    ];
    let path = Path::new(&points);
    assert_eq!(path.area(), 600.0);
    assert_eq!(path.centroid().unwrap(), Point::new(20, 10).expect("!"));
    assert_eq!(path.bounds().unwrap(), &Bounds::new(0, 0, 20, 40));

    let pc = Point::new(20, 10).expect("!");
    assert!(path.contains(&pc, &Mode::Closed));
    assert!(path.contains(&pc, &Mode::Open));

    let tp0 = Point::new(20, -1).expect("!");
    assert!(!path.contains(&tp0, &Mode::Closed));
    assert!(!path.contains(&tp0, &Mode::Open));

    let tp1 = Point::new(20, 0).expect("!");
    assert!(path.contains(&tp1, &Mode::Closed));
    assert!(!path.contains(&tp1, &Mode::Open));

    let tp2 = Point::new(20, 1).expect("!");
    assert!(path.contains(&tp2, &Mode::Closed));
    assert!(path.contains(&tp2, &Mode::Open));

    let lp0 = Point::new(-1, 10).expect("!");
    assert!(!path.contains(&lp0, &Mode::Closed));
    assert!(!path.contains(&lp0, &Mode::Open));

    let lp1 = Point::new(5, 10).expect("!");
    assert!(path.contains(&lp1, &Mode::Closed));
    assert!(!path.contains(&lp1, &Mode::Open));

    let lp2 = Point::new(11, 10).expect("!");
    assert!(path.contains(&lp2, &Mode::Closed));
    assert!(path.contains(&lp2, &Mode::Open));

    let bp0 = Point::new(20, 21).expect("!");
    assert!(!path.contains(&bp0, &Mode::Closed));
    assert!(!path.contains(&bp0, &Mode::Open));

    let bp1 = Point::new(20, 20).expect("!");
    assert!(path.contains(&bp1, &Mode::Closed));
    assert!(!path.contains(&bp1, &Mode::Open));

    let bp2 = Point::new(20, 19).expect("!");
    assert!(path.contains(&bp2, &Mode::Closed));
    assert!(path.contains(&bp2, &Mode::Open));

    let rp0 = Point::new(41, 10).expect("!");
    assert!(!path.contains(&rp0, &Mode::Closed));
    assert!(!path.contains(&rp0, &Mode::Open));

    let rp1 = Point::new(35, 10).expect("!");
    assert!(path.contains(&rp1, &Mode::Closed));
    assert!(!path.contains(&rp1, &Mode::Open));

    let rp2 = Point::new(29, 10).expect("!");
    assert!(path.contains(&rp2, &Mode::Closed));
    assert!(path.contains(&rp2, &Mode::Open));

}
