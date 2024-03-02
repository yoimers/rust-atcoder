trait BinarySearch<T> {
  fn lower_bound(&self, range: impl std::ops::RangeBounds<usize>, x: &T) -> Result<usize, usize>;
  fn upper_bound(&self, range: impl std::ops::RangeBounds<usize>, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
  fn lower_bound(&self, range: impl std::ops::RangeBounds<usize>, x: &T) -> Result<usize, usize> {
    let mut l = match range.start_bound() {
      std::ops::Bound::Unbounded => 0,
      std::ops::Bound::Excluded(&s) => s + 1,
      std::ops::Bound::Included(&s) => s.max(0),
    };
    let mut r = match range.end_bound() {
      std::ops::Bound::Unbounded => self.len(),
      std::ops::Bound::Excluded(&e) => e.min(self.len()),
      std::ops::Bound::Included(&e) => (e + 1).min(self.len()),
    };
    while l < r {
      let mid = (l + r) / 2;
      if x <= &self[mid] {
        r = mid;
      } else {
        l = mid + 1;
      }
    }
    if self.len() <= l {
      return Err(l);
    }
    if x == &self[l] {
      Ok(l)
    } else {
      Err(l)
    }
  }
  fn upper_bound(&self, range: impl std::ops::RangeBounds<usize>, x: &T) -> usize {
    let mut l = match range.start_bound() {
      std::ops::Bound::Unbounded => 0,
      std::ops::Bound::Excluded(&s) => s + 1,
      std::ops::Bound::Included(&s) => s.max(0),
    };
    let mut r = match range.end_bound() {
      std::ops::Bound::Unbounded => self.len(),
      std::ops::Bound::Excluded(&e) => e.min(self.len()),
      std::ops::Bound::Included(&e) => (e + 1).min(self.len()),
    };
    while l < r {
      let mid = (l + r) / 2;
      if x < &self[mid] {
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
  let x = xs.lower_bound(0..4, &2);
  assert_eq!(x, Ok(1));
  let x = xs.lower_bound(0..xs.len(), &3);
  assert_eq!(x, Ok(6));
  let x = xs.lower_bound(0..5, &3);
  assert_eq!(x, Err(5));
  let x = xs.lower_bound(0.., &100);
  assert_eq!(x, Err(xs.len()));
  let x = xs.lower_bound(0.., &0);
  assert_eq!(x, Err(0));
  let x = xs.lower_bound(4.., &0);
  assert_eq!(x, Err(4));

  let xs = vec![1, 2, 2, 2, 2, 2, 3, 4, 5];
  let x = xs.upper_bound(4.., &0);
  assert_eq!(x, 4);
  let x = xs.upper_bound(0.., &2);
  assert_eq!(x, 6);
  let x = xs.upper_bound(0.., &1000);
  assert_eq!(x, xs.len());
  let x = xs.upper_bound(0.., &0);
  assert_eq!(x, 0);
}
