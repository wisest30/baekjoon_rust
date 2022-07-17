use std::io::Read;

fn dfs(
    cur: usize,
    visited: &mut Vec<bool>,
    capacity: &mut Vec<Vec<i32>>,
    flow: &mut Vec<Vec<i32>>,
    g: &Vec<Vec<usize>>,
    n: usize,
    m: usize,
) -> bool {
    if cur == n + m + 1 {
        true
    } else if visited[cur] {
        false
    } else {
        visited[cur] = true;
        for &nxt in &g[cur] {
            if flow[cur][nxt] < capacity[cur][nxt] {
                flow[cur][nxt] += 1;
                flow[nxt][cur] -= 1;

                if dfs(nxt, visited, capacity, flow, g, n, m) {
                    return true;
                }

                flow[cur][nxt] -= 1;
                flow[nxt][cur] += 1;
            }
        }

        false
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_whitespace();
    let n = input.next().unwrap().parse::<usize>().unwrap();
    let m = input.next().unwrap().parse::<usize>().unwrap();
    let node_cnt = n + m + 2;

    let mut capacity = vec![vec![0; node_cnt]; node_cnt];
    let mut flow = vec![vec![0; node_cnt]; node_cnt];
    let mut g = vec![vec![0usize; 0]; node_cnt];
    for u in 1..=n {
        let sz = input.next().unwrap().parse::<usize>().unwrap();
        for _ in 0..sz {
            let v = input.next().unwrap().parse::<usize>().unwrap() + n;
            capacity[u][v] = 1;
            g[u].push(v);
            g[v].push(u);
        }
        capacity[0][u] = 1;
        g[0].push(u);
        g[u].push(0);
    }

    for v in n + 1..node_cnt - 1 {
        capacity[v][node_cnt - 1] = 1;
        g[v].push(node_cnt - 1);
        g[node_cnt - 1].push(v);
    }

    let mut ans = 0;
    loop {
        let mut visited = vec![false; node_cnt];
        if dfs(0, &mut visited, &mut capacity, &mut flow, &g, n, m) {
            ans += 1;
        } else {
            break;
        }
    }

    println!("{}", ans);
}
