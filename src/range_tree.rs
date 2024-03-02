use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Point<X, Y, V> {
  x: X,
  y: Y,
  val: V,
}
struct RangeTree<X, Y, V> {
  seg: Vec<Vec<Y>>,
  sum: Vec<Vec<V>>,
  size: usize,
  xs: Vec<X>,
}

impl RangeTree<i64, i64, i64> {
  pub fn new(points: &mut Vec<Point<i64, i64, i64>>) -> Self {
    points.sort();
    let mut xs = points.iter().map(|p| p.x).dedup().collect_vec();
    let size = xs.len().next_power_of_two();
    xs.resize(size, i64::MAX / 2);
    let mut seg = vec![vec![]; 2 * size];
    let mut sum = vec![vec![]; 2 * size];
    let mut idx = 0;
    for p in points.iter() {
      if xs[idx] != p.x {
        idx += 1;
      }
      seg[idx + size].push(p.y);
      sum[idx + size].push(p.val);
    }
    for i in (1..size).rev() {
      let (mut l, mut r) = (0, 0);
      while l < seg[2 * i].len() || r < seg[2 * i + 1].len() {
        let f = if r >= seg[2 * i + 1].len() {
          true
        } else if l >= seg[2 * i].len() {
          false
        } else if seg[2 * i][l] < seg[2 * i + 1][r] {
          true
        } else {
          false
        };
        if f {
          let tmp = seg[2 * i][l];
          seg[i].push(tmp);
          let tmp = sum[2 * i][l];
          sum[i].push(tmp);
          l += 1;
        } else {
          let tmp = seg[2 * i + 1][r];
          seg[i].push(tmp);
          let tmp = sum[2 * i + 1][r];
          sum[i].push(tmp);
          r += 1;
        }
      }
    }
    for i in 0..2 * size {
      let mut replace = Vec::with_capacity(sum[i].len() + 1);
      replace.push(i64::default());
      for &v in sum[i].iter() {
        replace.push(replace.last().unwrap() + v);
      }
      std::mem::swap(&mut sum[i], &mut replace);
    }
    Self { seg, sum, size, xs }
  }

  pub fn fold(&self, x_range: impl std::ops::RangeBounds<i64>, y_range: impl std::ops::RangeBounds<i64>) -> i64 {
    let sx = match x_range.start_bound() {
      std::ops::Bound::Unbounded => i64::MIN,
      std::ops::Bound::Excluded(&s) => s + 1,
      std::ops::Bound::Included(&s) => s,
    };
    let tx = match x_range.end_bound() {
      std::ops::Bound::Unbounded => i64::MAX,
      std::ops::Bound::Excluded(&e) => e,
      std::ops::Bound::Included(&e) => e + 1,
    };
    let sy = match y_range.start_bound() {
      std::ops::Bound::Unbounded => i64::MIN,
      std::ops::Bound::Excluded(&s) => s + 1,
      std::ops::Bound::Included(&s) => s,
    };
    let ty = match y_range.end_bound() {
      std::ops::Bound::Unbounded => i64::MAX,
      std::ops::Bound::Excluded(&e) => e,
      std::ops::Bound::Included(&e) => e + 1,
    };
    let mut l = self.xs.binary_search(&sx).unwrap_or_else(|v| v) + self.size;
    let mut r = self.xs.binary_search(&tx).unwrap_or_else(|v| v) + self.size;
    let mut ret = 0;
    while l < r {
      if l & 1 == 1 {
        let hi = self.seg[l].binary_search(&ty).unwrap_or_else(|v| v);
        let lo = self.seg[l].binary_search(&sy).unwrap_or_else(|v| v);
        ret += self.sum[l][hi] - self.sum[l][lo];
        l += 1;
      }
      if r & 1 == 1 {
        r -= 1;
        let hi = self.seg[r].binary_search(&ty).unwrap_or_else(|v| v);
        let lo = self.seg[r].binary_search(&sy).unwrap_or_else(|v| v);
        ret += self.sum[r][hi] - self.sum[r][lo];
      }
      l = l >> 1;
      r = r >> 1;
    }
    ret
  }
}
