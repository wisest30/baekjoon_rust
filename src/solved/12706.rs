use std::io::{Read, Write};

struct Problem {
    n: i64,
    m: i64,
    a: i64,
}

impl Problem {
    fn new(n: i64, m: i64, a: i64) -> Self {
        Self { n, m, a }
    }

    fn solve(&mut self) -> String {
        let (mut n, mut m, a) = (self.n, self.m, self.a);
        let swap = if n > m {
            std::mem::swap(&mut n, &mut m);
            true
        } else {
            false
        };

        if a > n * m {
            "IMPOSSIBLE".to_string()
        } else if m == 1 {
            format!("0 0 0 1 1 0")
        } else {
            let r = (n - a % n) % n;
            let k = (a + r) / n;
            if swap {
                format!("0 0 {k} 1 {r} {n}")
            } else {
                format!("0 0 1 {k} {n} {r}")
            }
        }
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let test_case = input.next().unwrap().parse::<usize>().unwrap();
    (1..=test_case).for_each(|tc| {
        let n = input.next().unwrap().parse::<i64>().unwrap();
        let m = input.next().unwrap().parse::<i64>().unwrap();
        let a = input.next().unwrap().parse::<i64>().unwrap();
        let ans = Problem::new(n, m, a).solve();
        writeln!(stdout, "Case #{tc}: {ans}").ok();
    });
}
