use cargo_snippet::snippet;

use std::cmp::Ordering;
#[snippet("BinarySearch")]
pub trait BinarySearch<T> {
  fn lower_bound(&self, x: &T) -> usize;
  fn upper_bound(&self, x: &T) -> usize;
}

#[snippet("BinarySearch")]
impl<T: Ord> BinarySearch<T> for [T] {
  fn lower_bound(&self, x: &T) -> usize {
    let mut low = 0;
    let mut high = self.len();

    while low != high {
      let mid = (low + high) / 2;
      match self[mid].cmp(x) {
        Ordering::Less => low = mid + 1,
        Ordering::Equal | Ordering::Greater => high = mid,
      }
    }
    low
  }
  fn upper_bound(&self, x: &T) -> usize {
    let mut low = 0;
    let mut high = self.len();

    while low != high {
      let mid = (low + high) / 2;
      match self[mid].cmp(x) {
        Ordering::Less | Ordering::Equal => low = mid + 1,
        Ordering::Greater => high = mid,
      }
    }
    low
  }
}

use std::cmp::Ordering;

#[snippet("BinarySearch")]
struct BinarySearch<T, F> {
  lb: T,
  ub: T,
  f: F,
}

#[snippet("BinarySearch")]
impl<F: FnMut(f64) -> bool> BinarySearch<f64, F> {
  const eps: f64 = 1e-9;
  fn new(lb: f64, ub: f64, f: F) -> BinarySearch<f64, F> {
    BinarySearch { lb, ub, f }
  }
  fn search(&mut self) -> f64 {
    let mut lb = self.lb;
    let mut ub = self.ub;
    while ub > lb + Self::eps {
      let mid = (lb + ub) / 2.;
      if (self.f)(mid) {
        ub = mid;
      } else {
        lb = mid;
      }
    }
    lb
  }
}
#[snippet("BinarySearch")]
impl<F: FnMut(i64) -> bool> BinarySearch<i64, F> {
  #[doc = "O(log(upper-lower))"]
  pub fn lower_bound(&mut self) -> i64 {
    assert!(self.lb <= self.ub);
    let mut l = self.lb;
    let mut r = self.ub;
    while r > l {
      let mid = (l + r) / 2;
      let ok = (self.f)(mid);
      if ok {
        r = mid;
      } else {
        l = mid + 1;
      }
    }
    l
  }
}

#[test]
fn test_binary_search() {
  let xs = vec![1, 2, 2, 2, 2, 2, 3, 4, 5];
  let p0 = |i: i64| xs[i as usize] >= 2;
  let mut bs0 = BinarySearch {
    f: p0,
    lb: 0,
    ub: xs.len() as i64,
  };
  assert_eq!(bs0.lower_bound(), 1);

  let p1 = |i: i64| xs[i as usize] > 2;
  let mut bs1 = BinarySearch {
    f: p1,
    lb: 0,
    ub: xs.len() as i64,
  };
  assert_eq!(bs1.lower_bound(), 6);

  let p2 = |i: i64| xs[i as usize] >= 0;
  let mut bs2 = BinarySearch {
    f: p2,
    lb: 0,
    ub: xs.len() as i64,
  };
  assert_eq!(bs2.lower_bound(), 0);

  let mut extval = 0;
  let p3 = |i: i64| {
    extval += 1;
    xs[i as usize] >= 100
  };
  let mut bs3 = BinarySearch {
    f: p3,
    lb: 0,
    ub: xs.len() as i64,
  };
  assert_eq!(bs3.lower_bound(), 9);
}
