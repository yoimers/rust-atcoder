fn get_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).ok();
  s.trim().to_string()
}
fn readln<T>() -> T
where
  T: std::str::FromStr,
  <T as std::str::FromStr>::Err: std::fmt::Debug,
{
  get_line().parse().unwrap()
}
