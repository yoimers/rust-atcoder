


const M: i64 = 998244353;
#[derive(Debug, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct Mod(pub i64);
impl ::std::fmt::Display for Mod {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
impl Mod {
  pub fn new(v: i64) -> Mod {
    Mod((v + M) % M)
  }
  pub fn pow(self, r: i64) -> Mod {
    let mut r = r;
    let mut k = self;
    let mut ret = 1.into();
    while r > 0 {
      if r % 2 != 0 {
        ret = ret * k;
      }
      r /= 2;
      k = k * k;
    }
    ret
  }
  pub fn inv(self) -> Mod {
    self.pow(M - 2)
  }
}
use std::ops::*;
impl<T: Into<Mod>> Add<T> for Mod {
  type Output = Mod;
  fn add(self, rhs: T) -> Self::Output {
    Mod::new(self.0 + rhs.into().0)
  }
}
impl<T: Into<Mod>> AddAssign<T> for Mod {
  fn add_assign(&mut self, rhs: T) {
    *self = *self + rhs;
  }
}
impl<T: Into<Mod>> Sub<T> for Mod {
  type Output = Mod;
  fn sub(self, rhs: T) -> Self::Output {
    Mod::new(self.0 - rhs.into().0 + M)
  }
}
impl<T: Into<Mod>> SubAssign<T> for Mod {
  fn sub_assign(&mut self, rhs: T) {
    *self = *self - rhs;
  }
}
impl<T: Into<Mod>> Mul<T> for Mod {
  type Output = Mod;
  fn mul(self, rhs: T) -> Self::Output {
    Mod::new(self.0 * rhs.into().0)
  }
}
impl<T: Into<Mod>> MulAssign<T> for Mod {
  fn mul_assign(&mut self, rhs: T) {
    *self = *self * rhs;
  }
}
impl<T: Into<Mod>> Div<T> for Mod {
  type Output = Mod;
  fn div(self, rhs: T) -> Self::Output {
    self * rhs.into().inv()
  }
}
impl<T: Into<Mod>> DivAssign<T> for Mod {
  fn div_assign(&mut self, rhs: T) {
    *self = *self / rhs;
  }
}
impl Neg for Mod {
  type Output = Mod;
  fn neg(self) -> Self::Output {
    Mod(0) - self
  }
}
impl<T: std::convert::Into<i64>> std::convert::From<T> for Mod {
  fn from(v: T) -> Self {
    Mod::new(v.into())
  }
}
