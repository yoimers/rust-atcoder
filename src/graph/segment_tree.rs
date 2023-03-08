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
