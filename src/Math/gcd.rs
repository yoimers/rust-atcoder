use cargo_snippet::snippet;

#[snippet("gcd")]
fn gcd(a: i64, b: i64) -> i64 {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}
#[snippet("lcd")]
fn lcm(a: i64, b: i64) -> i64 {
  a * b / gcd(a, b)
}
#[snippet("extgcd")]
fn extgcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
  let mut d = a;
  if b != 0 {
    d = extgcd(b, a % b, y, x);
    *y -= (a / b) * (*x);
  } else {
    *x = 1;
    *y = 0;
  }
  d
}
