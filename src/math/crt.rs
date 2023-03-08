pub fn modinv(a: usize, modulo: usize) -> usize {
  let mut a = a as i64;
  let modulo = modulo as i64;
  let mut b = modulo;
  let mut u = 1;
  let mut v = 0;
  while b > 0 {
    let t = a / b;
    a -= t * b;
    std::mem::swap(&mut a, &mut b);
    u -= t * v;
    std::mem::swap(&mut u, &mut v);
  }
  u %= modulo;
  if u < 0 {
    u += modulo;
  }
  u as usize
}

fn gauss_crt(rm: &Vec<(usize, usize)>) -> (usize, usize) {
  let mm = rm.iter().map(|(_, m)| m).product::<usize>();
  let mut retv = 0;
  for &(r, m) in rm.iter() {
    let mi = mm / m;
    let mi_inv = modinv(mi, m);
    retv += r * mi * mi_inv;
    retv %= mm;
  }
  (retv % mm, mm)
}

#[cfg(test)]
mod test {
  use crate::math::crt::gauss_crt;
  #[test]
  fn crt() {
    let rm = vec![(2, 3), (3, 5), (3, 7)];
    let (a, m) = gauss_crt(&rm);
    assert_eq!(m, 105);
    assert_eq!(a, 38);

    let rm = vec![(1, 3), (1, 5), (1, 7), (10, 16)];
    let (a, m) = gauss_crt(&rm);
    assert_eq!(m, 1680);
    assert_eq!(a, 106);
    assert_eq!(106 % 3, 1);
    assert_eq!(106 % 5, 1);
    assert_eq!(106 % 7, 1);
    assert_eq!(106 % 16, 10);
  }
}
