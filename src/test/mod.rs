pub mod test_helper;

pub use test_helper::*;

use crate::primitives::{Point};
use crate::shape::Polygon;
use super::Queue;

use crate::test::test_helper::{
    all_graph_points,
    get_complex_polygon,
    test_operation,
    get_polygon,
    get_supercomplex_polygon,
};
use crate::intersection_algorithm::IntersectionAlgorithm;
use crate::operation::{Operation, Operand};
use crate::{Snipper, Path};

#[test]
fn near_vertical_test() {
    let nv = get_polygon("near_vertical");
    let ms = get_polygon("mildly_slanted");
    let operands = vec![nv, ms];
    let edges = IntersectionAlgorithm::perform(operands[0].clone(), operands[1].clone()).unwrap();
    let all: Vec<Point> = all_graph_points(edges);
    // ALL:
    // #0[-10, -10], #1[-10, 10],
    // #2[-1, -100],
    // #3[0, -11],
    // #4[0, 9],
    // #5[1, 100],
    // #6[10, -11], #7[10, 9],
    // #8[199, -100], #9[201, 100]
    assert_eq!(all.len(), 10); // there are two intersection points at 0
    assert_eq!(all[3], Point::new(0, -11).expect("!"));
    assert_eq!(all[4], Point::new(0, 9).expect("!"));

}
#[test]
fn bo_intersection_test() {
    let mut operands = Vec::new();
    operands.push(get_polygon("big"));
    operands.push(get_polygon("cross"));

    let all = IntersectionAlgorithm::perform(operands[0].clone(), operands[1].clone()).unwrap();
    assert_eq!(all.len(), 12);

    let mut operands = Vec::new();
    operands.push(get_polygon("big"));
    operands.push(get_polygon("horizontal"));

    let all = IntersectionAlgorithm::perform(operands[0].clone(), operands[1].clone()).unwrap();
    assert_eq!(all.len(), 12);

}
#[test]
fn null_segment_polygon_test() {
    let mut g1 = Queue::new();
    let _ = g1.add_operand(get_polygon("null"), Operand::Clipping);
    assert_eq!(g1.num_edges(), 4);
}

#[test]
fn first_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand0"),
        get_complex_polygon("rand1")
    );
    assert!(r.is_ok());
}
#[test]
fn second_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand2"),
        get_complex_polygon("rand3")
    );
    assert!(r.is_ok());
}
#[test]
fn third_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand4"),
        get_complex_polygon("rand5")
    );
    assert!(r.is_ok());
}
#[test]
fn fourth_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand6"),
        get_complex_polygon("rand7")
    );
    assert!(r.is_ok());
}
#[test]
fn fifth_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand8"),
        get_complex_polygon("rand9")
    );
    assert!(r.is_ok());
}
#[test]
fn single_sixth_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand10"),
        get_complex_polygon("rand11")
    );
    assert!(r.is_ok());
}
#[test]
fn seventh_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand12"),
        get_complex_polygon("rand13")
    );
    assert!(r.is_ok());
}
#[test]
fn eighth_random_fiasco_test() {
    // ALL: #0[1, 4], #1[1, 5], #2[2, 0], #3[2, 2], #4[2, 3], #5[3, 0], #6[3, 1], #7[4, 1], #8[5, 6]
    let r = Snipper::union(
        get_complex_polygon("rand14"),
        get_complex_polygon("rand15")
    );
    assert!(r.is_ok());
}
#[test]
fn ninth_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand14"),
        get_complex_polygon("rand15_modified")
    );
    assert!(r.is_ok());
}
#[test]
fn tenth_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand16"),
        get_complex_polygon("rand17")
    );
    assert!(r.is_ok(), "{:?}", r.err());
}
#[test]
fn eleventh_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand18"),
        get_complex_polygon("rand19")
    );
    assert!(r.is_ok());
}
#[test]
fn twelfth_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand20"),
        get_complex_polygon("rand21")
    );
    assert!(r.is_ok());
}
#[test]
fn thirteenth_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand22"),
        get_complex_polygon("rand23")
    );
    assert!(r.is_ok());
}
#[test]
fn fourteenth_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand24"),
        get_complex_polygon("rand25")
    );
    assert!(r.is_ok());
}
#[test]
fn fifteenth_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand26"),
        get_complex_polygon("rand27")
    );
    assert!(r.is_ok());
}
#[test]
fn sixteenth_random_fiasco_test() {
    // Intersection moves a ray down crossing another
    let r = Snipper::union(
        get_complex_polygon("rand28"),
        get_complex_polygon("rand29")
    );
    assert!(r.is_ok());
}
#[test]
fn seventeenth_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand30"),
        get_complex_polygon("rand31")
    );
    assert!(r.is_ok());
}
#[test]
fn eighteenth_random_fiasco_test() {
    // edge [0, 13] -> [6, 4] is intersected twice and attracted once
    // while its tip travels distance of 1.468 which is too much
    let r = Snipper::union(
        get_complex_polygon("rand32"),
        get_complex_polygon("rand33")
    );
    assert!(r.is_ok());
}
#[test]
fn nineteenth_random_fiasco_test() {
    // traverse not expected
    let r = Snipper::union(
        get_complex_polygon("rand34"),
        get_complex_polygon("rand35")
    );
    assert!(r.is_ok());
}

#[test]
fn twentieth_random_fiasco_test() {
    // trivial: switched x and y
    let r = Snipper::union(
        get_complex_polygon("rand36"),
        get_complex_polygon("rand37")
    );
    assert!(r.is_ok());
}

#[test]
fn twenty_first_random_fiasco_test() {
    // failed sweep -- the edge to be swept is on same support
    // as the snipped edge; may also be failed dirty
    let r = Snipper::union(
        get_complex_polygon("rand38"),
        get_complex_polygon("rand39")
    );
    assert!(r.is_ok());
}
#[test]
fn twenty_second_random_fiasco_test() {
    // borderline intersection problem --
    // two intersection points of non overlapping segments,
    // also, constraint should be tighter
    let r = Snipper::union(
        get_complex_polygon("rand40"),
        get_complex_polygon("rand41")
    );
    assert!(r.is_ok());
}

#[test]
fn twenty_third_random_fiasco_test() {
    // missing sweep -- max actually not used
    // when calculating sweep zone
    let r = Snipper::union(
        get_complex_polygon("rand42"),
        get_complex_polygon("rand43")
    );
    assert!(r.is_ok());
}

#[test]
fn twenty_fourth_random_fiasco_test() {
    // We actually need to restrict intersections
    // (particularly with near verticals)
    let r = Snipper::union(
        get_complex_polygon("rand44"),
        get_complex_polygon("rand45")
    );
    assert!(r.is_ok());
}

#[test]
fn twenty_fifth_random_fiasco_test() {
    // Ray wasn't emptied in wipe obstacles but traverse was removed
    let r = Snipper::union(
        get_complex_polygon("rand46"),
        get_complex_polygon("rand47")
    );
    assert!(r.is_ok());
}

#[test]
fn twenty_sixth_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand48"),
        get_complex_polygon("rand49")
    );
    assert!(r.is_ok());
}

#[test]
fn twenty_seventh_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand50"),
        get_complex_polygon("rand51")
    );
    assert!(r.is_ok());
}

#[test]
fn twenty_eighth_random_fiasco_test() {
    // used point instead of preferred when wiping after intersection denied
    let r = Snipper::union(
        get_complex_polygon("rand52"),
        get_complex_polygon("rand53")
    );
    assert!(r.is_ok());
}

#[test]
fn twenty_ninth_random_fiasco_test() {
    // intersection must be allowed iff new position has sufficient allowance
    // ALL: #0[1, 5], #1[5, 15], #2[7, 6], #3[7, 22],
    // #4[7, 25], #5[8, 23], #6[8, 24], #7[9, 19],
    // #8[9, 20], #9[9, 23], #10[9, 27], #11[10, 30], #12[19, 14]
    let r = Snipper::union(
        get_complex_polygon("rand54"),
        get_complex_polygon("rand55")
    );
    assert!(r.is_ok());
}

#[test]
fn thirtieth_random_fiasco_test() {
    // allowance safety was actually negative
    let r = Snipper::union(
        get_complex_polygon("rand56"),
        get_complex_polygon("rand57")
    );
    assert!(r.is_ok());
}

#[test]
fn thirty_first_random_fiasco_test() {
    let r = Snipper::union(
        get_complex_polygon("rand58"),
        get_complex_polygon("rand59")
    );
    assert!(r.is_ok());
}

#[test]
fn thirty_second_random_fiasco_test() {
    // neighbor set as upper dirty and then wiped away
    let r = Snipper::union(
        get_complex_polygon("rand60"),
        get_complex_polygon("rand61")
    );
    assert!(r.is_ok());
}

#[test]
fn thirty_third_random_fiasco_test() {
    // bad implementation of wipe in intersect
    let r = Snipper::union(
        get_complex_polygon("rand62"),
        get_complex_polygon("rand63")
    );
    assert!(r.is_ok());
}

#[test]
fn thirty_fourth_random_fiasco_test() {
    // traverses become parallel and are discarded
    // ALL: #0[1, 0], #1[2, 3], #2[5, 2], #3[6, 2], #4[6, 3], #5[7, 3]
    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("ss01"));
    operands.push(unsafe { Polygon::trivial(Path::new(&vec![])) });

    let exp = vec![vec![1, 2, 4]];
    test_operation(operands, &Operation::XOR, &exp)
}

#[test]
fn thirty_fifth_random_fiasco_test() {
    // traverses become parallel and are discarded
    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("ss02"));
    operands.push(get_complex_polygon("ss03"));
    let exp = vec![
        vec![1, 4, 2],
        vec![3, 5, 6]
    ];
    test_operation(operands, &Operation::UNION, &exp)
}

#[test]
fn thirty_sixth_random_fiasco_test() {
    // edge is snipped and should be reinserted,
    // but if only angle is tested, the change
    // is not detected - test crossing too
    let a = get_complex_polygon("ss04");
    let b = unsafe { Polygon::trivial(Path::new(&vec![])) };

    let r = Snipper::xor(a, b);
    assert!(r.is_ok());
}

#[test]
fn thirty_seventh_random_fiasco_test() {
    // An intersection moves edge across the path of a builder,
    // builder is not found when edge is examined so a new builder
    // is created. The old builder is terminated later on so created
    // and terminated builders are not paired
    // Possible solutions:
    // 1) Builder loose points may be stored in a hash map so that
    // suitable candidate is found in constant time
    // 2) Created and terminated builders may be reattached in the connect method.
    // This seems to be simpler but the builder to reattach would have to be carefully
    // selected so that the correct order of builders is maintained
    let a = get_complex_polygon("ss05");
    let b = unsafe { Polygon::trivial(Path::new(&vec![])) };
    let r = Snipper::xor(a, b);
    assert!(r.is_ok(), "{:?}", r.err());
}

#[test]
fn thirty_eighth_random_fiasco_test() {
    // An intersection moves edge into the path of a builder
    // while crossing with the scope is equal for both.
    // This was disallowed before, now we simply consider
    // the builder terminated. More robust solution would
    // compare pseudoangles of the edge and last projection
    // of the builder and decide whether edge is over or under the latter
    let a = get_complex_polygon("ss06");
    let b = unsafe { Polygon::trivial(Path::new(&vec![])) };
    let r = Snipper::xor(a, b);
    assert!(r.is_ok());
}

#[test]
fn thirty_ninth_random_fiasco_test() {
    // An out-of-scope chain is reused (at 13/20)
    let a = get_complex_polygon("ss07");
    let b = unsafe { Polygon::trivial(Path::new(&vec![])) };
    let r = Snipper::xor(a, b);
    assert!(r.is_ok());
}

#[test]
fn fortieth_random_fiasco_test() {
    // Intersection moves two edges but only
    // one crosses the newly created support
    // so if we perform connect in between
    // loops, created builders do not match
    let a = get_complex_polygon("ss08");
    let b = unsafe { Polygon::trivial(Path::new(&vec![])) };
    let r = Snipper::xor(a, b);
    assert!(r.is_ok(), "{:?}", r.err());
}
#[test]
fn forty_first_random_fiasco_test() {
    // Builder is terminated twice when iteration
    // restarts just after newly created builder
    // ALL:
    // #0[1, 5], #1[7, 6], #2[7, 25], #3[8, 23],
    // #4[8, 24], #5[9, 19], #6[9, 20], #7[9, 23],
    // #8[9, 27], #9[10, 30], #10[19, 14]

    let a = get_complex_polygon("rand54r");
    let b = get_complex_polygon("rand55r");

    let r = Snipper::union(a, b);
    assert!(r.is_ok(), "{:?}", r.err());
}
#[test]
fn forty_second_random_fiasco_test() {
    // When builder follows chain, the search_start information is lost
    let a = get_complex_polygon("ss09");
    let b = unsafe { Polygon::trivial(Path::new(&vec![])) };
    let r = Snipper::xor(a, b);
    assert!(r.is_ok(), "{:?}", r.err());
}
#[test]
fn forty_third_random_fiasco_test() {
    // In FollowChain mode, builder is not written off as terminated
    // if the vertical edge is the last edge in scope
    let a = get_complex_polygon("ss10");
    let b = unsafe { Polygon::trivial(Path::new(&vec![])) };
    let r = Snipper::xor(a, b);
    assert!(r.is_ok(), "{:?}", r.err());
}

#[test]
fn normalize_fiasco_test() {
    // ALL:
    // #0[6, 14], #1[9, 17], #2[11, 16], #3[13, 7], #4[13, 15],
    // #5[17, 10], #6[22, 13], #7[25, 17], #8[25, 18], #9[27, 12], #10[30, 8]

    let poly0 = get_complex_polygon("norm0p0");
    let poly1 = get_complex_polygon("norm0p1");
    let exp = vec![
        vec![0, 10, 6, 4, 2, 7, 6, 9, 10, 8, 1],
        vec![3, 10, 5],
    ];
    test_operation(vec![poly0, poly1], &Operation::XOR, &exp);


}
#[test]
fn null_edges_perform1_test() {
    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("big"));
    operands.push(get_complex_polygon("vane_ns"));
    // ALL:
    // #0[-6, -4], #1[-6, -2], #2[-5, 1],
    // #3[-4, -2], #4[-3, -2], #5[-2, -2],
    // #6[-1, -5], #7[0, -2], #8[0, 0],
    // #9[1, 5], #10[5, -1]

    let exp = vec![
        vec![0, 3, 1],
        vec![2, 4, 6, 10, 9],
    ];
    test_operation(operands, &Operation::UNION, &exp);
}

#[test]
fn giant_polygons_perform_test() {
    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("giant_square"));
    operands.push(get_complex_polygon("giant_polygon"));
    // ALL:
    // #0[-16777216, -16777216],
    // #1[-16777216, 16777214],
    // #2[-16777215, -16777216],
    // #3[-16777215, -16777215],
    // #4[-16777215, 16777214],
    // #5[-16777215, 16777215],
    // #6[-8388607, 16777215],
    // #7[0, -16777215],
    // #8[0, 16777216],
    // #9[8388607, 16777215],
    // #10[16777214, -16777214],
    // #11[16777214, 16777214],
    // #12[16777215, -16777215],
    // #13[16777215, 16777215]

    let exp = vec![
        vec![0, 2, 7, 12, 13, 9, 8, 6, 5, 4, 1],
        vec![4, 6, 9, 11, 10, 7, 3]
    ];
    test_operation(operands, &Operation::XOR, &exp);
}
#[test]
fn neighbour_polygons_perform_test() {
    let rect_s0_0 = Path::new(&vec![
        Point::new(0, 0).expect("!"),
        Point::new(10, 0).expect("!"),
        Point::new(10, 20).expect("!"),
        Point::new(0, 20).expect("!")
    ]);
    let rect_s1_2 = Path::new(&vec![
        Point::new(30, 0).expect("!"),
        Point::new(40, 0).expect("!"),
        Point::new(40, 30).expect("!"),
        Point::new(30, 30).expect("!")
    ]);
    let rect_s2_4 = Path::new(&vec![
        Point::new(10, 20).expect("!"),
        Point::new(20, 20).expect("!"),
        Point::new(20, 30).expect("!"),
        Point::new(10, 30).expect("!")
    ]);

    let rect_c0_1 = Path::new(&vec![
        Point::new(10, 0).expect("!"),
        Point::new(30, 0).expect("!"),
        Point::new(30, 20).expect("!"),
        Point::new(10, 20).expect("!")
    ]);


    let rect_c1_3 = Path::new(&vec![
        Point::new(0, 20).expect("!"),
        Point::new(10, 20).expect("!"),
        Point::new(10, 50).expect("!"),
        Point::new(0, 50).expect("!")
    ]);

    let rect_c2_5 = Path::new(&vec![
        Point::new(10, 30).expect("!"),
        Point::new(40, 30).expect("!"),
        Point::new(40, 50).expect("!"),
        Point::new(10, 50).expect("!")
    ]);
    let s = unsafe { Polygon::flat(vec![rect_s0_0, rect_s1_2, rect_s2_4]).unwrap() };
    let c = unsafe { Polygon::flat(vec![rect_c0_1, rect_c1_3, rect_c2_5]).unwrap() };
    // ALL: #0[0, 0], #1[0, 20], #2[0, 50], #3[10, 0],
    // #4[10, 20], #5[10, 30], #6[10, 50], #7[20, 20],
    // #8[20, 30], #9[30, 0], #10[30, 20], #11[30, 30],
    // #12[40, 0], #13[40, 30], #14[40, 50]

    let operands = vec![s, c];

    let exp = vec![
        vec![0, 3, 9, 12, 13, 14, 6, 2, 1],
        vec![8, 11, 10, 7]
    ];
    test_operation(operands.iter().cloned().collect(), &Operation::UNION, &exp);
    let exp = vec![];
    test_operation(operands.iter().cloned().collect(), &Operation::INTERSECTION, &exp);
}

#[test]
fn xor_perform_test() {
    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("invalid_bow"));
    operands.push(unsafe { Polygon::trivial(Path::new(&vec![])) });
    // ALL: #0[-10, -10], #1[-10, 10], #2[0, 0], #3[10, -10], #4[10, 10]
    let exp = vec![vec![0, 2, 3, 4, 2, 1]];
    test_operation(operands, &Operation::XOR, &exp);

    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("star"));
    operands.push(unsafe { Polygon::trivial(Path::new(&vec![])) });
    // ALL:
    // #0[0, -10], #1[0, 0], #2[4, -8], #3[5, -10], #4[10, -20],
    // #5[10, -5], #6[15, -10], #7[16, -8], #8[20, -10], #9[20, 0]
    let exp = vec![vec![0, 3, 4, 6, 8, 7, 6, 3, 2], vec![1, 2, 5, 7, 9, 5]];
    test_operation(operands, &Operation::XOR, &exp);

    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("star"));
    operands.push(get_complex_polygon("invalid_bow"));
    // ALL:
    // #0[-10, -10], #1[-10, 10], #2[0, -10], #3[0, 0],
    // #4[4, -8], #5[5, -10], #6[7, -7], #7[10, -20],
    // #8[10, -10], #9[10, -5], #10[10, 10], #11[15, -10],
    // #12[16, -8], #13[20, -10], #14[20, 0]
    let exp = vec![
        vec![0, 3, 4, 6, 8, 9, 12, 14, 9, 6, 3, 1],
        vec![2, 5, 7, 11, 13, 12, 11, 8, 5, 4],
        vec![3, 9, 10]];
    test_operation(operands, &Operation::XOR, &exp);

}
#[test]
fn combination_perform_test() {

    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("big"));
    operands.push(get_complex_polygon("cross"));
    // ALL: #0[-6, -4], #1[-5, 1], #2[-3, -2], #3[-2, -10], #4[-1, -5], #5[0, 0], #6[1, 5], #7[2, -3], #8[4, -6], #9[5, -1]
    let exp = vec![
        vec![0, 3, 8, 7, 4, 2],
        vec![1, 2, 5, 7, 9, 6]
    ];
    test_operation(operands, &Operation::XOR, &exp);

    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("bow"));
    operands.push(get_complex_polygon("aligned"));
    // ALL: #0[-2, 3], #1[0, -3], #2[0, 0], #3[0, 2], #4[0, 3], #5[2, -3], #6[8, -3], #7[8, 2]
    let exp = vec![
        vec![0, 2, 5, 6, 7, 3, 4]
    ];
    test_operation(operands, &Operation::XOR, &exp);

    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("bow"));
    operands.push(get_complex_polygon("cross"));
    // ALL: #0[-6, -4], #1[-2, -10], #2[-2, 3], #3[0, -3], #4[0, 0], #5[0, 3], #6[2, -3], #7[4, -6]
    let exp = vec![
        vec![0, 1, 7, 6, 3, 4],
        vec![2, 4, 5]
    ];
    test_operation(operands, &Operation::XOR, &exp);
}
#[test]
fn perform_operation_test() {
    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("cross"));
    operands.push(get_complex_polygon("big"));
//        let ee = GraphBuilder::build(operands.iter().cloned().collect()).unwrap();
//        all_graph_points(ee);
    // ALL: #0[-6, -4], #1[-5, 1], #2[-3, -2],
    // #3[-2, -10], #4[-1, -5], #5[0, 0], #6[1, 5],
    // #7[2, -3], #8[4, -6], #9[5, -1]

    let exp = vec![vec![0, 3, 8, 7, 9, 6, 1, 2]];
    test_operation(operands.iter().cloned().collect(), &Operation::UNION, &exp);

    let exp = vec![vec![2, 4, 7, 5]];
    test_operation(operands.iter().cloned().collect(), &Operation::INTERSECTION, &exp);

    let exp = vec![
        vec![0, 3, 8, 7, 4, 2]
    ];
    test_operation(operands.iter().cloned().collect(), &Operation::DIFFERENCE, &exp);

    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("touch"));
    operands.push(get_complex_polygon("big"));

    // ALL: #0[-5, 1], #1[-2, 3], #2[-1, -5], #3[0, 0], #4[1, 5], #5[3, 2], #6[5, -1]
    let exp = vec![vec![0, 2, 6, 5, 4, 1]];
    test_operation(operands.iter().cloned().collect(), &Operation::UNION, &exp);
    let exp = vec![vec![1, 3, 5, 4]];
    test_operation(operands.iter().cloned().collect(), &Operation::INTERSECTION, &exp);
    let exp = vec![];
    test_operation(operands.iter().cloned().collect(), &Operation::DIFFERENCE, &exp);

    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("aligned"));
    operands.push(get_complex_polygon("big"));
    // ALL:
    // #0[-5, 1], #1[-1, -5], #2[0, -3],
    // #3[0, 2], #4[1, 5], #5[2, -3],
    // #6[3, 2], #7[5, -1], #8[8, -3], #9[8, 2]
    let exp = vec![vec![0, 1, 5, 8, 9, 6, 4]];
    test_operation(operands.iter().cloned().collect(), &Operation::UNION, &exp);
    let exp = vec![vec![2, 5, 7, 6, 3]];
    test_operation(operands.iter().cloned().collect(), &Operation::INTERSECTION, &exp);
    let exp = vec![vec![5, 8, 9, 6, 7]];
    test_operation(operands.iter().cloned().collect(), &Operation::DIFFERENCE, &exp);

    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("bow"));
    operands.push(get_complex_polygon("cross"));
    // ALL:
    // #0[-6, -4], #1[-2, -10], #2[-2, 3], #3[0, -3],
    // #4[0, 0], #5[0, 3], #6[2, -3], #7[4, -6]
    let exp = vec![
        vec![0, 1, 7, 6, 4],
        vec![2, 4, 5]
    ];
    test_operation(operands.iter().cloned().collect(), &Operation::UNION, &exp);
    let exp = vec![vec![3, 6, 4]];
    test_operation(operands.iter().cloned().collect(), &Operation::INTERSECTION, &exp);
    let exp = vec![vec![2, 4, 5]];
    test_operation(operands.iter().cloned().collect(), &Operation::DIFFERENCE, &exp);

}
#[test]
fn perform_difference_test() {

    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("big"));
    operands.push(get_complex_polygon("bow"));
    // ALL: #0[-5, 1], #1[-2, 3], #2[-1, -5], #3[0, -3], #4[0, 0], #5[0, 3], #6[1, 5], #7[2, -3], #8[5, -1]
    let exp = vec![vec![0, 2, 7, 8, 6, 1, 5, 4, 1], vec![4, 7, 3]];
    test_operation(operands.iter().cloned().collect(), &Operation::DIFFERENCE, &exp);
}
#[test]
fn super_complex_a_test() {
    let a0 = get_supercomplex_polygon("a0");
    let a1 = get_supercomplex_polygon("a1");
    let result = Snipper::union(a0, a1);
    assert!(result.is_ok());
}
#[test]
fn super_complex_b_test() {
    let b0 = get_supercomplex_polygon("b0");
    let b1 = get_supercomplex_polygon("b1");
    let result = Snipper::union(b0, b1);
    assert!(result.is_ok());
}
#[test]
fn hole_test() {
    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("big"));
    operands.push(get_complex_polygon("bow"));
    // ALL: #0[-5, 1], #1[-2, 3], #2[-1, -5],
    // #3[0, -3], #4[0, 0], #5[0, 3],
    // #6[1, 5], #7[2, -3], #8[5, -1]
    let exp = vec![vec![0, 2, 7, 8, 6, 1, 5, 4, 1], vec![4, 7, 3]];
    test_operation(operands, &Operation::DIFFERENCE, &exp);
}

#[test]
fn touch_test() {
    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_complex_polygon("touch"));
    operands.push(get_complex_polygon("big"));
    // ALL: #0[-5, 1], #1[-2, 3], #2[-1, -5], #3[0, 0], #4[1, 5], #5[3, 2], #6[5, -1]
    let exp = vec![vec![0, 2, 6, 5, 4, 1]];
    test_operation(operands, &Operation::UNION, &exp);

}
#[test]
fn drawing_algorithm_end_to_end_test() {
    let mut operands: Vec<Polygon> = Vec::new();
    operands.push(get_supercomplex_polygon("daeet0"));
    operands.push(get_supercomplex_polygon("daeet1"));
    // ALL:
    // #0[0, 20], #1[0, 30], #2[0, 40], #3[10, 20],
    // #4[10, 60], #5[20, 0], #6[20, 10], #7[20, 30],
    // #8[20, 40], #9[20, 50], #10[20, 70], #11[30, 20],
    // #12[30, 60], #13[40, 20], #14[40, 30], #15[40, 40]
    let exp = vec![
        vec![0, 5, 13, 14, 15, 7, 1, 8, 15, 9, 2, 1],
        vec![7, 11, 6, 3],
        vec![4, 9, 12, 10],
    ];
    let structure = vec![None, Some(0), None];
    test_operation_and_structure(
        operands,
        &Operation::XOR,
        &exp,
        Some(&structure)
    );

}