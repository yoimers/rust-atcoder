#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Rational {
  numerator: i64,
  denominator: i64,
}

impl Rational {
  pub fn new(numerator: i64, denominator: i64) -> Self {
    assert_ne!(denominator, 0);
    let sign = if denominator.is_negative() { -1 } else { 1 };
    let g = Self::gcd(numerator.abs(), denominator.abs());
    Self {
      numerator: sign * numerator / g,
      denominator: sign * denominator / g,
    }
  }
  fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
      a
    } else {
      Self::gcd(b, a % b)
    }
  }
}

impl std::ops::Add for Rational {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    let numerator = self.numerator * rhs.denominator + rhs.numerator * self.denominator;
    let denominator = self.denominator * rhs.denominator;
    Self::new(numerator, denominator)
  }
}

use std::ops::Add;
impl std::ops::AddAssign for Rational {
  fn add_assign(&mut self, rhs: Self) {
    *self = self.add(rhs);
  }
}
impl std::ops::Sub for Rational {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    let numerator = self.numerator * rhs.denominator - rhs.numerator * self.denominator;
    let denominator = self.denominator * rhs.denominator;
    Self::new(numerator, denominator)
  }
}

use std::ops::Sub;
impl std::ops::SubAssign for Rational {
  fn sub_assign(&mut self, rhs: Self) {
    *self = self.sub(rhs);
  }
}

impl std::ops::Mul for Rational {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    let numerator = self.numerator * rhs.numerator;
    let denominator = self.denominator * rhs.denominator;
    Self::new(numerator, denominator)
  }
}

impl Ord for Rational {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    let l = self.numerator * other.denominator;
    let r = other.numerator * self.denominator;
    l.cmp(&r)
  }
}
impl PartialOrd for Rational {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

#[test]
fn ord_test() {
  let a = Rational::new(1, 2);
  let b = Rational::new(1, 4);
  assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater);
  let b = Rational::new(2, 4);
  assert_eq!(a.cmp(&b), std::cmp::Ordering::Equal);
  let b = Rational::new(-1, 4);
  assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater);
  let a = Rational::new(-1, 2);
  assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
  let a = Rational::new(-0, 2);
  let b = Rational::new(0, 4);
  assert_eq!(a.cmp(&b), std::cmp::Ordering::Equal);
}
