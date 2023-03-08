use std::collections::VecDeque;

use cargo_snippet::snippet;

#[snippet(Dinic)]
#[derive(Clone, Copy, Debug)]
struct Edge {
    to: usize,
    cap: i64,
    rev: usize,
}
struct Network {
    graph: Vec<Vec<Edge>>,
    level: Vec<Option<usize>>,
    iter: Vec<usize>,
}
impl Network {
    fn new(n: usize) -> Self {
        Self {
            graph: vec![vec![]; n],
            level: vec![None; n],
            iter: vec![0; n],
        }
    }
    fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        assert!(cap >= 0);
        let from_rev = self.graph[to].len();
        let to_rev = self.graph[from].len();
        self.graph[from].push(Edge {
            to,
            cap,
            rev: from_rev,
        });
        self.graph[to].push(Edge {
            to: from,
            cap: 0,
            rev: to_rev,
        });
    }
    fn n(&self) -> usize {
        self.graph.len()
    }
    //s からの距離を計算
    fn bfs(&mut self, s: usize) {
        self.level = vec![None; self.n()];
        let mut q = VecDeque::new();
        q.push_back(s);
        self.level[s] = Some(0);
        while let Some(v) = q.pop_front() {
            for &u in self.graph[v].iter() {
                if u.cap > 0 && self.level[u.to].is_none() {
                    self.level[u.to] = self.level[v].map(|x| x + 1);
                    q.push_back(u.to);
                }
            }
        }
    }
    fn dfs(&mut self, s: usize, t: usize, f: i64) -> i64 {
        if s == t {
            return f;
        }
        let iter_v_cur = self.iter[s];
        for i in iter_v_cur..self.graph[s].len() {
            let e = self.graph[s][i].clone();
            if e.cap > 0 && self.level[s] < self.level[e.to] {
                let d = self.dfs(e.to, t, f.min(e.cap));
                if d > 0 {
                    self.graph[s][i].cap -= d;
                    self.graph[e.to][e.rev].cap += d;
                    return d;
                }
            }
            self.iter[s] += 1;
        }
        0
    }
    fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0;
        loop {
            self.bfs(s);
            if self.level[t].is_none() {
                return flow;
            }
            let INF = 2_000_000_001;
            self.iter = vec![0; self.n()];
            let mut f = self.dfs(s, t, INF);
            while f > 0 {
                flow += f;
                f = self.dfs(s, t, INF)
            }
        }
    }
}

#[test]
fn test_dinic() {
    let mut nw = Network::new(5);

    let conns = [
        (0, 1, 10),
        (0, 2, 2),
        (1, 2, 6),
        (1, 3, 6),
        (3, 2, 3),
        (2, 4, 5),
        (3, 4, 8),
    ];

    for conn in &conns {
        nw.add_edge(conn.0, conn.1, conn.2);
    }
    assert_eq!(nw.max_flow(0, 4), 11);
}
