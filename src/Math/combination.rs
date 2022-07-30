use cargo_snippet::snippet;

#[snippet("combination")]
fn combination(n: usize, r: usize) -> usize {
  if n < r {
    return 0;
  }
  if r == 0 {
    return 1;
  }
  let mut res = 1;
  for i in 0..r {
    res *= n - i;
    res %= MD;
  }
  let mut dev = 1;
  for i in 1..=r {
    dev *= i;
    dev %= MD;
  }
  (res * pow(dev, MD - 2)) % MD
}
fn pow(x: usize, n: usize) -> usize {
  let mut ret = 1;
  let mut m = n;
  let mut y = x;
  while m > 0 {
    if 1 & m == 1 {
      ret *= y;
      ret %= 1_000_000_007;
    }
    y *= y;
    y %= 1_000_000_007;
    m /= 2;
  }
  ret
}
