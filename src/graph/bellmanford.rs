const MAX: i64 = std::i64::MAX;

struct BellmanFord {}

impl BellmanFord {
  pub fn shortest_path(v: usize, n: usize, edge: &[(usize, usize, i64)]) -> Vec<i64> {
    let mut d: Vec<i64> = vec![MAX; n];
    d[v] = 0;
    loop {
      let mut update = false;
      for e in edge.iter() {
        if d[e.0] != MAX && d[e.1] > d[e.0] + e.2 {
          d[e.1] = d[e.0] + e.2;
          update = true;
        }
      }
      if !update {
        break;
      }
    }
    d
  }
  pub fn find_negative_loop(v: usize, n: usize, edge: &[(usize, usize, i64)]) -> bool {
    let mut d: Vec<i64> = vec![0; n];
    for i in 0..n {
      for e in edge.iter() {
        if d[e.1] > d[e.0] + e.2 {
          d[e.1] = d[e.0] + e.2;
          if i == n - 1 {
            return true;
          }
        }
      }
    }
    false
  }
}
