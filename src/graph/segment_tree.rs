#[derive(Debug)]
struct SegmentTree<T> {
  size: usize,
  tree: Vec<T>,
}

trait Monoid: Copy {
  fn identity() -> Self;
  fn op(l: Self, r: Self) -> Self;
}
use std::fmt::Debug;
use std::ops::{Bound, Range, RangeBounds};
impl<T: Copy + Eq + Debug + Monoid> SegmentTree<T> {
  fn new(n: usize) -> Self {
    let size = n.next_power_of_two();
    Self {
      size,
      tree: vec![T::identity(); 2 * size - 1],
    }
  }
  //[left, right) left < right,   left >= right → return self.identity
  fn get(&self, range: impl RangeBounds<usize>) -> T {
    let start = match range.start_bound() {
      Bound::Unbounded => 0,
      Bound::Excluded(&s) => s + 1,
      Bound::Included(&s) => s.max(0),
    };
    let end = match range.end_bound() {
      Bound::Unbounded => self.size,
      Bound::Excluded(&e) => e,
      Bound::Included(&e) => (e + 1).min(self.size),
    };
    self._get(&(start..end), 0, 0, self.size)
  }

  fn _get(&self, range: &Range<usize>, k: usize, l: usize, r: usize) -> T {
    if range.end <= l || r <= range.start {
      return T::identity();
    }
    if range.start <= l && r <= range.end {
      return self.tree[k];
    }
    let vl = self._get(&range, 2 * k + 1, l, (l + r) / 2);
    let vr = self._get(&range, 2 * k + 2, (l + r) / 2, r);
    (T::op)(vl, vr)
  }

  fn update(&mut self, index: usize, value: T) {
    let mut i = self.size + index - 1;
    self.tree[i] = value;
    while i > 0 {
      i = (i - 1) / 2;
      self.tree[i] = (T::op)(self.tree[2 * i + 1], self.tree[2 * i + 2])
    }
  }
  fn add(&mut self, index: usize, value: T) {
    let v = self.get(index..=index);
    self.update(index, (T::op)(v, value))
  }
}

impl Monoid for usize {
  fn identity() -> Self {
    0
  }
  fn op(l: Self, r: Self) -> Self {
    l.max(r)
  }
}
impl Monoid for i64 {
  fn identity() -> Self {
    10000000
  }
  fn op(l: Self, r: Self) -> Self {
    l.min(r)
  }
}
#[test]
fn test_segment_tree() {
  let mut s = SegmentTree::<usize>::new(8);
  s.update(0, 0);
  s.update(1, 1);
  s.update(2, 2);
  s.update(4, 10);
  s.update(7, 20);
  assert_eq!(s.get(0..4), 2);
  assert_eq!(s.get(0..), 20);
  assert_eq!(s.get(0..1), 0);
  assert_eq!(s.get(2..4), 2);
  assert_eq!(s.get(3..5), 10);

  let mut s = SegmentTree::<i64>::new(12);
  s.update(0, 33);
  s.update(1, 1);
  s.update(2, 2);
  s.update(4, 10);
  s.update(7, 20);
  s.update(9, 1);
  s.update(10, 12);
  s.update(11, 200);
  assert_eq!(s.get(0..4), 1);
  assert_eq!(s.get(0..1), 33);
  assert_eq!(s.get(2..2), 10000000);
  assert_eq!(s.get(3..11), 1);
  assert_eq!(s.get(8..5), 10000000);
  assert_eq!(s.get(10..11), 12);
  assert_eq!(s.get(0..11), 1);
}

mod SegTree {
  use std::ops::{Bound, RangeBounds};

  use itertools::Itertools;

  pub trait Monoid: Copy {
    fn id() -> Self;
    fn op(l: Self, r: Self) -> Self;
  }
  pub struct SegTree<T: Monoid> {
    pub n: usize,
    pub data: Vec<T>,
    _build: bool,
  }

  impl<T: Monoid> SegTree<T> {
    pub fn new(n: usize) -> Self {
      Self {
        n,
        data: vec![T::id(); 2 * n],
        _build: false,
      }
    }
    pub fn set(&mut self, idx: usize, x: T) {
      let mut idx = idx + self.n;
      self.data[idx] = x;
      if !self._build {
        return;
      }
      while idx > 1 {
        idx = idx >> 1;
        self.data[idx] = T::op(self.data[idx << 1], self.data[(idx << 1) | 1]);
      }
    }
    fn build(&mut self) {
      for i in (0..self.n).rev() {
        self.data[i] = T::op(self.data[i << 1], self.data[(i << 1) | 1]);
      }
      self._build = true;
    }
    pub fn query(&mut self, range: impl RangeBounds<usize>) -> T {
      if !self._build {
        self.build();
      }
      let mut start = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Excluded(&s) => s + 1,
        Bound::Included(&s) => s.max(0),
      } + self.n;
      let mut end = match range.end_bound() {
        Bound::Unbounded => self.n,
        Bound::Excluded(&e) => e,
        Bound::Included(&e) => (e + 1).min(self.n),
      } + self.n;
      let mut lv = T::id();
      let mut rv = T::id();
      while start < end {
        if start & 1 > 0 {
          lv = T::op(lv, self.data[start]);
          start += 1;
        }
        if end & 1 > 0 {
          end -= 1;
          rv = T::op(self.data[end], rv);
        }
        start = start >> 1;
        end = end >> 1;
      }
      T::op(lv, rv)
    }
  }

  impl Monoid for usize {
    fn id() -> Self {
      0
    }
    fn op(l: Self, r: Self) -> Self {
      l.max(r)
    }
  }
  impl Monoid for i64 {
    fn id() -> Self {
      10000000
    }
    fn op(l: Self, r: Self) -> Self {
      l.min(r)
    }
  }
  #[test]
  fn test_segment_tree() {
    let mut s = SegTree::<usize>::new(8);
    s.set(0, 0);
    s.set(1, 1);
    s.set(2, 2);
    s.set(4, 10);
    s.set(7, 20);
    assert_eq!(s.query(0..4), 2);
    assert_eq!(s.query(0..), 20);
    assert_eq!(s.query(0..1), 0);
    assert_eq!(s.query(2..4), 2);
    assert_eq!(s.query(3..5), 10);

    let mut s = SegTree::<i64>::new(12);
    s.set(0, 33);
    s.set(1, 1);
    s.set(2, 2);
    s.set(4, 10);
    s.set(7, 20);
    s.set(9, 1);
    s.set(10, 12);
    s.set(11, 200);
    assert_eq!(s.query(0..4), 1);
    assert_eq!(s.query(0..1), 33);
    assert_eq!(s.query(2..2), 10000000);
    assert_eq!(s.query(3..11), 1);
    assert_eq!(s.query(8..5), 10000000);
    assert_eq!(s.query(10..11), 12);
    assert_eq!(s.query(0..11), 1);
  }

  impl<const N: usize> Monoid for [(usize, usize); N] {
    fn id() -> Self {
      [(0, 0); N]
    }
    fn op(l: Self, r: Self) -> Self {
      let mut x: Vec<(usize, usize)> = l.into_iter().chain(r.into_iter()).collect_vec();
      x.sort();
      let mut y = vec![];
      for (d, cnt) in x {
        if let Some((prev, c)) = y.pop() {
          if d == prev {
            y.push((d, c + cnt));
          } else {
            y.push((prev, c));
            y.push((d, cnt));
          }
        } else {
          y.push((d, cnt));
        }
      }
      y.into_iter().rev().take(N).collect_vec().try_into().unwrap()
    }
  }
}
