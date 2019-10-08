# snipper
### A polygon clipping library in Rust
Snipper performs common boolean operations with polygons: union, intersection, xor, difference. There are no assumptions about how the polygon should be formed: complex polygons with holes or self-intersection polygons will do. It differs from similar libraries in that it only uses integer coordinates in the API as well as internally. Algorithm used here is the classic Bentley-Ottmann modified to work with integer coordinates. 

#### Performance
Compared with similar Rust library [rust-geo-booleanop](https://github.com/21re/rust-geo-booleanop), this one comes out slow. In benchmarks using from 10 to 10000 edges, Snipper is increasingly slower by factor of 4 to 10. The troubling part is that it means asymptotic complexity of the implementation is not quite right. This is most probably due to the fact that BTreeMap is used internally to implement scope. As scope is recreated at each stop, this adds some complexity over the inherent complexity of Bentley-Ottmann algorithm. Some future version may address this problem.

#### Purpose
The library evolved from what was originally an educational project and its performance at the current stage is not on par with existing professional libraries. Nevertheless I consider it to be an interesting catalogue of Rust specific solutions and techniques that may be inspirational for some users.

#### Usage
Coordinates are limited to range from -2<sup>24</sup> to 2<sup>24</sup>. This is why point constructor returns Result and needs to be unwrapped:

`let point = Point::new(5, 10).unwrap();`

Paths may be created from a vector of points, or incrementally using a builder:

`let path = Path::new(&vec![p0, p1, p2]);`

or

```
let mut builder = PathBuilder::new();
builder.add(&p0);
builder.add(&p1);
builder.add(&p2);
let path = builder.build();
```

All paths are considered closed.

Polygon is created either from a path or a vector of paths. A polygon created this way may not meet prerequisites of a normal polygon (no self-intersections). This fact will not prevent Snipper to handle it correctly, but some convenience methods may not work as expected. To remind the user of this, polygon constructors are marked as unsafe:

```
let polygon = unsafe { Polygon::trivial(path) };
let polygon = unsafe { Polygon::flat(vec![path0, path1]) };
```

A safe way to create polygon is to normalize a vector of paths. This method returns a Solution object that may be used to simply retrieve a vector of paths or to build a correct polygon:

```
let solution = Snipper::normalize(vec![path0, path1, path2]).unwrap();
let polygon = solution.polygon().unwrap();
```

