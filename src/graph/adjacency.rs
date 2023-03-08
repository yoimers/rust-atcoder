use std::collections::VecDeque;

enum Graph {
    Undirected(UndirectedGraph),
    Directed(DirectedGraph),
}
struct UndirectedGraph {
    adj: Vec<Vec<usize>>,
    siz: usize,
}
struct DirectedGraph {
    adj: Vec<Vec<usize>>,
    siz: usize,
}
impl UndirectedGraph {
    pub fn new(n: usize, edge: &[(usize, usize)]) -> UndirectedGraph {
        let mut adj = vec![Vec::new(); n];
        for (a, b) in edge.iter() {
            adj[a - 1].push(b - 1);
            adj[b - 1].push(a - 1);
        }
        UndirectedGraph { adj, siz: n }
    }
    pub fn is_bipartite(&mut self) -> bool {
        let mut color = vec![-1; self.siz];
        let mut visit = vec![false; self.siz];
        let mut que: VecDeque<usize> = VecDeque::new();
        que.push_back(0);
        color[0] = 0;
        while !que.is_empty() {
            let v = que.pop_back().unwrap();
            visit[v] = true;
            for &ad in self.adj[v].iter() {
                if visit[ad] && color[ad] == color[v] {
                    return false;
                }
                if !visit[ad] && color[ad] == -1 {
                    color[ad] = 1 - color[v];
                    que.push_back(ad);
                }
            }
        }
        true
    }
    pub fn kruskal(&mut self) {}
}

#[test]
fn test_graph() {
    let mut graph = UndirectedGraph::new(3, &[(1, 2), (2, 3), (3, 1)]);
    assert_eq!(graph.is_bipartite(), false);
    let mut graph = UndirectedGraph::new(4, &[(1, 2), (2, 3), (3, 4), (4, 1)]);
    assert_eq!(graph.is_bipartite(), true);
    let mut graph = UndirectedGraph::new(4, &[(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)]);
    assert_eq!(graph.is_bipartite(), false);
}
