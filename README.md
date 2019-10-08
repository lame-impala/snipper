# Snipper
### A polygon clipping library in Rust
Snipper performs common boolean operations with polygons: union, intersection, xor, difference. There are no restriction on how the polygons passed in as operands should be formed: complex polygons with holes or self-intersecting polygons will do. It differs from similar libraries in that it only uses integer coordinates in the API as well as internally. Algorithm used here is the classic Bentley-Ottmann modified to work with integer coordinates. 

#### Performance
Compared to a similar Rust library [rust-geo-booleanop](https://github.com/21re/rust-geo-booleanop) in benchmarks using different numbers of edges on input this library turns out slower. Starting with 10 edges, Snipper falls behind by a factor of 4, and it is about 10 times slower with 10000 edges. This means asymptotic complexity of the implementation is not quite right. Most probably this is to a great extent due to the fact that BTreeMap is used internally to implement scope. As scope is recreated at each stop, this adds overhead to the inherent complexity of Bentley-Ottmann algorithm. Some future version may address this problem.

#### Purpose
The library evolved from what was originally an educational project and its performance at the current stage is not on par with existing professional libraries. Nevertheless even at this stage of development it makes for an interesting catalogue of Rust specific solutions and techniques that may be inspirational for some users.

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

Polygon is created either from a path or a vector of paths. A polygon created this way may not meet prerequisites for a normal polygon (no self-intersections). This fact will not prevent Snipper from handling it just fine, but some convenience methods may not work as expected. To remind the user of this, polygon constructors are marked as unsafe:

```
let polygon = unsafe { Polygon::trivial(path) };
let polygon = unsafe { Polygon::flat(vec![path0, path1]) };
```

A safe way to create polygon is to normalize a vector of paths. This method returns a Solution object that may be used to simply retrieve a vector of paths or to build a well-formed polygon:

```
let solution = Snipper::normalize(vec![path0, path1, path2]).unwrap();
let paths = solution.paths(); // or let polygon = solution.polygon().unwrap();
```

Once polygons are created, the usage is quite straightforward: 

```
let solution = Snipper::xor(poly0, poly1).unwrap();
let polygon = solution.polygon().unwrap();
```

Again, the result of the operation is Result<Solution, Error>. Client code may just pull a vector of paths out of the solution, or have a new polygon built, which comes with a time penalty (significant with very complex polygons) but makes it possible to use polygon convenience methods:

```
let bounds = polygon.bounds().unwrap();
let area = polygon.area();
let centroid = polygon.centroid();
let position = polygon.position(&point);
```
 
#### License
This project is licensed under the terms of the MIT license.

