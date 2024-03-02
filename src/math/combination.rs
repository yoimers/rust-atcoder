use cargo_snippet::snippet;

use super::modint::Mod;

const MOD: usize = 998244353;
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
    res %= MOD;
  }
  let mut dev = 1;
  for i in 1..=r {
    dev *= i;
    dev %= MOD;
  }
  (res * pow(dev, MOD - 2)) % MOD
}
// let mut frac = vec![1; n + 1];
// let mut invfrac = vec![1; n + 1];
// for i in 0..n {
//     frac[i + 1] = frac[i] * (i + 1);
//     frac[i + 1] %= MOD;
//     invfrac[i + 1] = invfrac[i] * pow(i + 1, MOD - 2);
//     invfrac[i + 1] %= MOD;
// }
// let comb = |n: usize, r: usize| {
//     if n < r {
//         return 0;
//     }
//     ((frac[n] * invfrac[r]) % MOD) * invfrac[n - r] % MOD
// };
fn pow(x: usize, n: usize) -> usize {
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

fn binom() -> Vec<Vec<Mod>> {
  let mut binom = vec![vec![Mod::new(0); 5010]; 5010];
  for i in 0..5010 {
    binom[i][0] = Mod::new(1);
    for j in 1..=i {
      binom[i][j] = binom[i - 1][j - 1] + binom[i - 1][j];
    }
  }
  binom
}
