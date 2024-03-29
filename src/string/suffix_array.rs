use cargo_snippet::snippet;

/// 各suffixを列挙し、その辞書順
/// suffix -> 順番
/// を構築する。
///
/// 構築計算量 O(|S| log |S|)
///
/// ある文字列Tがどれかのsuffixとprefixが一致するとき、
/// マッチングが検索出来ていることになる。
/// 従って、suffixについて二分探索をすることによって、
/// マッチングしたインデックスを調べることが出来る。
///
/// 計算量 O(|T| log |S|)

#[snippet("SuffixArray")]
struct SuffixArray {
  // sのうち前からSA[i]個消したやつが辞書順i番目のsuffixである
  sa: Vec<usize>,
  s: Vec<u64>,
}
#[snippet("SuffixArray")]
impl SuffixArray {
  pub fn new(s: Vec<u64>) -> Self {
    let mut s = s;
    s.push('$' as u64);
    let mut sa = Self::sort_cyclic_shifts(&s);
    sa.remove(0);
    s.remove(s.len() - 1);
    SuffixArray { sa: sa, s: s }
  }
  fn sort_cyclic_shifts(s: &[u64]) -> Vec<usize> {
    let n = s.len();
    const alphabet: usize = 256;
    let mut p = vec![0; n];
    let mut c = vec![0; n];
    let mut cnt = vec![0; std::cmp::max(alphabet, n)];

    for i in 0..n {
      cnt[s[i] as usize] += 1;
    }
    for i in 1..alphabet {
      cnt[i] += cnt[i - 1];
    }
    for i in 0..n {
      cnt[s[i] as usize] -= 1;
      p[cnt[s[i] as usize]] = i;
    }
    c[p[0]] = 0;
    let mut classes = 1;
    for i in 1..n {
      if s[p[i]] != s[p[i - 1]] {
        classes += 1;
      }
      c[p[i]] = classes - 1;
    }

    let mut pn = vec![0; n];
    let mut cn = vec![0; n];
    for k in 0.. {
      // OK in 1.15
      if (1 << k) >= n {
        break;
      }

      for i in 0..n {
        if p[i] >= (1 << k) {
          pn[i] = p[i] - (1 << k);
        } else {
          pn[i] = p[i] + n - (1 << k);
        }
      }
      for i in 0..classes {
        cnt[i] = 0;
      }
      for i in 0..n {
        cnt[c[pn[i]]] += 1;
      }
      for i in 1..classes {
        cnt[i] += cnt[i - 1];
      }
      for i in (0..n).rev() {
        cnt[c[pn[i]]] -= 1;
        p[cnt[c[pn[i]]]] = pn[i];
      }
      cn[p[0]] = 0;
      classes = 1;
      for i in 1..n {
        let cur = (c[p[i]], c[(p[i] + (1 << k)) % n]);
        let prev = (c[p[i - 1]], c[(p[i - 1] + (1 << k)) % n]);
        if cur != prev {
          classes += 1;
        }
        cn[p[i]] = classes - 1;
      }
      let tmp = c;
      c = cn;
      cn = tmp
    }
    p
  }
  // 文字列比較をする。バイナリサーチのために必要
  // sの方が辞書順で前ならばtrue
  // O(m)
  fn lt_substr(s: &[u64], t: &[u64], si: usize, ti: usize) -> bool {
    let mut si = si;
    let mut ti = ti;
    let sn = s.len();
    let tn = t.len();
    while si < sn && ti < tn {
      if s[si] < t[ti] {
        return true;
      }
      if s[si] > t[ti] {
        return false;
      }
      si += 1;
      ti += 1;
    }
    si >= sn && ti < tn
  }
  #[doc = "find the rightmost match of the string t to s. O(mlogn) where n=|s|,m=|t|"]
  fn lower_bound(&self, t: &[u64]) -> usize {
    let mut low: i64 = -1;
    let mut high: i64 = self.sa.len() as i64;
    while high - low > 1 {
      let mid = (low + high) / 2;
      if Self::lt_substr(&self.s, t, self.sa[mid as usize], 0) {
        low = mid;
      } else {
        high = mid;
      }
    }
    return high as usize;
  }

  pub fn right_most_index(&self, t: &[u64]) -> usize {
    self.sa[self.lower_bound(t)]
  }
}

fn as_v(s: &str) -> Vec<u64> {
  let mut v = vec![];
  for c in s.chars() {
    v.push(c as u64);
  }
  v
}
#[test]
fn test_lt_substr() {
  assert_eq!(SuffixArray::lt_substr(&as_v("abc"), &as_v("abd"), 0, 0), true);
  assert_eq!(SuffixArray::lt_substr(&as_v("abd"), &as_v("abc"), 0, 0), false);
  assert_eq!(SuffixArray::lt_substr(&as_v("abc"), &as_v("abcd"), 0, 0), true);
  assert_eq!(SuffixArray::lt_substr(&as_v("abc"), &as_v("bcd"), 0, 0), true);
  assert_eq!(SuffixArray::lt_substr(&as_v("abc"), &as_v("abc"), 0, 0), false);
}
#[test]
fn test_suffix_array() {
  let s = "abracadabra";
  let sa = SuffixArray::new(as_v(s));
  assert_eq!(sa.sa, [10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);

  assert_eq!(sa.right_most_index(&as_v("rac")), 2);
  assert_eq!(sa.right_most_index(&as_v("bra")), 8);
  assert_eq!(sa.right_most_index(&as_v("abra")), 7);
  assert_eq!(sa.right_most_index(&as_v("abr")), 7);
  assert_eq!(sa.right_most_index(&as_v("a")), 10);
}
