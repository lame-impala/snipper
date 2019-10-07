pub fn approx_eq(lhs: f64, rhs: f64, e: f64) -> bool {
    ((lhs - rhs).abs() < e)
}