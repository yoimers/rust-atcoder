#[derive(Debug)]
struct LazySegmentTree<T> {
    size: usize,
    tree: Vec<T>,
    lazy: Vec<Option<T>>,
}
use std::fmt::Debug;
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

/// https://ei1333.github.io/luzhiled/snippets/structure/segment-tree.html
use cargo_snippet::snippet;

/// 遅延セグ木
///
/// 範囲updateをサポートする。
/// update時には作用素のみをノードに載せるのみにとどめ、
/// query時にあるノードの作用素の値が必要になった時に作用素を伝搬する。
/// この回数がたかだかO(logN)しかない。
///
/// 計算量:
/// update l r f: O(logN)
/// query l r: O(logN)

#[snippet("SEG_LAZY")]
trait SEGLazyImpl {
    type Monoid: Copy;
    type F: Copy + PartialEq;
    fn e() -> Self::Monoid;
    fn id() -> Self::F;
    /// x `op` y
    fn op(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid;
    /// f(x)
    fn ap(f: Self::F, x: Self::Monoid) -> Self::Monoid;
    /// f . g
    /// f: 更新時上から来る値
    /// g: 更新時の今の値
    fn compose(f: Self::F, g: Self::F) -> Self::F;
}

#[snippet("SEG_LAZY")]
struct SEGLazy<T: SEGLazyImpl> {
    n: usize,
    data: Vec<T::Monoid>,
    lazy: Vec<T::F>,
}

#[snippet("SEG_LAZY")]
impl<T: SEGLazyImpl> SEGLazy<T> {
    pub fn new(n: usize, init: T::Monoid) -> Self {
        let mut m = 1;
        while m < n {
            m *= 2;
        }
        SEGLazy {
            n: m,
            data: vec![init; m * 2],
            lazy: vec![T::id(); m * 2],
        }
    }
    fn propagate(&mut self, k: usize) {
        if self.lazy[k] != T::id() {
            if k < self.n {
                self.lazy[2 * k + 0] = T::compose(self.lazy[k], self.lazy[2 * k + 0]);
                self.lazy[2 * k + 1] = T::compose(self.lazy[k], self.lazy[2 * k + 1]);
            }
            self.data[k] = T::ap(self.lazy[k], self.data[k]);
            self.lazy[k] = T::id();
        }
    }
    fn do_update(
        &mut self,
        a: usize,
        b: usize,
        x: T::F,
        k: usize,
        l: usize,
        r: usize,
    ) -> T::Monoid {
        self.propagate(k);
        if r <= a || b <= l {
            self.data[k]
        } else if a <= l && r <= b {
            self.lazy[k] = T::compose(x, self.lazy[k]);
            self.propagate(k);
            self.data[k]
        } else {
            self.data[k] = T::op(
                self.do_update(a, b, x, 2 * k + 0, l, (l + r) >> 1),
                self.do_update(a, b, x, 2 * k + 1, (l + r) >> 1, r),
            );
            self.data[k]
        }
    }
    #[doc = "[l,r)"]
    pub fn update(&mut self, l: usize, r: usize, x: T::F) -> T::Monoid {
        let n = self.n;
        self.do_update(l, r, x, 1, 0, n)
    }
    fn do_query(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T::Monoid {
        self.propagate(k);
        if r <= a || b <= l {
            T::e()
        } else if a <= l && r <= b {
            self.data[k]
        } else {
            T::op(
                self.do_query(a, b, 2 * k + 0, l, (l + r) >> 1),
                self.do_query(a, b, 2 * k + 1, (l + r) >> 1, r),
            )
        }
    }
    #[doc = "[l,r)"]
    pub fn query(&mut self, l: usize, r: usize) -> T::Monoid {
        let n = self.n;
        self.do_query(l, r, 1, 0, n)
    }
}

#[snippet("SEG_LAZY_MAX_RUQ")]
struct MAX_RUQ;
#[snippet("SEG_LAZY_MAX_RUQ")]
impl SEGLazyImpl for MAX_RUQ {
    type Monoid = i64;
    type F = i64;
    fn e() -> Self::Monoid {
        0
    }
    fn id() -> Self::F {
        0
    }
    fn op(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid {
        std::cmp::max(x, y)
    }
    fn ap(f: Self::F, x: Self::Monoid) -> Self::Monoid {
        f
    }
    fn compose(f: Self::F, g: Self::F) -> Self::F {
        f
    }
}
#[test]
fn test_MAX_RUQ() {
    let mut seg: SEGLazy<MAX_RUQ> = SEGLazy::new(10, MAX_RUQ::id());
    assert_eq!(seg.query(0, 3), 0);
    seg.update(0, 2, 10); // [10,10,0,...]
    assert_eq!(seg.query(0, 3), 10);
    assert_eq!(seg.query(2, 3), 0);
    seg.update(1, 5, 20);
    assert_eq!(seg.query(0, 3), 20);
    assert_eq!(seg.query(0, 1), 10);
    seg.update(0, 1, 5);
    assert_eq!(seg.query(0, 1), 5);
}

#[snippet("SEG_LAZY_MIN_RUQ")]
struct MIN_RUQ;
#[snippet("SEG_LAZY_MIN_RUQ")]
impl SEGLazyImpl for MIN_RUQ {
    type Monoid = i64;
    type F = i64;
    fn e() -> Self::Monoid {
        std::i64::MAX
    }
    fn id() -> Self::F {
        std::i64::MAX
    }
    fn op(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid {
        std::cmp::min(x, y)
    }
    fn ap(f: Self::F, x: Self::Monoid) -> Self::Monoid {
        f
    }
    fn compose(f: Self::F, g: Self::F) -> Self::F {
        f
    }
}
#[test]
fn test_MIN_RUQ() {
    // DSL_2_D
    let mut seg: SEGLazy<MIN_RUQ> = SEGLazy::new(8, MIN_RUQ::e());
    seg.update(1, 7, 5);
    seg.update(2, 8, 2);
    seg.update(2, 6, 7);
    assert_eq!(seg.query(3, 4), 7);
    seg.update(4, 7, 6);
    assert_eq!(seg.query(0, 1), std::i64::MAX);
    seg.update(0, 8, 9);
    assert_eq!(seg.query(2, 3), 9);
    assert_eq!(seg.query(3, 4), 9);
    seg.update(1, 8, 2);
}

#[snippet("SEG_LAZY_MAX_RAQ")]
struct MAX_RAQ;
#[snippet("SEG_LAZY_MAX_RAQ")]
impl SEGLazyImpl for MAX_RAQ {
    type Monoid = i64;
    type F = i64;
    fn e() -> Self::Monoid {
        std::i64::MIN
    }
    fn id() -> Self::F {
        0
    }
    fn op(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid {
        std::cmp::max(x, y)
    }
    fn ap(f: Self::F, x: Self::Monoid) -> Self::Monoid {
        x + f
    }
    fn compose(f: Self::F, g: Self::F) -> Self::F {
        g + f
    }
}

#[snippet("SEG_LAZY_MIN_RAQ")]
struct MIN_RAQ;
#[snippet("SEG_LAZY_MIN_RAQ")]
impl SEGLazyImpl for MIN_RAQ {
    type Monoid = i64;
    type F = i64;
    fn e() -> Self::Monoid {
        std::i64::MAX
    }
    fn id() -> Self::F {
        0
    }
    fn op(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid {
        std::cmp::min(x, y)
    }
    fn ap(f: Self::F, x: Self::Monoid) -> Self::Monoid {
        x + f
    }
    fn compose(f: Self::F, g: Self::F) -> Self::F {
        g + f
    }
}
#[test]
fn test_rmq_raq() {
    // DSL_2_H
    let mut seg: SEGLazy<MIN_RAQ> = SEGLazy::new(6, 0);
    seg.update(1, 4, 1);
    seg.update(2, 5, -2);
    assert_eq!(seg.query(0, 6), -2);
    assert_eq!(seg.query(0, 2), 0);
    seg.update(3, 6, 3);
    assert_eq!(seg.query(3, 5), 1);
    assert_eq!(seg.query(0, 6), -1);
}
