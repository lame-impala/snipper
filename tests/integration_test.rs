extern crate snipper;
use snipper::{Snipper, Path, Polygon};
use crate::test_helper::{test_result, test_result_and_structure};

mod test_helper;


#[test]
fn union_test() {
    let big = test_helper::get_polygon("left");
    let cross = test_helper::get_polygon("right");
    let result = Snipper::union(big, cross).unwrap();
    let expected = vec![vec![
        (-15, 0), (-5, -10), (0, -5), (5, -10), (15, 0), (5, 10), (0, 5), (-5, 10)
    ]];
    test_result(result, &expected);
}
#[test]
fn intersection_test() {
    let big = test_helper::get_polygon("left");
    let cross = test_helper::get_polygon("right");
    let result = Snipper::intersection(big, cross).unwrap();
    let expected = vec![vec![
        (-5, 0), (0, -5), (5, 0), (0, 5)
    ]];
    test_result(result, &expected);
}
#[test]
fn xor_test() {
    let big = test_helper::get_polygon("left");
    let cross = test_helper::get_polygon("right");
    let result = Snipper::xor(big, cross).unwrap();
    let expected = vec![
        vec![
            (-15, 0), (-5, -10), (0, -5), (5, -10), (15, 0), (5, 10), (0, 5), (-5, 10)
        ],
        vec![
            (0, 5), (5, 0), (0, -5), (-5, 0)
        ],
    ];
    let structure = vec![None, Some(0)];
    test_result_and_structure(result, &expected, Some(&structure));
}
#[test]
fn difference_test() {
    let big = test_helper::get_polygon("left");
    let cross = test_helper::get_polygon("right");
    let result = Snipper::difference(cross, big).unwrap();
    let expected = vec![vec![
        (0, -5), (5, -10), (15, 0), (5, 10), (0, 5), (5, 0)
    ]];
    test_result(result, &expected);
}
#[test]
fn normalization_test() {
    let bow = &test_helper::TEST_POLYGONS["bow"];
    let path = Path::new(bow);
    let result = Snipper::normalize(vec![path]).unwrap();
    let expected = vec![vec![
        (-10, -10), (0, 0), (10, -10), (10, 10), (0, 0), (-10, 10)
    ]];
    test_result(result, &expected);
}
#[test]
fn hair_test() {
    let hair = test_helper::get_polygon("hair");
    let null = unsafe { Polygon::trivial(Path::new(&vec![])) };
    let result = Snipper::union(hair, null).unwrap();
    let expected = vec![vec![
        (-15, 0), (-5, -10), (5, 0), (-5, 10)
    ]];
    test_result(result, &expected);
}
#[test]
fn simple_graph_test() {
    let horizontal = test_helper::get_polygon("horizontal");
    let vertical = test_helper::get_polygon("vertical");
    let result = Snipper::union(
        horizontal, vertical
    ).unwrap();
    let expected = vec![
        vec![
            (0, 2), (2, 2), (2, 0), (5, 0), (5, 2), (7, 2), (7, 5), (5, 5), (5, 7), (2, 7), (2, 5), (0, 5)
        ],
        vec![
            (1, 4), (2, 4), (2, 3), (1, 3)
        ],
        vec![
            (3, 2), (4, 2), (4, 1), (3, 1)
        ],
        vec![
            (3, 4), (4, 4), (4, 3), (3, 3)
        ],
        vec![
            (3, 6), (4, 6), (4, 5), (3, 5)
        ],
        vec![
            (5, 4), (6, 4), (6, 3), (5, 3)
        ],
    ];
    let structure = vec![None, Some(0), Some(0), Some(0), Some(0), Some(0)];
    test_result_and_structure(result, &expected, Some(&structure));
}