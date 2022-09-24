use std::io::Read;

struct DisjointSet {
    n: usize,
    g: Vec<i32>,
    g_set: std::collections::HashSet<i32>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        DisjointSet {
            n,
            g: (0..n as i32).collect::<Vec<_>>(),
            g_set: (0..n as i32).collect::<std::collections::HashSet<_>>(),
        }
    }

    pub fn group(&mut self, x: i32) -> i32 {
        if self.g[x as usize] == x {
            x
        } else {
            self.g[x as usize] = self.group(self.g[x as usize]);
            self.g[x as usize]
        }
    }

    pub fn merge(&mut self, x: i32, y: i32) -> bool {
        let gx = self.group(x);
        let gy = self.group(y);

        if gx == gy {
            false
        } else {
            self.g[gy as usize] = gx;
            self.g_set.remove(&gy);
            true
        }
    }

    pub fn get_group_set(&self) -> &std::collections::HashSet<i32> {
        &self.g_set
    }
}

pub fn get_primes(n: usize) -> Vec<i64> {
    let mut v = vec![true; n];
    let mut ret = Vec::new();
    for i in 2..n {
        if v[i] {
            ret.push(i as i64);
            for j in (2 * i..n).step_by(i) {
                v[j] = false;
            }
        }
    }

    ret
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let test_case = input.next().unwrap().parse::<usize>().unwrap();

    let primes = get_primes(1000001);
    for tc in 1..=test_case {
        let a = input.next().unwrap().parse::<i64>().unwrap();
        let b = input.next().unwrap().parse::<i64>().unwrap();
        let p = input.next().unwrap().parse::<i64>().unwrap();

        let mut ds = DisjointSet::new((b - a) as usize + 1);
        for &prime in &primes {
            if prime < p {
                continue;
            } else {
                let first = if a % prime == 0 {
                    a
                } else {
                    a + (prime - a % prime)
                };
                for i in (first + prime..=b).step_by(prime as usize) {
                    ds.merge((first - a) as i32, (i - a) as i32);
                }
            }
        }

        let ans = ds.get_group_set().len();
        println!("Case #{}: {}", tc, ans);
    }
}
