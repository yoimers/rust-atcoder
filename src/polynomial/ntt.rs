fn pow(x: i64, n: i64) -> i64 {
  let mut ret = 1;
  let mut m = n;
  let mut y = x;
  while m > 0 {
    if 1 & m == 1 {
      ret *= y;
      ret %= MOD;
    }
    y *= y;
    y %= MOD;
    m /= 2;
  }
  ret % MOD
}

#[test]
fn test_ntt() {
  let a = vec![1, 2];
  let b = vec![1, 2, 3];
  let c = ntt_multiply(&a, &b, 1_000_000_007);
  assert_eq!(c, vec![1, 4, 7, 6]);
}

pub fn modpow(x: i64, n: i64, m: i64) -> i64 {
  let mut res = 1;
  let mut x = x % m;
  let mut n = n;
  while n > 0 {
    if n & 1 == 1 {
      res = (res * x) % m;
    }
    x = (x * x) % m;
    n >>= 1;
  }
  res
}

pub fn modinv(a: i64, m: i64) -> i64 {
  let (_, x, _) = extgcd(a, m);
  (m + x % m) % m
}

pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
  if b == 0 {
    (a, 1, 0)
  } else {
    let (gcd, x, y) = extgcd(b, a % b);
    (gcd, y, x - (a / b) * y)
  }
}

pub fn garner(rm: Vec<(i64, i64)>, mo: i64) -> i64 {
  let mut rm = rm;
  rm.push((0, mo));
  let mut coef = vec![1; rm.len()];
  let mut constants = vec![0; rm.len()];
  for i in 0..rm.len() - 1 {
    let v = (rm[i].0 + rm[i].1 - constants[i]) * modinv(coef[i], rm[i].1) % rm[i].1;
    for j in i + 1..rm.len() {
      constants[j] += coef[j] * v;
      constants[j] %= rm[j].1;
      coef[j] *= rm[i].1;
      coef[j] %= rm[j].1;
    }
  }
  constants[rm.len() - 1]
}

struct NTT {
  pub mo: i64,
}
impl NTT {
  pub fn new(mo: i64) -> NTT {
    NTT { mo: mo }
  }
  fn _ntt(&self, a: &mut [i64], n: usize, inverse: bool) {
    let g = 3;
    let mut h = modpow(g, (self.mo - 1) / n as i64, self.mo);
    if inverse {
      h = modinv(h, self.mo);
    }

    let mut i = 0;
    for j in 1..n - 1 {
      let mut k = n >> 1;
      loop {
        i ^= k;
        if k > i {
          k >>= 1;
        } else {
          break;
        }
      }
      if j < i {
        let tmp = a[i];
        a[i] = a[j];
        a[j] = tmp;
      }
    }

    let mut m = 1;
    while m < n {
      let m2 = m * 2;
      let base = modpow(h, (n / m2) as i64, self.mo);
      let mut w = 1;
      for x in 0..m {
        let mut s = x;
        while s < n {
          let u = a[s];
          let d = (a[s + m] * w) % self.mo;
          a[s] = u + d;
          if a[s] >= self.mo {
            a[s] -= self.mo;
          }
          a[s + m] = u - d;
          if a[s + m] < 0 {
            a[s + m] += self.mo;
          }
          s += m2;
        }
        w = (w * base) % self.mo;
      }
      m *= 2;
    }
    for i in 0..n {
      if a[i] < 0 {
        a[i] += self.mo;
      }
    }
  }
  fn ntt(&self, a: &mut [i64], n: usize) {
    self._ntt(a, n, false);
  }
  fn intt(&self, a: &mut [i64], n: usize) {
    self._ntt(a, n, true);
    let n_inv = modinv(a.len() as i64, self.mo);
    for i in 0..n {
      a[i] = (a[i] * n_inv) % self.mo;
    }
  }
  pub fn convolve(&self, a: &[i64], b: &[i64]) -> Vec<i64> {
    let mut a = a.to_vec();
    let mut b = b.to_vec();

    let mut n = 1;
    while n < a.len() + b.len() {
      n <<= 1;
    }
    a.resize(n, 0);
    b.resize(n, 0);

    self.ntt(&mut a, n);
    self.ntt(&mut b, n);

    let mut c = vec![0; n];
    for i in 0..n {
      c[i] = (a[i] * b[i]) % self.mo;
    }

    self.intt(&mut c, n);
    c
  }
}
pub fn ntt_multiply_naive(a: &[i64], b: &[i64], mo: i64) -> Vec<i64> {
  let mut a = a.to_vec();
  let mut b = b.to_vec();
  let n = a.len();
  let m = b.len();
  for i in 0..n {
    a[i] %= mo;
  }
  for i in 0..m {
    b[i] %= mo;
  }
  let ntt1 = NTT::new(167772161);
  let ntt2 = NTT::new(469762049);
  let ntt3 = NTT::new(1224736769);

  let x = ntt1.convolve(&a, &b);
  let y = ntt2.convolve(&a, &b);
  let z = ntt3.convolve(&a, &b);

  let L = x.len();
  let mut res = vec![0; L];
  for i in 0..L {
    let rm = vec![(x[i], ntt1.mo), (y[i], ntt2.mo), (z[i], ntt3.mo)];
    res[i] = garner(rm, mo);
  }
  res.truncate(n + m - 1);
  res
}

pub fn ntt_multiply(a: &[i64], b: &[i64], mo: i64) -> Vec<i64> {
  let mut a = a.to_vec();
  let mut b = b.to_vec();
  let n = a.len();
  let m = b.len();
  for i in 0..n {
    a[i] %= mo;
  }
  for i in 0..m {
    b[i] %= mo;
  }
  let ntt1 = NTT::new(167772161);
  let ntt2 = NTT::new(469762049);
  let ntt3 = NTT::new(1224736769);

  let x = ntt1.convolve(&a, &b);
  let y = ntt2.convolve(&a, &b);
  let z = ntt3.convolve(&a, &b);

  let m1 = ntt1.mo;
  let m2 = ntt2.mo;
  let m3 = ntt3.mo;
  let m1_inv_m2 = modinv(m1, m2);
  let m12_inv_m3 = modinv(m1 * m2, m3);
  let m12_mod = (m1 * m2) % mo;

  let L = x.len();
  let mut res = vec![0; L];
  for i in 0..L {
    let mut v1 = (y[i] - x[i]) * m1_inv_m2;
    v1 %= m2;
    if v1 < 0 {
      v1 += m2;
    }

    let mut v2 = (z[i] - (x[i] + m1 * v1) % m3) * m12_inv_m3;
    v2 %= m3;
    if v2 < 0 {
      v2 += m3;
    }

    let mut const3 = (x[i] + m1 * v1 + m12_mod * v2) % mo;
    if const3 < 0 {
      const3 += mo;
    }
    res[i] = const3;
  }
  res.truncate(n + m - 1);
  res
}
