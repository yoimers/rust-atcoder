// const MOD: i64 = 998244353;
// // const MOD: i64 = 1_000_000_007;

// use proconio::{input, source::line::LineSource};
// use std::io::BufReader;

// fn main() {
//   let stdin = std::io::stdin();
//   let mut stdin = LineSource::new(BufReader::new(stdin));
//   input! {
//       from &mut stdin,
//       n: i32
//   }
//   input! {
//       from &mut stdin,
//       a: i32
//   }
// }
fn read<T: std::str::FromStr>() -> T {
  let s = std::io::stdin();
  let s = s.lock();
  let s: String = std::io::Read::bytes(s)
    .map(|c| c.expect("failed reading char") as char)
    .skip_while(|c| c.is_whitespace())
    .take_while(|c| !c.is_whitespace())
    .collect();
  s.parse().ok().expect("failed parsing")
}
