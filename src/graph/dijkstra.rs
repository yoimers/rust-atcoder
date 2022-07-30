fn main() {}
use std::collections::BinaryHeap;
const MAX: i64 = std::i64::MAX;
struct Dijkstra {
  adj: Vec<Vec<Edge>>,
  weitedadj: Vec<Vec<i64>>,
  siz: usize,
}
#[derive(Clone, Copy)]
struct Edge {
  from: usize,
  to: usize,
  cost: i64,
}

impl Dijkstra {
  pub fn new(n: usize, edge: &[Edge]) -> Dijkstra {
    let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); n];
    let mut weitedadj: Vec<Vec<i64>> = vec![vec![MAX / 2; n]; n];
    for &e in edge.iter() {
      adj[e.from - 1].push(Edge {
        from: e.from - 1,
        to: e.to - 1,
        cost: e.cost,
      });
      weitedadj[e.from - 1][e.to - 1] = e.cost;
    }
    for i in 0..n {
      weitedadj[i][i] = 0;
    }
    Dijkstra {
      adj,
      siz: n,
      weitedadj,
    }
  }
  pub fn dijkstra(&mut self, s: usize) -> (Vec<i64>, Vec<i64>) {
    let mut que: BinaryHeap<(i64, usize)> = BinaryHeap::new();
    let mut prev: Vec<i64> = vec![-1; self.siz];
    let mut d: Vec<i64> = vec![MAX; self.siz];
    d[s] = 0;
    que.push((0, s));
    while !que.is_empty() {
      let (distance, vertex) = que.pop().unwrap();
      let distance = -distance;
      if d[vertex] < distance {
        continue;
      }
      for &e in self.adj[vertex].iter() {
        if d[e.to] > d[vertex] + e.cost {
          d[e.to] = d[vertex] + e.cost;
          prev[e.to] = vertex as i64;
          que.push((-d[e.to], e.to));
        }
      }
    }

    (d, prev)
  }
  pub fn get_path(&mut self, s: usize, t: usize) -> Vec<i64> {
    let mut path = Vec::new();
    path.push(t as i64);
    let prev = self.dijkstra(s).1;
    let mut t = prev[t] as i64;
    while t != -1 {
      path.push(t);
      t = prev[t as usize];
    }
    path.reverse();
    path
  }
  pub fn warshall_floyed(&mut self) -> Vec<Vec<i64>> {
    let n = self.siz;
    let mut d: Vec<Vec<i64>> = self.weitedadj.clone();
    for k in 0..n {
      for i in 0..n {
        for j in 0..n {
          d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
        }
      }
    }
    d
  }
}

#[test]
fn test_dijkstra() {
  let mut edge = [
    (1, 2, 2),
    (1, 3, 5),
    (3, 2, 4),
    (3, 4, 2),
    (2, 4, 6),
    (4, 6, 1),
    (2, 5, 10),
    (6, 5, 3),
    (5, 7, 5),
    (6, 7, 9),
  ];
  let mut dik = Dijkstra::new(
    7,
    &edge
      .iter()
      .map(|&(from, to, cost)| Edge { from, to, cost })
      .collect::<Vec<Edge>>(),
  );
  assert_eq!(dik.dijkstra(0).0, [0, 2, 5, 7, 11, 8, 16]);
  assert_eq!(dik.get_path(0, 6), [0, 2, 3, 5, 4, 6]);
  assert_eq!(dik.dijkstra(1).0, [MAX, 0, MAX, 6, 10, 7, 15]);
  assert_eq!(
    dik.warshall_floyed(),
    [
      [0, 2, 5, 7, 11, 8, 16],
      [MAX / 2, 0, MAX / 2, 6, 10, 7, 15],
      [MAX / 2, 4, 0, 2, 6, 3, 11],
      [MAX / 2, MAX / 2, MAX / 2, 0, 4, 1, 9],
      [MAX / 2, MAX / 2, MAX / 2, MAX / 2, 0, MAX / 2, 5],
      [MAX / 2, MAX / 2, MAX / 2, MAX / 2, 3, 0, 8],
      [MAX / 2, MAX / 2, MAX / 2, MAX / 2, MAX / 2, MAX / 2, 0],
    ]
  );
}
