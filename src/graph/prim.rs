struct Prim {}

struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}
impl Prim {
    pub fn new(n: usize, edge: &[Edge]);
    pub fn prim(&mut self) -> i64 {
        let n = self.siz;
        let mut mincost = vec![MAX; n];
        let mut used = vec![false; n];
        mincost[0] = 0;
        let mut res = 0;
        loop {
            let mut v = -1;
            for i in 0..n {
                if !used[i] && (v == -1 || mincost[i] < mincost[v]) {
                    v = i
                }
            }
            if v == -1 {
                break;
            }
            used[v] = true;
            res += mincost[v];
            for u in 0..n {
                mincost[u] = mincost[u].min(cost[v][u])
            }
        }
        res
    }
    pub fn kruskal(&mut self) {}
}
