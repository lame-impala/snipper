use lazy_static;
use std::collections::{HashMap, HashSet};
use crate::primitives::{
    AbstractPoint,
    Point,
};
use crate::shape::Path;
use crate::shape::Polygon;
use crate::units::Coordinate;
use crate::edge::{Edge};
use crate::operation::Operation;
use crate::{Snipper};
use crate::intersection_algorithm::IntersectionAlgorithm;
use crate::api::Solution;


lazy_static! {
    static ref TEST_POLYGONS: HashMap<&'static str, Vec<Point>> = test_polygons();
}
fn test_polygons() -> HashMap<&'static str, Vec<Point>> {
    let mut map = HashMap::new();
    let big = vec![
        Point::new(0, 0).expect("!"),
        Point::new(30, 0).expect("!"),
        Point::new(30, 30).expect("!"),
        Point::new(0, 30).expect("!")];
    map.insert("big", big);
    let center = vec![
        Point::new(10, 10).expect("!"),
        Point::new(20, 10).expect("!"),
        Point::new(20, 20).expect("!"),
        Point::new(10, 20).expect("!")
    ];
    map.insert("center", center);
    let top_left = vec![
        Point::new(0, 0).expect("!"),
        Point::new(10, 0).expect("!"),
        Point::new(10, 10).expect("!"),
        Point::new(0, 10).expect("!")
    ];
    map.insert("top_left", top_left);
    let bottom_right = vec![
        Point::new(10, 10).expect("!"),
        Point::new(30, 10).expect("!"),
        Point::new(30, 30).expect("!"),
        Point::new(10, 30).expect("!")
    ];
    map.insert("bottom_right", bottom_right);
    let touch = vec![
        Point::new(30, 0).expect("!"),
        Point::new(40, 0).expect("!"),
        Point::new(40, 10).expect("!"),
        Point::new(30, 10).expect("!")
    ];
    map.insert("touch", touch);
    let remote = vec![
        Point::new(40, 0).expect("!"),
        Point::new(50, 0).expect("!"),
        Point::new(50, 10).expect("!"),
        Point::new(40, 10).expect("!")
    ];
    map.insert("remote", remote);
    let cross = vec![
        Point::new(20, 20).expect("!"),
        Point::new(40, 20).expect("!"),
        Point::new(40, 40).expect("!"),
        Point::new(20, 40).expect("!")
    ];
    map.insert("cross", cross);
    let big_cross = vec![
        Point::new(-10, -10).expect("!"),
        Point::new(20, -10).expect("!"),
        Point::new(20, 20).expect("!"),
        Point::new(-10, 20).expect("!")
    ];

    map.insert("big_cross", big_cross);
    let horizontal = vec![
        Point::new(20, 10).expect("!"),
        Point::new(40, 10).expect("!"),
        Point::new(40, 20).expect("!"),
        Point::new(20, 20).expect("!")
    ];
    map.insert("horizontal", horizontal);

    let null = vec![
        Point::new(0, 0).expect("!"),
        Point::new(0, 0).expect("!"),
        Point::new(0, 0).expect("!"),
        Point::new(10, 0).expect("!"),
        Point::new(10, 0).expect("!"),
        Point::new(10, 0).expect("!"),
        Point::new(10, 10).expect("!"),
        Point::new(10, 10).expect("!"),
        Point::new(10, 10).expect("!"),
        Point::new(0, 10).expect("!"),
        Point::new(0, 10).expect("!"),
        Point::new(0, 10).expect("!"),
        Point::new(0, 0).expect("!")
    ];
    map.insert("null", null);

    let near_vertical = vec![
        Point::new(-1, -100).expect("!"),
        Point::new(199, -100).expect("!"),
        Point::new(201, 100).expect("!"),
        Point::new(1, 100).expect("!"),
    ];
    map.insert("near_vertical", near_vertical);

    let mildly_slanted = vec![
        Point::new(-10, -10).expect("!"),
        Point::new(10, -12).expect("!"),
        Point::new(10, 8).expect("!"),
        Point::new(-10, 10).expect("!"),
    ];
    map.insert("mildly_slanted", mildly_slanted);

    map
}
lazy_static! {
    pub static ref COMPLEX_POLYGONS: HashMap<&'static str, Vec<Point>> = complex_polygons();
}
fn complex_polygons() -> HashMap<&'static str, Vec<Point>> {
    let big = vec![
        Point::new(-1, -5).expect("!"),
        Point::new(5, -1).expect("!"),
        Point::new(1, 5).expect("!"),
        Point::new(-5, 1).expect("!")
    ];
    let cross = vec![
        Point::new(0, 0).expect("!"),
        Point::new(-6, -4).expect("!"),
        Point::new(-2, -10).expect("!"),
        Point::new(4, -6).expect("!")
    ];
    let touch = vec![
        Point::new(0, 0).expect("!"),
        Point::new(3, 2).expect("!"),
        Point::new(1, 5).expect("!"),
        Point::new(-2, 3).expect("!")
    ];
    let aligned = vec![
        Point::new(0, -3).expect("!"),
        Point::new(8, -3).expect("!"),
        Point::new(8, 2).expect("!"),
        Point::new(0, 2).expect("!")
    ];
    let bow = vec![
        Point::new(0, 0).expect("!"),
        Point::new(0, -3).expect("!"),
        Point::new(2, -3).expect("!"),
        Point::new(0, 0).expect("!"),
        Point::new(0, 3).expect("!"),
        Point::new(-2, 3).expect("!")
    ];
    let diagonal = vec![
        Point::new(0, -3).expect("!"),
        Point::new(2, -3).expect("!"),
        Point::new(0, 3).expect("!"),
        Point::new(-2, 3).expect("!")
    ];
    let poke = vec![
        Point::new(-2, -2).expect("!"),
        Point::new(-1, -2).expect("!"),
        Point::new(-1, -1).expect("!")
    ];
    let pinch = vec![
        Point::new(-1, -1).expect("!"),
        Point::new(0, -1).expect("!"),
        Point::new(0, 0).expect("!")
    ];
    let pierce = vec![
        Point::new(-1, -2).expect("!"),
        Point::new(0, -2).expect("!"),
        Point::new(0, -1).expect("!"),
        Point::new(-1, -1).expect("!")
    ];
    let giant_square = vec![
        Point::new(Coordinate::MIN + 1, Coordinate::MIN + 1).expect("!"),
        Point::new(Coordinate::MAX - 1, Coordinate::MIN + 1).expect("!"),
        Point::new(Coordinate::MAX - 1, Coordinate::MAX - 1).expect("!"),
        Point::new(Coordinate::MIN + 1, Coordinate::MAX - 1).expect("!")
    ];
    let giant_polygon = vec![
        Point::new(Coordinate::MIN, Coordinate::MIN).expect("!"),
        Point::new(Coordinate::MAX - 2, Coordinate::MIN + 2).expect("!"),
        Point::new(Coordinate::MAX - 2, Coordinate::MAX - 2).expect("!"),
        Point::new(0, Coordinate::MAX).expect("!"),
        Point::new(Coordinate::MIN, Coordinate::MAX - 2).expect("!")
    ];
    let invalid_bow = vec![
        Point::new(-10, -10).expect("!"),
        Point::new(10, 10).expect("!"),
        Point::new(10, -10).expect("!"),
        Point::new(-10, 10).expect("!")
    ];
    let star = vec![
        Point::new(0, 0).expect("!"),
        Point::new(10, -20).expect("!"),
        Point::new(20, 0).expect("!"),
        Point::new(0, -10).expect("!"),
        Point::new(20, -10).expect("!")
    ];

    let null0 = vec![
        Point::new(0, 0).expect("!"),
        Point::new(-4, 6).expect("!")
    ];

    let null1 = vec![
        Point::new(2, -3).expect("!"),
        Point::new(8, 1).expect("!")
    ];

    let vane = vec![
        Point::new(0, 0).expect("!"),
        Point::new(-2, -2).expect("!"),
        Point::new(-6, -2).expect("!"),
        Point::new(-6, -4).expect("!"),
        Point::new(-4, -2).expect("!"),
        Point::new(-0, -2).expect("!")
    ];


    let null0_ns = vec![
        Point::new(0, 0).expect("!"),
        Point::new(0, 0).expect("!"),
        Point::new(0, 0).expect("!"),
        Point::new(-4, 6).expect("!"),
        Point::new(-4, 6).expect("!"),
        Point::new(-4, 6).expect("!"),
        Point::new(0, 0).expect("!"),
        Point::new(0, 0).expect("!"),
        Point::new(0, 0).expect("!")
    ];

    let null1_ns = vec![
        Point::new(2, -3).expect("!"),
        Point::new(2, -3).expect("!"),
        Point::new(8, 1).expect("!"),
        Point::new(8, 1).expect("!")
    ];

    let vane_ns = vec![
        Point::new(0, 0).expect("!"),
        Point::new(0, 0).expect("!"),
        Point::new(-2, -2).expect("!"),
        Point::new(-2, -2).expect("!"),
        Point::new(-6, -2).expect("!"),
        Point::new(-6, -2).expect("!"),
        Point::new(-6, -4).expect("!"),
        Point::new(-6, -4).expect("!"),
        Point::new(-4, -2).expect("!"),
        Point::new(-4, -2).expect("!"),
        Point::new(-0, -2).expect("!"),
        Point::new(-0, -2).expect("!"),
        Point::new(0, 0).expect("!"),
        Point::new(0, 0).expect("!")
    ];


    let rand0 = vec![
        Point::new(2, 5).expect("!"),
        Point::new(15, 2).expect("!"),
        Point::new(15, 3).expect("!"),
        Point::new(12, 6).expect("!"),
        Point::new(14, 3).expect("!")
    ];
    let rand1 = vec![
        Point::new(2, 11).expect("!"),
        Point::new(4, 0).expect("!"),
        Point::new(5, 7).expect("!"),
        Point::new(6, 6).expect("!"),
        Point::new(15, 1).expect("!"),
        Point::new(6, 10).expect("!"),
        Point::new(5, 7).expect("!")
    ];

    let rand2 = vec![
        Point::new(1, 2).expect("!"),
        Point::new(8, 7).expect("!"),
        Point::new(2, 5).expect("!")
    ];
    let rand3 = vec![
        Point::new(0, 3).expect("!"),
        Point::new(13, 2).expect("!"),
        Point::new(8, 4).expect("!")
    ];

    let bo00 = vec![
        Point::new(0, 0).expect("!"),
        Point::new(2, 0).expect("!"),
        Point::new(3, 5).expect("!"),
        Point::new(1, 11).expect("!"),
        Point::new(0, 11).expect("!")
    ];
    let bo01 = vec![
        Point::new(1, 1).expect("!"),
        Point::new(2, 1).expect("!"),
        Point::new(2, 2).expect("!"),
        Point::new(4, 6).expect("!"),
        Point::new(2, 6).expect("!"),
        Point::new(2, 7).expect("!"),
        Point::new(1, 7).expect("!"),
    ];

    let bo10 = vec![
        Point::new(0, 5).expect("!"),
        Point::new(5, -6).expect("!"),
        Point::new(5, -1).expect("!"),
        Point::new(6, 2).expect("!"),
        Point::new(4, 6).expect("!"),
    ];
    let bo11 = vec![
        Point::new(3, 0).expect("!"),
        Point::new(4, 0).expect("!"),
        Point::new(9, 1).expect("!"),
        Point::new(8, 3).expect("!"),
        Point::new(4, 4).expect("!"),
        Point::new(2, 4).expect("!"),
        Point::new(6, 3).expect("!"),
        Point::new(8, 1).expect("!"),
    ];
    let rand4 = vec![
        Point::new(2, 7).expect("!"),
        Point::new(3, 1).expect("!"),
        Point::new(4, 0).expect("!"),
    ];
    let rand5 = vec![
        Point::new(2, 3).expect("!"),
        Point::new(7, 5).expect("!"),
        Point::new(6, 7).expect("!"),
    ];

    let rand6 = vec![
        Point::new(2, 7).expect("!"),
        Point::new(4, 2).expect("!"),
        Point::new(4, 6).expect("!"),
    ];
    let rand7 = vec![
        Point::new(1, 7).expect("!"),
        Point::new(4, 4).expect("!"),
        Point::new(6, 4).expect("!"),
    ];

    let rand8 = vec![
        Point::new(3, 0).expect("!"),
        Point::new(6, 2).expect("!"),
        Point::new(3, 5).expect("!"),
    ];
    let rand9 = vec![
        Point::new(2, 2).expect("!"),
        Point::new(6, 4).expect("!"),
        Point::new(5, 7).expect("!"),
    ];

    let rand10 = vec![
        Point::new(1, 4).expect("!"),
        Point::new(7, 1).expect("!"),
        Point::new(6, 7).expect("!"),
    ];
    let rand11 = vec![
        Point::new(6, 0).expect("!"),
        Point::new(7, 3).expect("!"),
        Point::new(6, 6).expect("!"),
    ];

    let rand12 = vec![
        Point::new(0, 1).expect("!"),
        Point::new(2, 1).expect("!"),
        Point::new(1, 3).expect("!"),
    ];
    let rand13 = vec![
        Point::new(0, 7).expect("!"),
        Point::new(2, 0).expect("!"),
        Point::new(6, 4).expect("!"),
    ];


    let rand14 = vec![
        Point::new(2, 0).expect("!"),
        Point::new(4, 1).expect("!"),
        Point::new(5, 6).expect("!"),
    ];
    let rand15 = vec![
        Point::new(1, 4).expect("!"),
        Point::new(3, 0).expect("!"),
        Point::new(1, 5).expect("!"),
    ];
    let rand15_modified = vec![
        Point::new(1, 4).expect("!"),
        Point::new(3, 0).expect("!"),
        Point::new(5, -2).expect("!"),
    ];
    let rand16 = vec![
        Point::new(1, 7).expect("!"),
        Point::new(3, 2).expect("!"),
        Point::new(3, 4).expect("!"),
        Point::new(4, 2).expect("!"),
        Point::new(3, 7).expect("!"),
        Point::new(3, 4).expect("!"),
    ];
    let rand17 = vec![
        Point::new(3, 5).expect("!"),
        Point::new(4, 0).expect("!"),
        Point::new(5, 3).expect("!"),
        Point::new(5, 7).expect("!"),
    ];
    let rand18 = vec![
        Point::new(3, 0).expect("!"),
        Point::new(7, 6).expect("!"),
        Point::new(3, 1).expect("!"),
    ];
    let rand19 = vec![
        Point::new(6, 6).expect("!"),
        Point::new(7, 5).expect("!"),
        Point::new(6, 7).expect("!"),
    ];
    let rand20 = vec![
        Point::new(0, 0).expect("!"),
        Point::new(5, 1).expect("!"),
        Point::new(1, 4).expect("!"),
    ];
    let rand21 = vec![
        Point::new(2, 0).expect("!"),
        Point::new(7, 4).expect("!"),
        Point::new(2, 2).expect("!"),
    ];
    let rand22 = vec![
        Point::new(4, 6).expect("!"),
        Point::new(6, 1).expect("!"),
        Point::new(7, 0).expect("!"),
        Point::new(7, 3).expect("!"),
    ];
    let rand23 = vec![
        Point::new(0, 5).expect("!"),
        Point::new(6, 2).expect("!"),
        Point::new(2, 5).expect("!"),
    ];
    let rand24 = vec![
        Point::new(0, 4).expect("!"),
        Point::new(5, 2).expect("!"),
        Point::new(6, 5).expect("!"),
    ];
    let rand25 = vec![
        Point::new(0, 1).expect("!"),
        Point::new(3, 1).expect("!"),
        Point::new(4, 3).expect("!"),
        Point::new(0, 6).expect("!"),
    ];
    let rand26 = vec![
        Point::new(0, 0).expect("!"),
        Point::new(6, 5).expect("!"),
        Point::new(1, 2).expect("!"),
        // Point::new(1, 7).expect("!"),
    ];
    let rand27 = vec![
        Point::new(4, 5).expect("!"),
        Point::new(7, 0).expect("!"),
//        Point::new(7, 3).expect("!"),
        Point::new(7, 4).expect("!"),
    ];
    let rand28 = vec![
        Point::new(1, 0).expect("!"),
        Point::new(6, 4).expect("!"),
        Point::new(1, 1).expect("!"),
    ];
    let rand29 = vec![
        Point::new(5, 2).expect("!"),
        Point::new(7, 3).expect("!"),
        Point::new(6, 6).expect("!"),
    ];
    let rand30 = vec![
        Point::new(1, 1).expect("!"),
        Point::new(7, 1).expect("!"),
        Point::new(6, 6).expect("!"),
        Point::new(3, 6).expect("!"),
        Point::new(5, 4).expect("!"),
    ];
    let rand31 = vec![
        Point::new(1, 1).expect("!"),
        Point::new(2, 0).expect("!"),
        Point::new(4, 6).expect("!"),
    ];
    let rand32 = vec![
        Point::new(1, 15).expect("!"),
        Point::new(6, 4).expect("!"),
        Point::new(5, 3).expect("!"),
//        Point::new(7, 2).expect("!"),
        Point::new(6, 4).expect("!"),
        Point::new(8, 5).expect("!"),
    ];
    let rand33 = vec![
        Point::new(0, 13).expect("!"),
        Point::new(6, 3).expect("!"),
        Point::new(5, 7).expect("!"),
    ];
    // Traverse not expected
    let rand34 = vec![
        Point::new(5, 2).expect("!"),
        Point::new(6, 1).expect("!"),
        Point::new(7, 1).expect("!"),
        Point::new(6, 2).expect("!"),
    ];
    let rand35 = vec![
        Point::new(3, 2).expect("!"),
        Point::new(7, 3).expect("!"),
        Point::new(7, 4).expect("!"),
    ];

    // Unwrap None - switched x and y in Point::new
    let rand36 = vec![
        Point::new(1, 0).expect("!"),
        Point::new(6, 4).expect("!"),
        Point::new(1, 2).expect("!"),
    ];
    let rand37 = vec![
        Point::new(2, 0).expect("!"),
        Point::new(3, 3).expect("!"),
        Point::new(2, 3).expect("!"),
    ];
    let rand38 = vec![
        Point::new(2, 0).expect("!"),
        Point::new(4, 3).expect("!"),
        Point::new(3, 3).expect("!"),
    ];
    let rand39 = vec![
        Point::new(1, 3).expect("!"),
        Point::new(4, 0).expect("!"),
        Point::new(5, 7).expect("!"),
    ];

    let rand40 = vec![
        Point::new(2, 6).expect("!"),
        Point::new(6, 1).expect("!"),
        Point::new(7, 5).expect("!"),
    ];
    let rand41 = vec![
        Point::new(0, 0).expect("!"),
        Point::new(1, 0).expect("!"),
        Point::new(6, 3).expect("!"),
    ];

    let rand42 = vec![
        Point::new(3, 6).expect("!"),
        Point::new(7, 4).expect("!"),
        Point::new(3, 7).expect("!"),
    ];
    let rand43 = vec![
        Point::new(6, 7).expect("!"),
        Point::new(7, 0).expect("!"),
        Point::new(7, 7).expect("!"),
    ];

    let rand44 = vec![
        Point::new(4, 1).expect("!"),
        Point::new(10, 12).expect("!"),
        Point::new(6, 14).expect("!"),
    ];
    let rand45 = vec![
        Point::new(4, 14).expect("!"),
        Point::new(5, 9).expect("!"),
        Point::new(7, 13).expect("!"),
    ];

    let rand46 = vec![
        Point::new(0, 4).expect("!"),
        Point::new(15, 9).expect("!"),
        Point::new(13, 13).expect("!"),
    ];
    let rand47 = vec![
        Point::new(5, 1).expect("!"),
        Point::new(7, 15).expect("!"),
        Point::new(6, 9).expect("!"),
    ];
    // this error is intermittent
    let rand48 = vec![
        Point::new(10, 4).expect("!"),
        Point::new(20, 9).expect("!"),
        Point::new(9, 26).expect("!"),
        Point::new(6, 16).expect("!"),
        Point::new(12, 26).expect("!"),
        Point::new(10, 4).expect("!"),
        Point::new(5, 17).expect("!"),
    ];
    let rand49 = vec![
        Point::new(4, 29).expect("!"),
        Point::new(9, 6).expect("!"),
        Point::new(6, 23).expect("!"),
        Point::new(10, 17).expect("!"),
    ];
    // this error is intermittent
    let rand50 = vec![
        Point::new(1, 2).expect("!"),
        Point::new(14, 6).expect("!"),
        Point::new(16, 2).expect("!"),
        Point::new(16, 21).expect("!"),
        Point::new(25, 11).expect("!"),
        Point::new(18, 26).expect("!"),
    ];
    let rand51 = vec![
        Point::new(4, 24).expect("!"),
        Point::new(13, 20).expect("!"),
        Point::new(11, 25).expect("!"),
        Point::new(15, 24).expect("!"),
        Point::new(22, 23).expect("!"),
        Point::new(15, 25).expect("!"),
        Point::new(29, 27).expect("!"),
        Point::new(6, 28).expect("!"),
        Point::new(15, 25).expect("!"),
        Point::new(11, 25).expect("!"),
    ];
    let rand52 = vec![
        Point::new(0, 0).expect("!"),
        Point::new(4, 10).expect("!"),
        Point::new(10, 28).expect("!")
    ];
    let rand53 = vec![
        Point::new(8, 24).expect("!"),
        Point::new(19, 19).expect("!"),
        Point::new(17, 28).expect("!")
    ];
    let rand54 = vec![
        Point::new(1, 5).expect("!"),
        Point::new(5, 15).expect("!"),
        Point::new(7, 6).expect("!"),
        Point::new(10, 30).expect("!")
    ];
    let rand55 = vec![
        // Point::new(5, 27).expect("!"),
        // Point::new(6, 26).expect("!"),
        Point::new(7, 25).expect("!"),
        Point::new(9, 19).expect("!"),
        Point::new(19, 14).expect("!"),
        Point::new(7, 25).expect("!"),
        // Point::new(6, 27).expect("!")
    ];
    let rand54r = vec![
        Point::new(1, 5).expect("!"),
//        Point::new(5, 15).expect("!"),
        Point::new(7, 6).expect("!"),
        Point::new(10, 30).expect("!")
    ];
    let rand55r = vec![
        // Point::new(5, 27).expect("!"),
        // Point::new(6, 26).expect("!"),
//        Point::new(7, 25).expect("!"),
        Point::new(9, 19).expect("!"),
        Point::new(19, 14).expect("!"),
        Point::new(7, 25).expect("!"),
        // Point::new(6, 27).expect("!")
    ];

    let rand56 = vec![
        Point::new(11, 31).expect("!"),
        Point::new(19, 1).expect("!"),
        Point::new(24, 16).expect("!"),
        Point::new(15, 17).expect("!")
    ];
    let rand57 = vec![
        // Point::new(4, 23).expect("!"),
        Point::new(17, 6).expect("!"),
        Point::new(19, 4).expect("!"),
        Point::new(22, 9).expect("!"),
        Point::new(17, 6).expect("!"),
        // Point::new(17, 7).expect("!")
    ];

    let rand58 = vec![
        Point::new(7, 30).expect("!"),
        Point::new(17, 7).expect("!"),
        Point::new(30, 6).expect("!"),
        Point::new(17, 14).expect("!")
    ];
    let rand59 = vec![
        Point::new(3, 7).expect("!"),
        Point::new(15, 8).expect("!"),
        Point::new(16, 3).expect("!"),
        Point::new(20, 10).expect("!"),
        Point::new(15, 8).expect("!"),
        Point::new(13, 15).expect("!")
    ];
    // Expected lower position to be there
    let rand60 = vec![
        //Point::new(0, 19).expect("!"),
        Point::new(26, 7).expect("!"),
        Point::new(30, 7).expect("!"),
        Point::new(29, 8).expect("!")
    ];
    let rand61 = vec![
        //Point::new(15, 8).expect("!"),
        Point::new(26, 21).expect("!"),
        Point::new(31, 2).expect("!"),
        Point::new(26, 24).expect("!")
    ];
    let rand62 = vec![
        Point::new(2, 0).expect("!"),
        Point::new(6, 8).expect("!"),
        Point::new(3, 0).expect("!"),
        //Point::new(6, 7).expect("!"),
        Point::new(9, 14).expect("!"),
        Point::new(6, 8).expect("!"),
        Point::new(2, 1).expect("!"),
    ];
    let rand63 = vec![
        //Point::new(15, 8).expect("!"),
        Point::new(3, 8).expect("!"),
        Point::new(5, 8).expect("!"),
        Point::new(13, 2).expect("!"),
        Point::new(13, 15).expect("!")
    ];
    let norm0 = vec![
        Point::new(31, 8).expect("!"),
        Point::new(8, 17).expect("!"),
        Point::new(27, 12).expect("!"),
        Point::new(29, 31).expect("!"),
        Point::new(30, 7).expect("!"),
        Point::new(5, 13).expect("!"),
        Point::new(12, 20).expect("!"),
        Point::new(30, 8).expect("!"),
        Point::new(25, 18).expect("!"),
        Point::new(6, 16).expect("!"),
        Point::new(7, 1).expect("!"),
        Point::new(12, 30).expect("!"),
        Point::new(25, 17).expect("!"),
        //Point::new(18, 9).expect("!"),
        Point::new(8, 3).expect("!"),
        // Point::new(23, 6).expect("!")
    ];
    let norm0p0 = vec! [
        Point::new(30, 8).expect("!"),
        Point::new(22, 13).expect("!"),
        Point::new(13, 15).expect("!"),
        Point::new(11, 16).expect("!"),
        Point::new(25, 17).expect("!"),
        Point::new(22, 13).expect("!"),
        Point::new(27, 12).expect("!"),
        Point::new(30, 8).expect("!"),
        Point::new(17, 10).expect("!"),
        Point::new(13, 7).expect("!")
    ];
    let norm0p1 = vec! [
        Point::new(30, 8).expect("!"),
        Point::new(25, 18).expect("!"),
        Point::new(9, 17).expect("!"),
        Point::new(6, 14).expect("!")
    ];
    let ss01 = vec![
        Point::new(6, 2).unwrap(),
        Point::new(1, 0).unwrap(),
        Point::new(7, 3).unwrap(),
        Point::new(2, 3).unwrap()
    ];
    let ss02 = vec![
        Point::new(2, 3).unwrap(),
        Point::new(5, 3).unwrap(),
        Point::new(3, 5).unwrap()
    ];
    let ss03 = vec![
        Point::new(1, 6).unwrap(),
        Point::new(5, 5).unwrap(),
        Point::new(5, 6).unwrap()
    ];
    let ss04 = vec![
        Point::new(8, 1).unwrap(),
        Point::new(0, 10).unwrap(),
        Point::new(2, 4).unwrap(),
        Point::new(15, 10).unwrap(),
        Point::new(0, 1).unwrap(),
        Point::new(15, 11).unwrap(),
        Point::new(7, 14).unwrap(),
        Point::new(1, 15).unwrap()
    ];
    let ss05 = vec![
        Point::new(4, 2).unwrap(),
        Point::new(29, 19).unwrap(),
        Point::new(3, 16).unwrap(),
        Point::new(12, 21).unwrap(),
        Point::new(1, 11).unwrap(),
        Point::new(7, 19).unwrap(),
        Point::new(17, 27).unwrap(),
        Point::new(10, 26).unwrap(),
        Point::new(31, 20).unwrap(),
        Point::new(1, 26).unwrap(),
        Point::new(2, 4).unwrap(),
        Point::new(17, 9).unwrap(),
        Point::new(3, 30).unwrap(),
        Point::new(3, 3).unwrap(),
        Point::new(4, 3).unwrap(),
        Point::new(29, 17).unwrap()
    ];
    let ss06 = vec![
        Point::new(25, 10).unwrap(),
        Point::new(12, 1).unwrap(),
        Point::new(27, 10).unwrap(),
        Point::new(8, 0).unwrap(),
        Point::new(30, 2).unwrap(),
        //Point::new(14, 24).unwrap(),
        Point::new(7, 23).unwrap(),
        Point::new(14, 31).unwrap(),
        Point::new(21, 6).unwrap(),
        Point::new(10, 23).unwrap(),
        //Point::new(15, 28).unwrap(),
        //Point::new(6, 14).unwrap(),
        Point::new(31, 8).unwrap(),
        Point::new(0, 4).unwrap(),
        //Point::new(28, 31).unwrap(),
        Point::new(28, 31).unwrap(),
        Point::new(31, 28).unwrap()
    ];

    let ss07 = vec![
        Point::new(17, 21).unwrap(),
        Point::new(27, 6).unwrap(),
        Point::new(5, 28).unwrap(),
        Point::new(10, 21).unwrap(),
        //Point::new(12, 16).unwrap(),
        //Point::new(6, 15).unwrap(),
        Point::new(29, 30).unwrap(),
        //Point::new(21, 31).unwrap(),
        //Point::new(12, 26).unwrap(),
        Point::new(11, 19).unwrap(),
        Point::new(20, 5).unwrap(),
        Point::new(21, 13).unwrap(),
        //Point::new(14, 3).unwrap(),
        Point::new(10, 23).unwrap(),
        Point::new(31, 1).unwrap(),
        Point::new(24, 5).unwrap()
    ];
    let ss08 = vec![
        Point::new(11, 11).unwrap(),
        //Point::new(26, 22).unwrap(),
        //Point::new(13, 12).unwrap(),
        //Point::new(29, 12).unwrap(),
        //Point::new(15, 18).unwrap(),
        //Point::new(20, 0).unwrap(),
        //Point::new(21, 9).unwrap(),
        //Point::new(19, 16).unwrap(),
        Point::new(10, 5).unwrap(),
        //Point::new(18, 21).unwrap(),
        Point::new(25, 24).unwrap(),
        Point::new(0, 2).unwrap(),
        Point::new(14, 15).unwrap(),
        //Point::new(29, 27).unwrap(),
        Point::new(1, 4).unwrap(),
        Point::new(14, 17).unwrap(),
    ];
    let ss09 = vec![
        //Point::new(21, 12).unwrap(),
        //Point::new(22, 31).unwrap(),
        //Point::new(5, 1).unwrap(),
        //Point::new(30, 10).unwrap(),
        Point::new(12, 13).unwrap(),
        Point::new(23, 10).unwrap(),
        //Point::new(19, 10).unwrap(),
        Point::new(19, 31).unwrap(),
        //Point::new(17, 28).unwrap(),
        //Point::new(6, 15).unwrap(),
        Point::new(26, 22).unwrap(),
        Point::new(23, 17).unwrap(),
        Point::new(21, 24).unwrap(),
        Point::new(11, 2).unwrap(),
        Point::new(23, 19).unwrap(),
        //Point::new(7, 22).unwrap(),
    ];
    let ss10 = vec![
        Point::new(30, 31).unwrap(),
        Point::new(7, 2).unwrap(),
        Point::new(20, 2).unwrap(),
        Point::new(1, 21).unwrap(),
        //Point::new(0, 24).unwrap(),
        //Point::new(10, 1).unwrap(),
        //Point::new(12, 26).unwrap(),
        Point::new(24, 13).unwrap(),
        Point::new(7, 31).unwrap(),
        Point::new(20, 24).unwrap(),
        Point::new(21, 9).unwrap(),
        //Point::new(23, 18).unwrap(),
        //Point::new(21, 13).unwrap(),
        //Point::new(21, 14).unwrap(),
        //Point::new(1, 4).unwrap(),
        //Point::new(30, 12).unwrap(),
    ];
    let mut map = HashMap::new();
    map.insert("big", big);
    map.insert("cross", cross);
    map.insert("touch", touch);
    map.insert("aligned", aligned);
    map.insert("bow", bow);
    map.insert("diagonal", diagonal);
    map.insert("poke", poke);
    map.insert("pinch", pinch);
    map.insert("pierce", pierce);
    map.insert("giant_square", giant_square);
    map.insert("giant_polygon", giant_polygon);
    map.insert("invalid_bow", invalid_bow);
    map.insert("star", star);
    map.insert("null0", null0);
    map.insert("null1", null1);
    map.insert("vane", vane);
    map.insert("null0_ns", null0_ns);
    map.insert("null1_ns", null1_ns);
    map.insert("vane_ns", vane_ns);
//    map.insert("rect0", rect0);
//    map.insert("rect1", rect1);
//    map.insert("rect2", rect2);
//    map.insert("rect3", rect3);
//    map.insert("rect4", rect4);
//    map.insert("rect5", rect5);
    map.insert("rand0", rand0);
    map.insert("rand1", rand1);
    map.insert("rand2", rand2);
    map.insert("rand3", rand3);
    map.insert("rand4", rand4);
    map.insert("rand5", rand5);
    map.insert("rand6", rand6);
    map.insert("rand7", rand7);
    map.insert("rand8", rand8);
    map.insert("rand9", rand9);
    map.insert("rand10", rand10);
    map.insert("rand11", rand11);
    map.insert("rand12", rand12);
    map.insert("rand13", rand13);
    map.insert("rand14", rand14);
    map.insert("rand15", rand15);
    map.insert("rand15_modified", rand15_modified);
    map.insert("rand16", rand16);
    map.insert("rand17", rand17);
    map.insert("rand18", rand18);
    map.insert("rand19", rand19);
    map.insert("rand20", rand20);
    map.insert("rand21", rand21);
    map.insert("rand22", rand22);
    map.insert("rand23", rand23);
    map.insert("rand24", rand24);
    map.insert("rand25", rand25);
    map.insert("rand26", rand26);
    map.insert("rand27", rand27);
    map.insert("rand28", rand28);
    map.insert("rand29", rand29);
    map.insert("rand30", rand30);
    map.insert("rand31", rand31);
    map.insert("rand32", rand32);
    map.insert("rand33", rand33);
    map.insert("rand34", rand34);
    map.insert("rand35", rand35);
    map.insert("rand36", rand36);
    map.insert("rand37", rand37);
    map.insert("rand38", rand38);
    map.insert("rand39", rand39);
    map.insert("rand40", rand40);
    map.insert("rand41", rand41);
    map.insert("rand42", rand42);
    map.insert("rand43", rand43);
    map.insert("rand44", rand44);
    map.insert("rand45", rand45);
    map.insert("rand46", rand46);
    map.insert("rand47", rand47);
    map.insert("rand48", rand48);
    map.insert("rand49", rand49);
    map.insert("rand50", rand50);
    map.insert("rand51", rand51);
    map.insert("rand52", rand52);
    map.insert("rand53", rand53);
    map.insert("rand54", rand54);
    map.insert("rand55", rand55);
    map.insert("rand54r", rand54r);
    map.insert("rand55r", rand55r);
    map.insert("rand56", rand56);
    map.insert("rand57", rand57);
    map.insert("rand58", rand58);
    map.insert("rand59", rand59);
    map.insert("rand60", rand60);
    map.insert("rand61", rand61);
    map.insert("rand62", rand62);
    map.insert("rand63", rand63);
    map.insert("bo00", bo00);
    map.insert("bo01", bo01);
    map.insert("bo10", bo10);
    map.insert("bo11", bo11);
    map.insert("norm0", norm0);
    map.insert("norm0p0", norm0p0);
    map.insert("norm0p1", norm0p1);

    map.insert("ss01", ss01);
    map.insert("ss02", ss02);
    map.insert("ss03", ss03);
    map.insert("ss04", ss04);
    map.insert("ss05", ss05);
    map.insert("ss06", ss06);
    map.insert("ss07", ss07);
    map.insert("ss08", ss08);
    map.insert("ss09", ss09);
    map.insert("ss10", ss10);

    map
}
#[allow(dead_code)]
pub fn perform_test(
    polygons: &Vec<Polygon>,
    ids: &HashSet<usize>,
    operation: &Operation,
    expected: &Vec<Vec<usize>>) {

    let mut selected: Vec<Polygon> = polygons.iter().enumerate().filter_map(|(idx, polygon)| {
        if ids.contains(&idx) {
            Some(polygon.clone())
        } else {
            None
        }
    }).collect();
    assert_eq!(selected.len(), 2);
    let edges = IntersectionAlgorithm::perform(
        selected[0].clone(), selected[1].clone()
    ).unwrap();
    let all = all_graph_points(edges);
    let b = selected.pop().unwrap();
    let a = selected.pop().unwrap();
    let p = if operation  == &Operation::UNION {
        Snipper::union(a, b).unwrap()
    } else if operation == &Operation::INTERSECTION {
        Snipper::intersection(a, b).unwrap()
    } else if operation == &Operation::XOR {
        Snipper::xor(a, b).unwrap()
    } else if operation == &Operation::DIFFERENCE {
        assert_eq!(selected.len(), 2);
        Snipper::difference(a, b).unwrap()
    } else {
        panic!("Unknown operation: {}", operation.id())
    };
    test_result(p, &all, expected);

}

pub fn get_polygon(name: &str) -> Polygon {
    let path = Path::new(&TEST_POLYGONS[name]);
    unsafe {
        Polygon::trivial(path)
    }
}

pub fn get_complex_polygon(name: &str) -> Polygon {
    let path = Path::new(&COMPLEX_POLYGONS[name]);
    unsafe {
        Polygon::trivial(path)
    }
}

pub fn all_graph_points(edges: Vec<Edge>) -> Vec<Point> {
    let set: HashSet<Point> = edges.iter().flat_map(|edge| {
        vec![edge.straight.start.clone(), edge.straight.end.clone()]
    }).collect();
    let mut all: Vec<Point> = set.iter().cloned().collect();
    all.sort_by(|a: &Point, b: &Point| {
        let cmp = a.x().cmp(&b.x());
        if cmp == std::cmp::Ordering::Equal {
            a.y().cmp(&b.y())
        } else {
            cmp
        }
    });
    let strings: Vec<String> = all.iter().enumerate().map(|(idx, point)| {
        format!("#{}[{}, {}]", idx, point.x().to_int(), point.y().to_int())
    }).collect();
    println!("ALL: {}", strings.join(", "));
    all
}
pub fn test_operation_and_structure(
    operands: Vec<Polygon>,
    operation: &Operation,
    expected: &Vec<Vec<usize>>,
    structure: Option<&Vec<Option<usize>>>
) {
    assert_eq!(operands.len(), 2);
    let edges = IntersectionAlgorithm::perform(operands[0].clone(), operands[1].clone()).unwrap();
    let all = all_graph_points(edges);
    let result = match operation {
        _ if operation == &Operation::UNION => Snipper::union(operands[0].clone(), operands[1].clone()),
        _ if operation == &Operation::INTERSECTION => Snipper::intersection(operands[0].clone(), operands[1].clone()),
        _ if operation == &Operation::XOR => Snipper::xor(operands[0].clone(), operands[1].clone()),
        _ if operation == &Operation::DIFFERENCE => {
            Snipper::difference(operands[0].clone(), operands[1].clone())
        },
        _ => panic!("Unknown operation")
    };
    test_result_and_structure(result.unwrap(), &all, expected, structure);
}
pub fn test_operation(operands: Vec<Polygon>, operation: &Operation, expected: &Vec<Vec<usize>>) {
    test_operation_and_structure(operands, operation, expected, None);
}
pub fn test_result_and_structure(
    solution: Solution,
    points: &Vec<Point>,
    expected: &Vec<Vec<usize>>,
    structure: Option<&Vec<Option<usize>>>
) {
    let result = solution.polygon().unwrap();
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
            let expected_point_index = expected_path[point_index];
            let expected_point = &points[expected_point_index];
            if expected_point != point {
                panic!(
                    "Unexpected point at index #{} in path #{}: {}, expected: #{}/{} -- {}",
                    point_index,
                    path_index,
                    point.inspect(),
                    expected_point_index,
                    expected_point.inspect(),
                    path.inspect()
                )
            }
        }
    }
    if let Some(structure) = structure {
        debug_assert!(
            expected.len() == structure.len(),
            "Expected paths and structure should be of the same length, got: {} vs. {}",
            expected.len(),
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
#[allow(dead_code)]
pub fn test_result(solution: Solution, points: &Vec<Point>, expected: &Vec<Vec<usize>>) {
    test_result_and_structure(solution, points, expected, None);
}
lazy_static! {
    static ref SUPERCOMPLEX_POLYGONS: HashMap<&'static str, Polygon> = supercomplex_polygons();
}
pub fn supercomplex_polygons() -> HashMap<&'static str, Polygon> {
    fn build(points: Vec<Point>) -> Polygon {
        let path = Path::new(&points);
        let poly = unsafe {
            Polygon::trivial(path)
        };
        poly
    }
    // FIRST: 3 paths:
    let a00 = build (vec![
        Point::new(2, 2).unwrap(),
        Point::new(11, 11).unwrap(),
        Point::new(7, 7).unwrap(),
        Point::new(12, 12).unwrap(),
        Point::new(13, 13).unwrap(),
        Point::new(3, 3).unwrap(),
        Point::new(13, 13).unwrap(),
        Point::new(16, 16).unwrap(),
        Point::new(24, 24).unwrap(),
        Point::new(26, 26).unwrap(),
        Point::new(33, 33).unwrap(),
        Point::new(43, 43).unwrap(),
        Point::new(44, 44).unwrap(),
        Point::new(62, 62).unwrap(),
        Point::new(42, 42).unwrap(),
        Point::new(42, 42).unwrap(),
        Point::new(58, 58).unwrap(),
        Point::new(42, 42).unwrap(),
        Point::new(42, 42).unwrap(),
        Point::new(37, 37).unwrap(),
        Point::new(35, 35).unwrap(),
        Point::new(40, 40).unwrap(),
        Point::new(35, 35).unwrap(),
        Point::new(41, 41).unwrap(),
        Point::new(39, 39).unwrap(),
        Point::new(25, 25).unwrap(),

    ]);

    let a01 = build(vec![
        Point::new(25, 25).unwrap(),
        Point::new(30, 30).unwrap(),
        Point::new(34, 34).unwrap(),
        Point::new(35, 35).unwrap(),
        Point::new(35, 35).unwrap(),
        Point::new(34, 34).unwrap(),
        Point::new(31, 31).unwrap(),
        Point::new(33, 33).unwrap(),
        Point::new(35, 35).unwrap(),
        Point::new(37, 37).unwrap(),
        Point::new(38, 38).unwrap(),
        Point::new(42, 42).unwrap(),
        Point::new(42, 42).unwrap(),
        Point::new(39, 39).unwrap(),
        Point::new(42, 42).unwrap(),
        Point::new(45, 45).unwrap(),
        Point::new(43, 43).unwrap(),
        Point::new(42, 42).unwrap(),
        Point::new(37, 37).unwrap(),
        Point::new(35, 35).unwrap(),
        Point::new(33, 33).unwrap(),
        Point::new(28, 28).unwrap(),
        Point::new(31, 31).unwrap(),
        Point::new(30, 30).unwrap(),
        Point::new(15, 15).unwrap(),
        Point::new(18, 18).unwrap(),
        Point::new(26, 26).unwrap(),
        Point::new(27, 27).unwrap(),
        Point::new(26, 26).unwrap(),
        Point::new(24, 24).unwrap(),
        Point::new(18, 18).unwrap(),
        Point::new(13, 13).unwrap(),
        Point::new(13, 13).unwrap(),
        Point::new(15, 15).unwrap(),
        Point::new(12, 12).unwrap(),
        Point::new(11, 11).unwrap(),
    ]);

    let a02 = build(vec![
        Point::new(28, 28).unwrap(),
        Point::new(33, 33).unwrap(),
        Point::new(34, 34).unwrap(),
        Point::new(35, 35).unwrap(),
        Point::new(39, 39).unwrap(),
        Point::new(38, 38).unwrap(),
        Point::new(35, 35).unwrap(),
        Point::new(34, 34).unwrap(),
        Point::new(33, 33).unwrap(),

    ]);

    // SECOND: 3 paths:
    let a10 = build(vec![
        Point::new(1, 1).unwrap(),
        Point::new(9, 9).unwrap(),
        Point::new(8, 8).unwrap(),
        Point::new(9, 9).unwrap(),
        Point::new(10, 10).unwrap(),
        Point::new(38, 38).unwrap(),
        Point::new(38, 38).unwrap(),
        Point::new(39, 39).unwrap(),
        Point::new(38, 38).unwrap(),
        Point::new(39, 39).unwrap(),
        Point::new(44, 44).unwrap(),
        Point::new(60, 60).unwrap(),
        Point::new(55, 55).unwrap(),
        Point::new(50, 50).unwrap(),
        Point::new(26, 26).unwrap(),
        Point::new(24, 24).unwrap(),
        Point::new(17, 17).unwrap(),
        Point::new(25, 25).unwrap(),
        Point::new(43, 43).unwrap(),
        Point::new(3, 3).unwrap(),
        Point::new(17, 17).unwrap(),
        Point::new(11, 11).unwrap(),

    ]);

    let a11 = build(vec![
        Point::new(11, 11).unwrap(),
        Point::new(13, 13).unwrap(),
        Point::new(16, 16).unwrap(),
        Point::new(24, 24).unwrap(),
        Point::new(31, 31).unwrap(),
        Point::new(13, 13).unwrap(),
        Point::new(10, 10).unwrap(),
        Point::new(9, 9).unwrap()

    ]);

    let a12 = build(vec![
        Point::new(39, 39).unwrap(),
        Point::new(39, 39).unwrap(),
        Point::new(44, 44).unwrap(),
        Point::new(40, 40).unwrap(),
        Point::new(39, 39).unwrap(),
        Point::new(38, 38).unwrap()
    ]);

    let a0a = Snipper::xor(a00, a01).unwrap().polygon().unwrap();
    let a0 = Snipper::xor(a0a, a02).unwrap().polygon().unwrap();
    let a1a = Snipper::xor(a10, a11).unwrap().polygon().unwrap();
    let a1 = Snipper::xor(a1a, a12).unwrap().polygon().unwrap();

    let b00 = build(vec![
        Point::new(2, 2).unwrap(),
        Point::new(18, 4).unwrap(),
        Point::new(20, 4).unwrap(),
        Point::new(17, 9).unwrap(),
        Point::new(26, 14).unwrap(),
        Point::new(23, 14).unwrap(),
        Point::new(13, 25).unwrap(),
        Point::new(8, 31).unwrap(),
        Point::new(15, 12).unwrap(),
        Point::new(11, 18).unwrap(),
        Point::new(16, 9).unwrap(),
    ]);
    let b10 = build(vec![
        Point::new(0, 9).unwrap(),
        Point::new(11, 11).unwrap(),
        Point::new(11, 9).unwrap(),
        Point::new(14, 12).unwrap(),
        Point::new(27, 15).unwrap(),
        Point::new(20, 18).unwrap(),
        Point::new(29, 27).unwrap(),
        Point::new(0, 29).unwrap(),
        Point::new(16, 21).unwrap(),
        Point::new(14, 18).unwrap(),
    ]);
    let b11 = build(vec![
        Point::new(11, 14).unwrap(),
        Point::new(14, 18).unwrap(),
        Point::new(17, 20).unwrap(),
        Point::new(16, 21).unwrap(),
        Point::new(18, 24).unwrap(),
        Point::new(22, 24).unwrap(),
        Point::new(17, 20).unwrap(),
        Point::new(20, 18).unwrap(),
        Point::new(14, 12).unwrap(),
        Point::new(11, 11).unwrap(),
    ]);
    let null = unsafe { Polygon::trivial(Path::new(&vec![])) };
    let b0 = Snipper::xor(b00, null).unwrap().polygon().unwrap();
    let b1 = Snipper::xor(b10, b11).unwrap().polygon().unwrap();

    let daeet0 = build(vec![
        Point::new(0, 20).unwrap(),
        Point::new(20, 0).unwrap(),
        Point::new(40, 20).unwrap(),
        Point::new(40, 30).unwrap(),
        Point::new(40, 40).unwrap(),
        Point::new(20, 40).unwrap(),
        Point::new(0, 30).unwrap(),
    ]);
    let daeet1 = build(vec![
        Point::new(0, 30).unwrap(),
        Point::new(20, 30).unwrap(),
        Point::new(10, 20).unwrap(),
        Point::new(20, 10).unwrap(),
        Point::new(30, 20).unwrap(),
        Point::new(20, 30).unwrap(),
        Point::new(40, 40).unwrap(),
        Point::new(20, 50).unwrap(),
        Point::new(30, 60).unwrap(),
        Point::new(20, 70).unwrap(),
        Point::new(10, 60).unwrap(),
        Point::new(20, 50).unwrap(),
        Point::new(0, 40).unwrap(),
    ]);

    let mut map = HashMap::new();
    map.insert("a0", a0);
    map.insert("a1", a1);
    map.insert("b0", b0);
    map.insert("b1", b1);
    map.insert("daeet0", daeet0);
    map.insert("daeet1", daeet1);

    map
}

pub fn get_supercomplex_polygon(name: &str) -> Polygon {
    let poly = &SUPERCOMPLEX_POLYGONS[name];
    poly.clone()
}
