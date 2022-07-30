#[derive(Debug)]
struct LazySegmentTree<T> {
    size: usize,
    tree: Vec<T>,
    lazy: Vec<Option<T>>,
}

trait Monoid: Copy {
    fn identity() -> Self;
    fn op(l: Self, r: Self) -> Self;
}

impl<T: Copy + Eq + Debug + Monoid> LazySegmentTree<T> {
    fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        Self {
            size,
            tree: vec![T::identity(); 2 * size - 1],
            lazy: vec![None; 2 * size - 1],
        }
    }
    //[left, right) left < right,   left >= right → return self.identity
    fn get(&mut self, left: usize, right: usize) -> T {
        self._get(left, right, 0, 0, self.size)
    }

    fn _get(&mut self, left: usize, right: usize, k: usize, l: usize, r: usize) -> T {
        self._eval(k);
        if right <= l || r <= left {
            return T::identity();
        }
        if left <= l && r <= right {
            return self.tree[k];
        }
        let vl = self._get(left, right, 2 * k + 1, l, (l + r) / 2);
        let vr = self._get(left, right, 2 * k + 2, (l + r) / 2, r);
        (T::op)(vl, vr)
    }
    fn range_update(&mut self, left: usize, right: usize, value: T) {
        self._range_update(left, right, value, 0, 0, self.size)
    }
    fn _range_update(&mut self, left: usize, right: usize, x: T, k: usize, l: usize, r: usize) {
        self._eval(k);
        if left <= l && r <= right {
            // 完全に内側の時
            self.lazy[k] = Some(x);
            self._eval(k);
        } else if left < r && l < right {
            // 一部区間が被る時
            self._range_update(left, right, x, k * 2 + 1, l, (l + r) / 2); // 左の子
            self._range_update(left, right, x, k * 2 + 2, (l + r) / 2, r); // 右の子
            self.tree[k] = (T::op)(self.tree[k * 2 + 1], self.tree[k * 2 + 2]);
        }
    }

    fn _eval(&mut self, k: usize) {
        if let Some(x) = self.lazy[k] {
            if k < self.size - 1 {
                self.lazy[k * 2 + 1] = self.lazy[k];
                self.lazy[k * 2 + 2] = self.lazy[k];
            }
            self.tree[k] = x;
            self.lazy[k] = None;
        }
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
    let mut s = LazySegmentTree::<usize>::new(5);
    s.range_update(0, 3, 20);
    s.range_update(1, 1, 2);
    s.range_update(2, 2, 4);
    s.range_update(4, 10, 1000);
    assert_eq!(s.get(0, 3), 20);
    assert_eq!(s.get(0, 1), 20);
    assert_eq!(s.get(3, 4), 0);
    assert_eq!(s.get(3, 5), 1000);

    let mut s = LazySegmentTree::<i64>::new(12);
    s.range_update(0, 3, 33);
    s.range_update(3, 5, 22);
    s.range_update(2, 4, 11);
    s.range_update(5, 9, 5);
    s.range_update(9, 12, 12);
    assert_eq!(s.get(0, 2), 33);
    assert_eq!(s.get(2, 4), 11);
    assert_eq!(s.get(2, 3), 11);
    assert_eq!(s.get(0, 11), 5);
    assert_eq!(s.get(8, 5), 10000000);
    assert_eq!(s.get(10, 11), 12);
    assert_eq!(s.get(0, 11), 5);
}
