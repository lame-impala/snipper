#![feature(test)]
#[macro_use]
extern crate criterion;
extern crate rand;
#[macro_use]
extern crate lazy_static;
use criterion::{Criterion, Benchmark};
use rand::Rng;

extern crate snipper;

use snipper::{Snipper, Point, Polygon, Path, Bounds, Coordinate, IntersectionAlgorithm};
use std::collections::HashMap;

static NUM_POINTS: usize = 4;
static BUG_HUNT: (usize, i32) = (16, 32);

fn random_polygons() -> HashMap<usize, Vec<Point>> {
    let map: HashMap<usize, Vec<Point>> = (0usize..1000usize).map(|index| {
        let points: Vec<Point> = (0..NUM_POINTS).map(|_| {
            let mut rng = rand::thread_rng();
            let x: i32 = rng.gen_range(0, 32);
            let y: i32 = rng.gen_range(0, 32);
            Point::new(x, y).unwrap()
        }).collect();
        (index, points)
    }).collect();
    map
}

lazy_static! {
    pub static ref RANDOM_POLYGONS: HashMap<usize, Vec<Point>> = random_polygons();
}
fn criterion_benchmark(c: &mut Criterion) {
//    c.bench(
//        "bug hunt",
//        Benchmark::new(
//            "bug hunt", |b| b.iter(|| bug_hunt()))
//            .sample_size(4000)
//    );
    c.bench_function("random triangles 10", |b| {
        let (p0, p1) = random_triangles_10();
        b.iter(|| {
//            let _ = BentleyOttmann::perform(p0.clone(), p1.clone()).unwrap();
            perform_benchmark(p0.clone(), p1.clone());
        })
    });
    c.bench_function("random triangles 100", |b| {
        let (p0, p1) = random_triangles_100();
        b.iter(|| {
            perform_benchmark(p0.clone(), p1.clone());
        })
    });
    c.bench(
        "333 triangles",
        Benchmark::new(
            "random triangles 1000", |b| {
                let (p0, p1) = random_triangles_1000();
                b.iter(|| {
                    perform_benchmark(p0.clone(), p1.clone());
                })
            })
            .sample_size(10)
    );
    c.bench(
        "3333 triangles",
        Benchmark::new(
            "random triangles 10000", |b| {
                let (p0, p1) = random_triangles_10000();
                b.iter(|| {
                    perform_benchmark(p0.clone(), p1.clone());
                })
            }).sample_size(10)
    );
}
fn bug_hunt() {
    fn random_polygon() -> Polygon {
        let num = BUG_HUNT.0;
        let side = BUG_HUNT.1;
        let points: Vec<Point> = (0..num).map(|_| {
            let mut rng = rand::thread_rng();
            let x: i32 = rng.gen_range(0, side);
            let y: i32 = rng.gen_range(0, side);
            Point::new(x, y).unwrap()
        }).collect();
        let path = Path::new(&points);
        let result = std::panic::catch_unwind(|| {
            Snipper::normalize(vec![path.clone()]).unwrap()
        });
        match result {
            Err(error) => {
                println!("---------------------------------------------------");
                println!("PATH: {}", path.inspect());
                println!("---------------------------------------------------");
                panic!("Error: {:?}", error);
            },
            Ok(solution) => solution.polygon().unwrap()
        }
    }
    fn beat() {
        let mut rng = rand::thread_rng();
        let x: i32 = rng.gen_range(0, 1000);
        if x == 0 {
            let id = rng.gen_range(0, 1000);
            println!("--- ALIVE {} ---", id);
        }
    }
    let first = random_polygon();
    let second = random_polygon();
    beat();
    let result = std::panic::catch_unwind (|| {
        Snipper::union(first.clone(), second.clone()).expect("Fatal error")
    });
    match result {
        Err(error) => {
            println!("---------------------------------------------------");
            println!("FIRST: {}", first.inspect());
            println!("SECOND: {}", second.inspect());
            println!("---------------------------------------------------");
            println!("Error: {:?}", error);
        },
        Ok(_) => ()
    }
}
fn perform_benchmark(s: Polygon, p: Polygon) {
//    let _ = IntersectionAlgorithm::perform(s, p).unwrap();
    let _ = Snipper::xor(s, p).unwrap();
//    let _ = Snipper::xor(s, p).unwrap().polygon().unwrap();
}
fn random_triangles_10() -> (Polygon, Polygon) {
    random_combinations(1, 2, 1, 1)
}
fn random_triangles_100() -> (Polygon, Polygon) {
    random_combinations(2, 11, 1, 11)
}
fn random_triangles_1000() -> (Polygon, Polygon) {
    random_combinations(3, 74, 3, 37)
}
fn random_triangles_10000() -> (Polygon, Polygon) {
    random_combinations(9, 740, 9, 370)
}
fn random_combinations(av: usize, ah: usize, bv: usize, bh: usize) -> (Polygon, Polygon) {
    let a = random_triangles(av, ah);
    let b = random_triangles(bv, bh);
    (a, b)
}
fn random_triangles(slots_v: usize, slots_h: usize) -> Polygon {
    let mut vec = Vec::new();
    for slot_v in 0..slots_v {
        for slot_h in 0..slots_h {
            let bounds = get_slot(slot_v, slot_h, slots_v, slots_h);
            vec.push(random_triangle(bounds));
        }
    }
    unsafe {
        Polygon::flat(vec).unwrap()
    }
}
fn get_slot(slot_v: usize, slot_h: usize, total_v: usize, total_h: usize) -> Bounds {
    let max = i32::from(Coordinate::MAX);
    let min = i32::from(Coordinate::MIN);

    let span = max - min;

    let span_v = span / (total_v as i32);
    let span_h = span / (total_h as i32);

    let top = min + ((slot_v as i32) * span_v) + 1;
    let left = min + ((slot_h as i32) * span_h) + 1;
    let bottom = (top + span_v) - 1;
    let right = (left + span_h) - 1;

    Bounds::new(top, left, bottom, right)
}
fn random_triangle(
    bounds: Bounds
) -> Path {
    let mut points: Vec<Point> = Vec::new();
    for _ in 0..3 {
        let point = random_point(&bounds);
        points.push(point);
    }
    Path::new(&points)
}
fn random_point(bounds: &Bounds) -> Point {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(bounds.left().to_int(), bounds.right().to_int());
    let y = rng.gen_range(bounds.top().to_int(), bounds.bottom().to_int());
    Point::new(x, y).unwrap()
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
