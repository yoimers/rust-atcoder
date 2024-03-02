// fn main() {
//   input! {
//     n: usize,
//     q: usize,
//     c: [usize; n],
//     lr: [(usize, usize); q],
//   }
//   let lr = lr.iter().map(|&(v, w)| (v - 1, w)).collect_vec();

//   let hilbert = lr.iter().map(|&(l, r)| hilbert_order(l, r)).collect_vec();
//   let idx = (0..q).sorted_by_key(|&v| hilbert[v]).collect_vec();

//   let mut count = 0;
//   let add = |x: usize, cnt: &mut Vec<usize>, count: &mut usize| {
//     if cnt[c[x]] == 0 {
//       *count += 1;
//     }
//     cnt[c[x]] += 1;
//   };
//   let del = |x: usize, cnt: &mut Vec<usize>, count: &mut usize| {
//     cnt[c[x]] -= 1;
//     if cnt[c[x]] == 0 {
//       *count -= 1;
//     }
//   };
//   let mut now_l = 0;
//   let mut now_r = 0;
//   let mut cnt = vec![0; n + 1];

//   let mut ans = vec![0; q];
//   for i in 0..q {
//     while now_l > lr[idx[i]].0 {
//       now_l -= 1;
//       add(now_l, &mut cnt, &mut count);
//     }
//     while now_r < lr[idx[i]].1 {
//       add(now_r, &mut cnt, &mut count);
//       now_r += 1;
//     }
//     while now_l < lr[idx[i]].0 {
//       del(now_l, &mut cnt, &mut count);
//       now_l += 1;
//     }
//     while now_r > lr[idx[i]].1 {
//       now_r -= 1;
//       del(now_r, &mut cnt, &mut count);
//     }
//     ans[idx[i]] = count;
//   }
//   println!("{}", ans.iter().map(|v| v.to_string()).join("\n"));
// }

const LOG_N: usize = 20; //2^LOG_N > n
const MAX_N: usize = 1 << LOG_N;
fn hilbert_order(x: usize, y: usize) -> usize {
  let mut d = 0;
  let mut x = x;
  let mut y = y;
  for s in (0..LOG_N).rev() {
    let s_mask = 1 << s;
    let rx = x & s_mask > 0;
    let ry = y & s_mask > 0;

    d = (d << 2) | (rx as usize * 3) ^ (ry as usize);
    if ry {
      continue;
    }
    if rx {
      x = MAX_N - x;
      y = MAX_N - y;
    }
  }
  d
}
