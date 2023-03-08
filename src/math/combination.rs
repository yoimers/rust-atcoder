use cargo_snippet::snippet;

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

trait LexicalPermutation {
    fn prev_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T]
where
    T: Ord,
{
    fn prev_permutation(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] <= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        self[i..].reverse();
        let mut j = self.len() - 1;
        while j >= i && self[j - 1] < self[i - 1] {
            j -= 1;
        }
        self.swap(i - 1, j);
        true
    }
}
