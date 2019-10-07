extern crate snipper;
extern crate lazy_static;
use lazy_static::lazy_static;
use std::collections::HashMap;
use snipper::{AbstractPoint, Point, Solution};
use snipper::Path;
use snipper::Polygon;

lazy_static! {
    pub static ref TEST_POLYGONS: HashMap<&'static str, Vec<Point>> = test_polygons();
}
fn test_polygons() -> HashMap<&'static str, Vec<Point>> {
    let mut map = HashMap::new();
    let big = vec![
        Point::new(-15, 0).expect("!"),
        Point::new(-5, 10).expect("!"),
        Point::new(5, 0).expect("!"),
        Point::new(-5, -10).expect("!")];
    map.insert("left", big);

    let right = vec![
        Point::new(-5, 0).expect("!"),
        Point::new(5, 10).expect("!"),
        Point::new(15, 0).expect("!"),
        Point::new(5, -10).expect("!")
    ];
    map.insert("right", right);

    let bow = vec![
        Point::new(-10, -10).expect("!"),
        Point::new(10, 10).expect("!"),
        Point::new(10, -10).expect("!"),
        Point::new(-10, 10).expect("!")
    ];
    map.insert("bow", bow);

    let hair = vec![
        Point::new(-15, 0).expect("!"),
        Point::new(-5, 10).expect("!"),
        Point::new(5, 0).expect("!"),
        Point::new(10, 0).expect("!"),
        Point::new(5, 0).expect("!"),
        Point::new(-5, -10).expect("!")
    ];
    map.insert("hair", hair);

    let vertical = vec![
        Point::new(2, 0).expect("!"),
        Point::new(5, 0).expect("!"),
        Point::new(5, 7).expect("!"),
        Point::new(2, 7).expect("!"),
        Point::new(2, 0).expect("!"),
        Point::new(3, 1).expect("!"),
        Point::new(3, 6).expect("!"),
        Point::new(4, 6).expect("!"),
        Point::new(4, 1).expect("!"),
        Point::new(3, 1).expect("!")
    ];
    map.insert("vertical", vertical);

    let horizontal = vec![
        Point::new(0, 2).expect("!"),
        Point::new(7, 2).expect("!"),
        Point::new(7, 5).expect("!"),
        Point::new(0, 5).expect("!"),
        Point::new(0, 2).expect("!"),
        Point::new(1, 3).expect("!"),
        Point::new(1, 4).expect("!"),
        Point::new(6, 4).expect("!"),
        Point::new(6, 3).expect("!"),
        Point::new(1, 3).expect("!")
    ];
    map.insert("horizontal", horizontal);


    map
}

pub fn get_polygon(name: &str) -> Polygon {
    let path = Path::new(&TEST_POLYGONS[name]);
    unsafe {
        Polygon::trivial(path)
    }
}
pub fn test_result_and_structure(solution: Solution, tuples: &Vec<Vec<(i32, i32)>>, structure: Option<&Vec<Option<usize>>>) {
    let result = solution.polygon().unwrap();
    let expected: Vec<Vec<Point>> = tuples
        .iter()
        .map(|vector| {
            vector.iter().map(|tuple| {
                Point::new(tuple.0, tuple.1).expect("!")
            }).collect()
        }).collect();
    let paths = result.paths();
    if paths.len() != expected.len() {
        panic!("Unexpected number of paths: {}, expected: {}", paths.len(), expected.len())
    }
    for (path_index, path) in paths.iter().enumerate() {
        let expected_path = &expected[path_index];
        let actual_points = path.points();
        if actual_points.len() != expected_path.len() {
            panic!(
                "Unexpected number of points in path #{}: {}, expected: {} -- {}",
                path_index,
                actual_points.len(),
                expected_path.len(),
                path.inspect()
            )
        }
        for (point_index, point) in actual_points.iter().enumerate() {
            let expected_point = &expected_path[point_index];
            if expected_point != point {
                panic!(
                    "Unexpected point at index #{} in path #{}: {}, expected: {} -- {}",
                    point_index,
                    path_index,
                    point.inspect(),
                    expected_point.inspect(),
                    path.inspect()
                )
            }
        }
    }
    if let Some(structure) = structure {
        debug_assert!(
            tuples.len() == structure.len(),
            "Expected paths and structure should be of the same length, got: {} vs. {}",
            tuples.len(),
            structure.len()
        );
        let records = result.structure();
        for (path_index, parent) in structure.iter().enumerate() {
            let record = &records[path_index];
            assert_eq!(parent, &record.parent(), "Parent for {} other than expected: {:?}, expected: {:?}",
                path_index, record.parent(), parent
            )
        }
    }
}
pub fn test_result(solution: Solution, tuples: &Vec<Vec<(i32, i32)>>) {
    test_result_and_structure(solution, tuples, None);
}