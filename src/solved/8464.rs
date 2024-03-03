use std::io::{Read, Write};

struct Problem {
    n: usize,
    mobius: Vec<i64>,
}

impl Problem {
    fn new(n: usize) -> Self {
        Problem {
            n,
            mobius: mobius_function(((n as f64 * 4.0).sqrt() + 10.0) as usize),
        }
    }

    fn solve(&mut self) -> usize {
        let mut l = 0usize;
        let mut r = self.n * 4 + 10;
        while l + 1 < r {
            let m = (l + r) / 2;
            if m - self.squre_free_count(m) < self.n {
                l = m;
            } else {
                r = m;
            }
        }
        r
    }

    fn squre_free_count(&self, n: usize) -> usize {
        let mut ret = 0i64;
        for i in 1i64.. {
            if i * i > n as i64 {
                break;
            }
            ret += n as i64 / (i * i) * self.mobius[i as usize];
        }
        ret as usize
    }
}

fn mobius_function(n: usize) -> Vec<i64> {
    let mut mobius_function = vec![0; n + 1];
    mobius_function[1] = 1;
    for i in 1..=n {
        for j in (2 * i..=n).step_by(i) {
            mobius_function[j] -= mobius_function[i];
        }
    }
    mobius_function
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let n = input.next().unwrap().parse::<usize>().unwrap();
    let ans = Problem::new(n).solve();

    writeln!(stdout, "{}", ans).ok();
}
