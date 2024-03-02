mod LazySeg {
  use std::{
    fmt::Debug,
    mem::swap,
    ops::{Bound, RangeBounds},
  };

  pub trait SEGLazyImpl {
    type Monoid: Copy + Default + Debug;
    type Action: Copy + PartialEq;
    fn id() -> Self::Monoid;
    fn op(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid;
    fn e() -> Self::Action;
    fn action(f: Self::Action, x: Self::Monoid) -> Self::Monoid;
    fn compose(f: Self::Action, g: Self::Action) -> Self::Action;
  }

  pub struct SEGLazy<T: SEGLazyImpl> {
    pub n: usize,
    pub data: Vec<T::Monoid>,
    pub lazy: Vec<T::Action>,
    _build: bool,
  }

  impl<T: SEGLazyImpl> SEGLazy<T> {
    pub fn new(n: usize) -> Self {
      Self {
        n,
        data: vec![T::id(); 2 * n],
        lazy: vec![T::e(); 2 * n],
        _build: false,
      }
    }

    pub fn set(&mut self, mut i: usize, x: T::Monoid) {
      i += self.n;
      if self._build {
        self._propagate_above(i);
        self.data[i] = x;
        self.lazy[i] = T::e();
        self._recalc_above(i);
      } else {
        self.data[i] = x;
      }
    }

    pub fn build(&mut self) {
      for i in (1..self.n).rev() {
        self.data[i] = T::op(self.data[i << 1], self.data[(i << 1) | 1]);
      }
      self._build = true;
    }

    pub fn query(&mut self, range: impl RangeBounds<usize>) -> T::Monoid {
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
        Bound::Excluded(&e) => e.min(self.n),
        Bound::Included(&e) => (e + 1).min(self.n),
      } + self.n;
      let s = start as isize;
      let e = end as isize;
      self._propagate_above((s / (s & -s)) as usize);
      self._propagate_above((e / (e & -e)) as usize - 1);

      let mut lv = T::id();
      let mut rv = T::id();
      while start < end {
        if start & 1 > 0 {
          lv = T::op(lv, self._eval_at(start));
          start += 1;
        }
        if end & 1 > 0 {
          end -= 1;
          rv = T::op(self._eval_at(end), rv);
        }
        start >>= 1;
        end >>= 1;
      }
      T::op(lv, rv)
    }

    pub fn operate_range(&mut self, range: impl RangeBounds<usize>, x: T::Action) {
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
      let s = start as isize;
      let s = (s / (s & -s)) as usize;
      let e = end as isize;
      let e = (e / (e & -e)) as usize - 1;
      self._propagate_above(s);
      self._propagate_above(e);

      while start < end {
        if start & 1 > 0 {
          self.lazy[start] = T::compose(self.lazy[start], x);
          start += 1;
        }
        if end & 1 > 0 {
          end -= 1;
          self.lazy[end] = T::compose(self.lazy[end], x);
        }
        start >>= 1;
        end >>= 1;
      }
      self._recalc_above(s);
      self._recalc_above(e);
    }

    fn _eval_at(&mut self, i: usize) -> T::Monoid {
      T::action(self.lazy[i], self.data[i])
    }

    fn _propagate_at(&mut self, i: usize) {
      self.data[i] = self._eval_at(i);
      self.lazy[i << 1] = T::compose(self.lazy[i << 1], self.lazy[i]);
      self.lazy[i << 1 | 1] = T::compose(self.lazy[i << 1 | 1], self.lazy[i]);
      self.lazy[i] = T::e();
    }

    fn _propagate_above(&mut self, i: usize) {
      for h in (1..Self::_bit_length(i)).rev() {
        self._propagate_at(i >> h);
      }
    }

    fn _recalc_above(&mut self, mut i: usize) {
      while i > 1 {
        i >>= 1;
        self.data[i] = T::op(self._eval_at(i << 1), self._eval_at((i << 1) | 1));
      }
    }

    fn _bit_length(mut n: usize) -> usize {
      let mut count = 0;
      while n > 0 {
        count += 1;
        n >>= 1;
      }
      count
    }
  }

  pub struct XXXXXXX;
  #[derive(Clone, Copy, Default, Debug)]
  pub struct S {
    pub max0: i64,
    pub max1: i64,
    pub l0: i64,
    pub l1: i64,
    pub r0: i64,
    pub r1: i64,
    pub w: i64,
  }
  impl SEGLazyImpl for XXXXXXX {
    type Monoid = S;
    type Action = i64;
    // (連続する1の長さのMAX, 左端の1の長さ, 左端の0の長さ, 右端の1の長さ, 右端の0の長さ, 区間の長さ)
    fn id() -> Self::Monoid {
      Self::Monoid::default()
    }
    fn e() -> Self::Action {
      0
    }
    fn op(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid {
      let mut retv = Self::Monoid::default();
      retv.max0 = x.max0.max(y.max0).max(x.r0 + y.l0);
      retv.max1 = x.max1.max(y.max1).max(x.r1 + y.l1);
      retv.l0 = if x.l0 == x.w { x.l0 + y.l0 } else { x.l0 };
      retv.l1 = if x.l1 == x.w { x.l1 + y.l1 } else { x.l1 };
      retv.r0 = if y.r0 == y.w { x.r0 + y.r0 } else { y.r0 };
      retv.r1 = if y.r1 == y.w { x.r1 + y.r1 } else { y.r1 };
      retv.w = x.w + y.w;
      retv
    }
    fn action(f: Self::Action, x: Self::Monoid) -> Self::Monoid {
      let mut y = x.clone();
      if f % 2 == 1 {
        swap(&mut y.l0, &mut y.l1);
        swap(&mut y.r0, &mut y.r1);
        swap(&mut y.max0, &mut y.max1);
      }
      y
    }
    fn compose(f: Self::Action, g: Self::Action) -> Self::Action {
      (f + g) % 2
    }
  }

  struct MIN_RAQ;
  impl SEGLazyImpl for MIN_RAQ {
    type Monoid = i64;
    type Action = i64;
    fn e() -> Self::Action {
      0
    }
    fn id() -> Self::Monoid {
      std::i64::MAX / 10
    }
    fn op(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid {
      std::cmp::min(x, y)
    }
    fn action(f: Self::Action, x: Self::Monoid) -> Self::Monoid {
      x + f
    }
    fn compose(f: Self::Action, g: Self::Action) -> Self::Action {
      g + f
    }
  }
  #[test]
  fn test_rmq_raq() {
    let mut seg: SEGLazy<MIN_RAQ> = SEGLazy::new(6);
    for i in 0..6 {
      seg.set(i, 0);
    }
    seg.operate_range(1..4, 1);
    seg.operate_range(2..5, -2);
    assert_eq!(seg.query(0..2), 0);
    assert_eq!(seg.query(0..6), -2);
    seg.operate_range(3..6, 3);
    assert_eq!(seg.query(3..5), 1);
    assert_eq!(seg.query(0..6), -1);
    seg.set(4, -10);
    assert_eq!(seg.query(0..6), -10);
  }
}
