use cargo_snippet::snippet;

/// 対称群の作用を互換に制限したもの
/// (i,j) * σ、σ * (i,j)　の計算量 O(1)
#[snippet("TranspositionGroup")]
#[derive(Debug, Clone, PartialEq)]
pub struct TranspositionGroup {
    n: usize,
    g: Vec<usize>,
    inv_g: Vec<usize>,
}
#[snippet("TranspositionGroup")]
impl TranspositionGroup {
    /// O(n)
    pub fn new(n: usize) -> Self {
        let mut g = vec![0; n];
        let mut inv_g = vec![0; n];
        for i in 0..n {
            g[i] = i;
            inv_g[i] = i;
        }
        Self { n, g, inv_g }
    }
    /// O(n)
    pub fn from(vec: Vec<usize>) -> Self {
        let mut inv_g = vec![0; vec.len()];
        for i in 0..vec.len() {
            inv_g[vec[i]] = i;
        }
        Self {
            n: vec.len(),
            g: vec,
            inv_g: inv_g,
        }
    }
    /// O(1)
    ///
    /// (i, j) * σ
    pub fn left(&mut self, i: usize, j: usize) {
        self.g.swap(self.inv_g[i], self.inv_g[j]);
        self.inv_g.swap(i, j);
    }
    /// O(1)
    ///
    /// σ * (i, j)
    pub fn right(&mut self, i: usize, j: usize) {
        self.inv_g.swap(self.g[i], self.g[j]);
        self.g.swap(i, j);
    }
    /// O(巡回置換の長さ-1)
    ///
    /// (i, j, k , l) * σ など
    /// ex (1 2 3 4) = (1 2)*(2 3)*(3 4)
    pub fn left_cyclic(&mut self, cyclic: &Vec<usize>) {
        if cyclic.len() <= 1 {
            return;
        }
        for i in (1..cyclic.len()).rev() {
            self.left(cyclic[i - 1], cyclic[i]);
        }
    }
    /// O(巡回置換の長さ-1)
    ///
    /// σ * (i, j, k , l) など
    /// ex (1 2 3 4) = (1 2)*(2 3)*(3 4)
    pub fn right_cyclic(&mut self, cyclic: &Vec<usize>) {
        if cyclic.len() <= 1 {
            return;
        }
        for i in 1..cyclic.len() {
            self.right(cyclic[i - 1], cyclic[i]);
        }
    }
}

#[test]
fn test_transposition_group() {
    {
        let mut g = TranspositionGroup::new(6);
        let mut h = TranspositionGroup::from(vec![1, 2, 0, 3, 4, 5]);
        g.left(1, 3);
        assert_eq!(g, TranspositionGroup::from(vec![0, 3, 2, 1, 4, 5]));
        h.left(0, 1);
        assert_eq!(h, TranspositionGroup::from(vec![0, 2, 1, 3, 4, 5]));
        h.left(2, 3);
        assert_eq!(h, TranspositionGroup::from(vec![0, 3, 1, 2, 4, 5]));
        h.left(3, 5);
        assert_eq!(h, TranspositionGroup::from(vec![0, 5, 1, 2, 4, 3]));
        h.right(3, 5);
        assert_eq!(h, TranspositionGroup::from(vec![0, 5, 1, 3, 4, 2]));
        h.right(1, 2);
        assert_eq!(h, TranspositionGroup::from(vec![0, 1, 5, 3, 4, 2]));
        h.right(3, 0);
        assert_eq!(h, TranspositionGroup::from(vec![3, 1, 5, 0, 4, 2]));
    }
    {
        let mut g = TranspositionGroup::new(6);
        g.left_cyclic(&vec![0, 1, 2, 3]);
        assert_eq!(g, TranspositionGroup::from(vec![1, 2, 3, 0, 4, 5]));
        g.right_cyclic(&vec![2, 3, 4, 5]);
        assert_eq!(g, TranspositionGroup::from(vec![1, 2, 0, 4, 5, 3]));
        g.left_cyclic(&vec![1, 2, 3]);
        assert_eq!(g, TranspositionGroup::from(vec![2, 3, 0, 4, 5, 1]));
        g.left_cyclic(&vec![1]);
        assert_eq!(g, TranspositionGroup::from(vec![2, 3, 0, 4, 5, 1]));
        g.left_cyclic(&vec![]);
        assert_eq!(g, TranspositionGroup::from(vec![2, 3, 0, 4, 5, 1]));
    }
}

// 0, 1, 2, ..., n-1 対称群
#[snippet("SymmetricGroup")]
#[derive(Debug, Clone, PartialEq)]
pub struct SymmetricGroup {
    n: usize,
    g: Vec<usize>,
}
#[snippet("SymmetricGroup")]
impl SymmetricGroup {
    /// O(n)
    pub fn new(n: usize) -> Self {
        let mut g = vec![0; n];
        for i in 0..n {
            g[i] = i;
        }
        Self { n, g }
    }
    /// O(1)
    pub fn from(g: Vec<usize>) -> Self {
        Self { n: g.len(), g: g }
    }
    /// O(n)
    ///
    /// (i, j) * σ
    pub fn transposition(&mut self, i: usize, j: usize) {
        let mut inv_gi = 0;
        let mut inv_gj = 0;
        for k in 0..self.g.len() {
            if self.g[k] == i {
                inv_gi = k;
            }
            if self.g[k] == j {
                inv_gj = k;
            }
        }
        self.g.swap(inv_gi, inv_gj);
    }
    /// O(1)
    ///
    /// σ * (i, j)
    pub fn inv_transposition(&mut self, i: usize, j: usize) {
        self.g.swap(i, j);
    }
    /// O(n*log(n))
    pub fn pow(&self, m: usize) -> Self {
        let mut m = m;
        let mut pow = Self::new(self.n);
        let mut y = self.clone();
        while m > 0 {
            if m % 2 == 1 {
                pow = pow.mul(&y);
            }
            y = y.mul(&y);
            m /= 2;
        }
        Self {
            n: self.n,
            g: pow.g,
        }
    }
    /// O(n)
    pub fn mul(&self, rhs: &Self) -> Self {
        assert_eq!(self.n, rhs.n);
        let mut mul = vec![0; self.n];
        for i in 0..self.n {
            mul[i] = self.g[rhs.g[i]];
        }
        Self { n: self.n, g: mul }
    }
    /// O(n)
    pub fn inv(&self) -> Self {
        let mut inv = vec![0; self.n];
        for i in 0..self.n {
            inv[self.g[i]] = i;
        }
        Self::from(inv)
    }
}

impl std::ops::Mul for SymmetricGroup {
    type Output = Self;
    /// O(n)
    fn mul(self, rhs: Self) -> Self::Output {
        let mut mul = vec![0; self.n];
        for i in 0..self.n {
            mul[i] = self.g[rhs.g[i]];
        }
        Self { n: self.n, g: mul }
    }
}

#[test]
fn test_symmetric_group() {
    {
        let g = SymmetricGroup::from(vec![1, 2, 3, 4, 5, 0]);
        let h = SymmetricGroup::from(vec![0, 2, 1, 4, 3, 5]);
        assert_eq!(g.pow(0), SymmetricGroup::new(g.n));
        assert_eq!(g.pow(1), g);
        assert_eq!(g.pow(2), SymmetricGroup::from(vec![2, 3, 4, 5, 0, 1]));
        assert_eq!(g.pow(3), SymmetricGroup::from(vec![3, 4, 5, 0, 1, 2]));
        assert_eq!(g.pow(4), SymmetricGroup::from(vec![4, 5, 0, 1, 2, 3]));
        assert_eq!(g.pow(5), SymmetricGroup::from(vec![5, 0, 1, 2, 3, 4]));
        assert_eq!(g.pow(6), SymmetricGroup::from(vec![0, 1, 2, 3, 4, 5]));
        assert_eq!(g.mul(&h), SymmetricGroup::from(vec![1, 3, 2, 5, 4, 0]));
        assert_eq!(
            g.mul(&h).mul(&h),
            SymmetricGroup::from(vec![1, 2, 3, 4, 5, 0])
        );
        assert_eq!(g.inv(), SymmetricGroup::from(vec![5, 0, 1, 2, 3, 4]));
        assert_eq!(h.inv(), h);
    }
    {
        let mut h1 = SymmetricGroup::from(vec![0, 1, 2, 3, 4, 5]);
        let mut h2 = SymmetricGroup::from(vec![0, 1, 2, 3, 4, 5]);
        h1.inv_transposition(0, 1);
        h2.transposition(0, 1);
        assert_eq!(h1, SymmetricGroup::from(vec![1, 0, 2, 3, 4, 5]));
        assert_eq!(h2, SymmetricGroup::from(vec![1, 0, 2, 3, 4, 5]));

        h1.inv_transposition(1, 2);
        h2.transposition(1, 2);
        assert_eq!(h1, SymmetricGroup::from(vec![1, 2, 0, 3, 4, 5]));
        assert_eq!(h2, SymmetricGroup::from(vec![2, 0, 1, 3, 4, 5]));

        h1.inv_transposition(2, 3);
        h2.transposition(2, 3);
        assert_eq!(h1, SymmetricGroup::from(vec![1, 2, 3, 0, 4, 5]));
        assert_eq!(h2, SymmetricGroup::from(vec![3, 0, 1, 2, 4, 5]));
    }
}
