struct BTreeRangeSet {
  set: std::collections::BTreeSet<(i64, i64)>,
}

impl BTreeRangeSet {
  pub fn new() -> Self {
    Self {
      set: vec![(i64::MIN, i64::MIN), (i64::MAX, i64::MAX)].into_iter().collect(),
    }
  }
  pub fn insert(&mut self, x: i64) -> bool {
    let &(nl, nr) = self.set.range((x + 1, x + 1)..).next().unwrap();
    let &(l, r) = self.set.range(..(x + 1, x + 1)).next_back().unwrap();
    if l <= x && x <= r {
      return false;
    }
    if r == x - 1 {
      if nl == x + 1 {
        self.set.remove(&(nl, nr));
        self.set.remove(&(l, r));
        self.set.insert((l, nr));
      } else {
        self.set.remove(&(l, r));
        self.set.insert((l, x));
      }
    } else {
      if nl == x + 1 {
        self.set.remove(&(nl, nr));
        self.set.insert((x, nr));
      } else {
        self.set.insert((x, x));
      }
    }
    true
  }
  pub fn range_insert(&mut self, range: impl std::ops::RangeBounds<i64>) -> bool {
    let start = match range.start_bound() {
      std::ops::Bound::Unbounded => i64::MIN,
      std::ops::Bound::Excluded(&s) => s + 1,
      std::ops::Bound::Included(&s) => s,
    };
    let end = match range.end_bound() {
      std::ops::Bound::Unbounded => i64::MAX,
      std::ops::Bound::Excluded(&e) => e - 1,
      std::ops::Bound::Included(&e) => e,
    };
    let mut range = (start, end);
    let mut remove = vec![];
    for &(l, r) in self.set.range(..(end + 1, end + 1)).rev() {
      if r < range.0 {
        break;
      }
      if l.max(range.0) <= r.min(range.1) {
        remove.push((l, r));
      }
    }
    for (l, r) in remove {
      range = (range.0.min(l), range.1.max(r));
      self.set.remove(&(l, r));
    }
    let &(l, r) = self.set.range((end + 1, end + 1)..).next().unwrap();
    if l == end + 1 {
      self.set.remove(&(l, r));
      range = (range.0.min(l), range.1.max(r));
    }
    let &(l, r) = self.set.range(..(start, start)).next_back().unwrap();
    if r == start - 1 {
      self.set.remove(&(l, r));
      range = (range.0.min(l), range.1.max(r));
    }
    self.set.insert(range);
    true
  }
  pub fn contains(&self, range: impl std::ops::RangeBounds<i64>) -> Option<(i64, i64)> {
    let start = match range.start_bound() {
      std::ops::Bound::Unbounded => i64::MIN,
      std::ops::Bound::Excluded(&s) => s + 1,
      std::ops::Bound::Included(&s) => s,
    };
    let end = match range.end_bound() {
      std::ops::Bound::Unbounded => i64::MAX,
      std::ops::Bound::Excluded(&e) => e - 1,
      std::ops::Bound::Included(&e) => e,
    };
    let &(l, r) = self.set.range(..(start + 1, start + 1)).next_back().unwrap();
    if l <= start && end <= r {
      Some((l, r))
    } else {
      None
    }
  }

  pub fn remove(&mut self, x: i64) -> bool {
    let &(l, r) = self.set.range(..(x + 1, x + 1)).next_back().unwrap();
    if r < x {
      return false;
    }
    if l == r && r == x {
      self.set.remove(&(l, r));
    } else if r == x {
      self.set.remove(&(l, r));
      self.set.insert((l, x - 1));
    } else if l == x {
      self.set.remove(&(l, r));
      self.set.insert((x + 1, r));
    } else {
      self.set.remove(&(l, r));
      self.set.insert((x + 1, r));
      self.set.insert((l, x - 1));
    }
    true
  }
  pub fn mex(&self, x: i64) -> i64 {
    let &(l, r) = self.set.range((x, x)..).next().unwrap();
    if l <= x && x <= r {
      return r + 1;
    }
    x
  }
}

#[test]
fn range_set_test() {
  let mut range_set = BTreeRangeSet::new();
  range_set.insert(0);
  range_set.insert(1);
  range_set.insert(2);
  assert_eq!(range_set.contains(0..1).is_some(), true);
  assert_eq!(range_set.contains(0..2).is_some(), true);
  assert_eq!(range_set.contains(0..3).is_some(), true);
  assert_eq!(range_set.contains(0..4).is_some(), false);
  assert_eq!(range_set.contains(2..10).is_some(), false);
}
