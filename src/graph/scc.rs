use std::collections::VecDeque;

use cargo_snippet::snippet;
#[snippet("SCC")]
pub struct SCC {
  g: Vec<Vec<usize>>,
  r_g: Vec<Vec<usize>>,
  post_order: VecDeque<usize>,
  used: Vec<bool>,
  pub order: Vec<usize>,
}

#[snippet("SCC")]
impl SCC {
  pub fn new(n: usize) -> Self {
    Self {
      g: vec![vec![]; n],
      r_g: vec![vec![]; n],
      post_order: VecDeque::new(),
      used: vec![false; n],
      order: vec![n; n],
    }
  }
  pub fn add_edge(&mut self, u: usize, v: usize) {
    self.g[u].push(v);
    self.r_g[v].push(u);
  }
  fn dfs(&mut self, u: usize) {
    self.used[u] = true;
    for i in 0..self.g[u].len() {
      let v = self.g[u][i];
      if !self.used[v] {
        self.dfs(v);
      }
    }
    self.post_order.push_front(u);
  }
  fn rdfs(&mut self, u: usize, k: usize) {
    self.used[u] = true;
    self.order[u] = k;
    for i in 0..self.r_g[u].len() {
      let v = self.r_g[u][i];
      if !self.used[v] {
        self.rdfs(v, k);
      }
    }
  }
  pub fn build(&mut self) {
    for v in 0..self.g.len() {
      if !self.used[v] {
        self.dfs(v);
      }
    }
    // dbg!(&self.post_order);
    self.used = vec![false; self.g.len()];
    let mut k = 0;
    for i in 0..self.post_order.len() {
      let v = self.post_order[i];
      if !self.used[v] {
        self.rdfs(v, k);
        k += 1;
      }
    }
  }
  pub fn is_same(&self, u: usize, v: usize) -> bool {
    self.order[u] == self.order[v]
  }
  pub fn size(&self) -> Vec<usize> {
    let mut ret = vec![0; self.g.len()];
    for &i in self.order.iter() {
      ret[i] += 1;
    }
    ret
  }
}
