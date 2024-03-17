use std::io::{Read, Write};

const INVALID_ANSWER: &str = "-1";
const INVALID_NODE_INDEX: usize = 123456;
struct Problem {
    n: usize,
    m: usize,
    source: usize,
    sync: usize,
    g: Vec<Vec<usize>>,
    capacity: Vec<Vec<i32>>,
    flow: Vec<Vec<i32>>,
}

impl Problem {
    fn new(n: usize, m: usize, a: Vec<i32>, b: Vec<i32>) -> Self {
        let node_cnt = n + m + 2;
        let mut g = vec![Vec::new(); node_cnt];
        let source = node_cnt - 2;
        let sync = node_cnt - 1;
        let mut capacity = vec![vec![0; node_cnt]; node_cnt];
        for i in 0..n {
            g[source].push(i);
            capacity[source][i] = a[i];
            for j in n..n + m {
                g[i].push(j);
                g[j].push(i);
                capacity[i][j] = 1;
            }
        }
        for i in n..n + m {
            g[i].push(sync);
            capacity[i][sync] = b[i - n];
        }
        Problem {
            n,
            m,
            source,
            sync,
            g,
            capacity,
            flow: vec![vec![0; node_cnt]; node_cnt],
        }
    }

    fn solve(&mut self) -> String {
        if !self.invalid_initial_state() {
            return INVALID_ANSWER.to_string();
        }
        if self.solve_max_flow() != self.capacity_from_sum(self.source) {
            return INVALID_ANSWER.to_string();
        }
        self.solve_order_problem();
        self.make_answer()
    }

    fn invalid_initial_state(&self) -> bool {
        self.capacity_from_sum(self.source) == self.capacity_to_sum(self.sync)
    }

    fn capacity_from_sum(&self, i: usize) -> i32 {
        self.capacity[i].iter().sum::<i32>()
    }

    fn capacity_to_sum(&self, i: usize) -> i32 {
        self.capacity.iter().fold(0, |acc, x| acc + x[i])
    }

    fn solve_max_flow(&mut self) -> i32 {
        let mut cnt = 0;
        while self.push_flow(self.source, self.sync, false) {
            cnt += 1
        }
        cnt
    }

    fn push_flow(&mut self, from: usize, to: usize, reorder: bool) -> bool {
        let mut st = vec![from];
        let mut prev = vec![INVALID_NODE_INDEX; self.node_cnt()];
        while !st.is_empty() && prev[to] == INVALID_NODE_INDEX {
            let cur = st.pop().unwrap();
            for &nxt in &self.g[cur] {
                if !self.can_flow(cur, nxt) {
                    continue;
                }
                if prev[nxt] != INVALID_NODE_INDEX {
                    continue;
                }
                if reorder {
                    if nxt < from || (cur == from && nxt <= to) {
                        continue;
                    }
                }
                prev[nxt] = cur;
                st.push(nxt);
            }
        }

        if prev[to] == INVALID_NODE_INDEX {
            return false;
        }

        let mut cur = to;
        while cur != from {
            self.add_flow(prev[cur], cur);
            cur = prev[cur];
        }

        true
    }

    fn node_cnt(&self) -> usize {
        self.g.len()
    }

    fn can_flow(&self, from: usize, to: usize) -> bool {
        self.flow[from][to] < self.capacity[from][to]
    }

    fn add_flow(&mut self, from: usize, to: usize) {
        self.flow[from][to] += 1;
        self.flow[to][from] -= 1;
    }

    fn solve_order_problem(&mut self) {
        for i in 0..self.n {
            for j in self.n..self.n + self.m {
                if self.flow[i][j] == 1 {
                    if self.push_flow(i, j, true) {
                        self.add_flow(j, i);
                    }
                }
            }
        }
    }

    fn make_answer(&self) -> String {
        let mut ret = String::new();
        for i in 0..self.n {
            for j in self.n..self.n + self.m {
                ret.push(if self.flow[i][j] == 1 { '1' } else { '0' });
            }
            ret.push('\n');
        }
        ret
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let n = input.next().unwrap().parse::<usize>().unwrap();
    let m = input.next().unwrap().parse::<usize>().unwrap();
    let a = (0..n)
        .map(|_| input.next().unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let b = (0..m)
        .map(|_| input.next().unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let ans = Problem::new(n, m, a, b).solve();

    writeln!(stdout, "{}", ans).ok();
}
