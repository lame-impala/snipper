use std::ops;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use super::integer_decode::integer_decode;


#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Pseudoangle {
    value: f64
}
impl Pseudoangle {
    const MOD: f64 = 4f64;
    pub const ONE: Pseudoangle = Pseudoangle{value: 1.0};
    pub const UP: Pseudoangle = Pseudoangle{value: 0.0};
    pub const DOWN: Pseudoangle = Pseudoangle{value: 2.0};
    // for use in ranges
    pub const STOP: Pseudoangle = Pseudoangle{value: 4.0};
    pub fn new(value: f64) -> Pseudoangle {
        if value.is_nan() { panic!("NaN value not allowed for Pseudoangle"); }
        let trimmed = if value < 0f64 {
            value % Pseudoangle::MOD + Pseudoangle::MOD
        } else {
            value % Pseudoangle::MOD
        };
        return Pseudoangle{value: trimmed};
    }
    pub fn to_float (&self) -> f64 {
        return self.value
    }
    pub fn display_value(&self) -> String {
        return format!("{:.2}", self.value);
    }
    pub fn integer_decode(&self) -> (u64, i16, i8){
        if self.value == 0f64 {
            (0u64, 0i16, 0i8)
        } else {
            integer_decode(self.value)
        }

    }
    pub fn reverse(&self) -> Pseudoangle {
        self + &Pseudoangle::new(2.0)
    }
}

impl Hash for Pseudoangle {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let (m, e, s) = self.integer_decode();
        state.write_u64(m);
        state.write_i16(e);
        state.write_i8(s);
    }
}

impl Default for Pseudoangle {
    fn default() -> Pseudoangle {
        return Pseudoangle{value: 0f64}
    }
}
impl<'a, 'b> ops::Add<&'b Pseudoangle> for &'a Pseudoangle{
    type Output = Pseudoangle;
    fn add (self, rhs: &'b Pseudoangle) -> Pseudoangle {
        let value = self.value + rhs.value;
        Pseudoangle::new(value)
    }
}
impl<'a, 'b> ops::Sub<&'b Pseudoangle> for &'a Pseudoangle{
    type Output = Pseudoangle;
    fn sub (self, rhs: &'b Pseudoangle) -> Pseudoangle {
        let value = self.value - rhs.value;
        Pseudoangle::new(value)
    }
}
impl Eq for Pseudoangle {}
impl Ord for Pseudoangle {
    fn cmp(&self, other: &Pseudoangle) -> Ordering {
        if self.value < other.value {
            Ordering::Less
        } else if self.value > other.value {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
#[test]
fn pseudoangle_test() {
    let pseudo1p = Pseudoangle::new(1.0);
    let pseudo2p = Pseudoangle::new(2.0);
    assert_eq!(pseudo1p.to_float(), 1.0);
    assert_eq!(&pseudo1p + &pseudo2p, Pseudoangle::new(3.0));
    assert_eq!(&pseudo1p - &pseudo2p, Pseudoangle::new(3.0));
    assert_eq!(&pseudo2p - &pseudo1p, Pseudoangle::new(1.0));
    assert!(&pseudo1p < &pseudo2p);
    assert!(&pseudo2p > &pseudo1p);
}
#[test]
fn integer_decode_test() {
    let pseudo0 = Pseudoangle::new(0f64);
    let pseudo_0 = Pseudoangle::new(-0f64);
    let pseudo_1 = Pseudoangle::new(1f64);
    let pseudo_1dot1 = Pseudoangle::new(1.1f64);
    let pseudo_2 = Pseudoangle::new(2.0);

    assert_eq!(pseudo0.integer_decode(), (0, 0, 0));
    assert_eq!(pseudo_0.integer_decode(), (0, 0, 0));
    assert_eq!(pseudo_1.integer_decode(), (4503599627370496, -52, 1));
    assert_eq!(pseudo_1dot1.integer_decode(), (4953959590107546, -52, 1));
    assert_eq!(pseudo_2.integer_decode(), (4503599627370496, -51, 1));
}
