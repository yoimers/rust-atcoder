use cargo_snippet::snippet;

#[snippet("UnionFind")]
pub struct UnionFind {
  par: Vec<usize>,
  siz: Vec<usize>,
}

#[snippet("UnionFind")]
impl UnionFind {
  pub fn new(n: usize) -> Self {
    UnionFind {
      par: (0..n).collect::<Vec<usize>>(),
      siz: vec![1; n],
    }
  }
  pub fn unite(&mut self, x: usize, y: usize) -> bool {
    let root_x = self.root(x);
    let root_y = self.root(y);
    if root_x == root_y {
      return false;
    }
    if self.siz[root_x] < self.siz[root_y] {
      self.par[root_x] = root_y;
      self.siz[root_y] += self.siz[root_x];
    } else {
      self.par[root_y] = root_x;
      self.siz[root_x] += self.siz[root_y];
    }
    true
  }
  pub fn issame(&mut self, x: usize, y: usize) -> bool {
    self.root(x) == self.root(y)
  }
  pub fn root(&mut self, x: usize) -> usize {
    if self.par[x] == x {
      return x;
    } else {
      self.par[x] = self.root(self.par[x]);
      return self.par[x];
    }
  }
  pub fn connected_size(&mut self) -> usize {
    let mut retv = 0;
    for i in 0..self.siz.len() {
      if self.root(i) == i {
        retv += 1;
      }
    }
    retv
  }
  pub fn size(&mut self, x: usize) -> usize {
    let root = self.root(x);
    self.siz[root]
  }
}

#[test]
#[cfg(test)]
fn test_union_find() {
  let mut s = UnionFind::new(5);
  s.unite(1, 4);
  s.unite(2, 3);
  assert_eq!(s.issame(1, 2), false);
  assert_eq!(s.issame(3, 4), false);
  assert_eq!(s.issame(1, 4), true);
  assert_eq!(s.issame(3, 2), true);
  assert_eq!(s.size(1), 2);
  assert_eq!(s.size(0), 1);

  s.unite(1, 3);
  assert_eq!(s.issame(2, 4), true);
  assert_eq!(s.issame(3, 0), false);
  assert_eq!(s.size(0), 1);
  assert_eq!(s.size(1), 4);
  assert_eq!(s.size(2), 4);

  s.unite(0, 4);
  assert_eq!(s.issame(0, 2), true);
  assert_eq!(s.issame(3, 0), true);
  assert_eq!(s.size(0), 5);
}
