use std::ops::Add;

use cargo_snippet::snippet;

#[snippet("CoordCompression")]
pub struct CoordCompression {
  comp: std::collections::HashMap<i64, usize>,
  dcmp: std::collections::HashMap<usize, i64>,
}

#[snippet("CoordCompression")]
impl CoordCompression {
  pub fn new(xs: &[i64], start: usize, step: usize) -> CoordCompression {
    let mut xs = xs.to_owned();
    xs.sort();
    let mut comp = std::collections::HashMap::new();
    let mut dcmp = std::collections::HashMap::new();
    let mut acc = start;
    for x in xs {
      if comp.contains_key(&x) {
        continue;
      }
      comp.insert(x, acc);
      dcmp.insert(acc, x);
      acc += step;
    }
    CoordCompression { comp, dcmp }
  }
  pub fn compress(&self, x: i64) -> usize {
    *self.comp.get(&x).unwrap()
  }
  pub fn decompress(&self, x: usize) -> i64 {
    *self.dcmp.get(&x).unwrap()
  }
  pub fn n(&self) -> usize {
    self.comp.len()
  }
}

#[test]
fn test_coord_compression() {
  let v = vec![-2, 3, 99999, 1000];
  let cc = CoordCompression::new(&v, 0, 1);
  assert_eq!(cc.compress(-2), 0);
  assert_eq!(cc.compress(1000), 2);
  assert_eq!(cc.decompress(1), 3);
  assert_eq!(cc.decompress(3), 99999);
}

fn compress_vec<T: PartialEq + Copy>(input_vec: Vec<T>) -> Vec<(T, usize)> {
  let mut retv = Vec::new();
  let mut iter = input_vec.iter().peekable();
  while let Some(&element) = iter.next() {
    let mut count = 1;
    while let Some(&next_element) = iter.peek() {
      if element == *next_element {
        count += 1;
        iter.next();
      } else {
        break;
      }
    }
    retv.push((element, count));
  }
  retv
}
