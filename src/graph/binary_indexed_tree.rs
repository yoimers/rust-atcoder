#[derive(Debug)]
struct BIT {
  size: usize,
  tree: Vec<i64>,
}

impl BIT {
  fn new(size: usize) -> Self {
    Self {
      size: size + 1,
      tree: vec![0; size + 1],
    }
  }
  //0-indexed
  fn add(&mut self, index: usize, value: i64) {
    let mut i = index + 1;
    while i < self.size {
      self.tree[i] += value;
      let ii = i as isize;
      i += (ii & -ii) as usize;
    }
  }

  //sum [0, index)
  fn sum(&mut self, index: usize) -> i64 {
    let mut retv = 0;
    let mut i = index.min(self.size - 1);
    while i > 0 {
      retv += self.tree[i];
      let ii = i as isize;
      i -= (ii & -ii) as usize;
    }
    retv
  }
}

#[test]
fn test_segment_tree() {
  let mut s = BIT::new(8);
  s.add(0, 0);
  s.add(1, 1);
  s.add(2, 2);
  s.add(4, 10);
  s.add(6, -10);
  s.add(7, 20);
  assert_eq!(s.sum(0), 0);
  assert_eq!(s.sum(1), 0);
  assert_eq!(s.sum(2), 1);
  assert_eq!(s.sum(3), 3);
  assert_eq!(s.sum(4), 3);
  assert_eq!(s.sum(5), 13);
  assert_eq!(s.sum(6), 13);
  assert_eq!(s.sum(7), 3);
  assert_eq!(s.sum(8), 23);
  assert_eq!(s.sum(18), 23);
  assert_eq!(s.sum(18000), 23);
}
