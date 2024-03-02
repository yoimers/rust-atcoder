fn matrix_product(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
  let mut ret = vec![vec![0; a[0].len()]; a.len()];
  for i in 0..a.len() {
    for j in 0..a[0].len() {
      for k in 0..b[0].len() {
        ret[i][k] += a[i][j] * b[j][k];
      }
    }
  }
  ret
}
fn vector_product(a: &Vec<Vec<i64>>, b: &Vec<i64>) -> Vec<i64> {
  let mut ret = vec![0; a.len()];
  for i in 0..a.len() {
    for j in 0..b.len() {
      ret[i] += a[i][j] * b[j];
    }
  }
  ret
}
fn matrix_pow(x: &Vec<Vec<usize>>, k: usize) -> Vec<Vec<usize>> {
  let mut ret = vec![vec![0; x.len()]; x.len()];
  for i in 0..x.len() {
    ret[i][i] = 1;
  }
  let mut m = k;
  let mut y = x.clone();
  while m > 0 {
    if 1 & m == 1 {
      ret = matrix_product(&ret, &y);
    }
    y = matrix_product(&y, &y);
    m /= 2;
  }
  ret
}

struct Matrix {
  a: Vec<Vec<f64>>, // 拡大係数行列
}

struct MatrixInfo {
  rank: usize,
}

impl Matrix {
  const EPS: f64 = 1e-10;
  fn new(a: Vec<Vec<f64>>) -> Self {
    Matrix { a }
  }
  fn gauss_jordan(&mut self) -> MatrixInfo {
    let (m, n) = (self.a.len(), self.a[0].len());
    let mut rank = 0;
    for col in 0..n - 1 {
      let mut pivot = m;
      let mut ma = Self::EPS;
      for row in rank..m {
        if self.a[row][col].abs() > ma {
          ma = self.a[row][col].abs();
          pivot = row;
        }
      }
      if pivot == m {
        continue;
      }
      self.a.swap(rank, pivot);
      let fac = self.a[rank][col];
      for j in 0..n {
        self.a[rank][j] /= fac;
      }
      for row in (0..m).filter(|&x| x != rank) {
        if self.a[row][col] > Self::EPS {
          let fac = self.a[row][col];
          for j in 0..n {
            self.a[row][j] -= self.a[rank][j] * fac
          }
        }
      }
      rank += 1;
    }
    MatrixInfo { rank }
  }
}

struct MatrixXOR {
  a: Vec<Vec<usize>>, // 拡大係数行列 && 0,1 only
}

impl MatrixXOR {
  fn new(a: Vec<Vec<usize>>) -> Self {
    MatrixXOR { a }
  }
  fn gauss_jordan(&mut self) -> MatrixInfo {
    let (m, n) = (self.a.len(), self.a[0].len());
    let mut rank = 0;
    for col in 0..n - 1 {
      let Some(pivot) = (rank..m).find(|&row| self.a[row][col] == 1) else { continue; };
      self.a.swap(rank, pivot);

      for row in (0..m).filter(|&x| x != rank) {
        if self.a[row][col] == 1 {
          for j in 0..n {
            self.a[row][j] ^= self.a[rank][j]
          }
        }
      }
      rank += 1;
    }
    MatrixInfo { rank }
  }
}
use fixedbitset::FixedBitSet;
struct MatrixBitSet {
  bitsets: Vec<FixedBitSet>, // 拡大係数行列 && 0,1 only
}

impl MatrixBitSet {
  pub fn new(bitsets: Vec<FixedBitSet>) -> Self {
    Self { bitsets }
  }

  pub fn gauss_jordan(&mut self) -> MatrixInfo {
    if self.bitsets.is_empty() {
      return MatrixInfo { rank: 0 };
    }

    let (m, n) = (self.bitsets.len(), self.bitsets[0].len());

    let mut rank = 0;
    for col in 0..n - 1 {
      let Some(pivot) = (rank..m).find(|&row| self.bitsets[row][col]) else { continue; };
      self.bitsets.swap(rank, pivot);
      let pivot_bitset = self.bitsets[rank].clone();

      for row in (0..m).filter(|&x| x != rank) {
        if self.bitsets[row][col] {
          self.bitsets[row].symmetric_difference_with(&pivot_bitset);
        }
      }
      rank += 1;
    }

    MatrixInfo { rank }
  }
}
