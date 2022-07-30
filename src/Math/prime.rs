use cargo_snippet::snippet;

#[snippet("prime")]
#[snippet("factors")]
fn factors(x: u64) -> Vec<u64> {
  if x <= 1 {
    return vec![];
  };
  let mut lst: Vec<u64> = Vec::new();
  let mut curn = x;
  loop {
    let m = firstfac(curn);
    lst.push(m);
    if m == curn {
      break;
    } else {
      curn /= m
    };
  }
  lst
}
#[snippet("prime")]
#[snippet("firstfac")]
fn firstfac(x: u64) -> u64 {
  if x % 2 == 0 {
    return 2;
  };
  // TODO: return to step_by
  // for n in (3..).step_by(2).take_while(|m| m*m <= x) {
  for n in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= x) {
    if x % n == 0 {
      return n;
    };
  }
  // No factor found. It must be prime.
  x
}

#[snippet("prime")]
#[snippet("is_prime")]
fn is_prime(n: u64) -> bool {
  if n <= 1 {
    return false;
  }
  firstfac(n) == n
}

#[snippet("divisor")]
fn divisor(n: u64) -> Vec<u64> {
  let res: Vec<u64> = Vec::new();
  let mut i = 1;
  while i * i <= n {
    if n % i == 0 {
      res.push(i);
      if i != n / i {
        res.push(n / i);
      }
    }
    i += 1;
  }
  res
}
//エラトステネスの篩
#[snippet("sieve")]
fn sieve(n: usize) -> usize {
  let mut is_prime = vec![true; n + 1];
  let mut p_count = 0;
  is_prime[0] = false;
  is_prime[1] = false;
  for i in 2..=n {
    if is_prime[i] {
      for j in 2..=n / i {
        is_prime[i * j] = false;
      }
      p_count += 1;
    }
  }
  p_count
}

#[snippet("pow")]
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
    m /= 2;
  }
  ret
}
