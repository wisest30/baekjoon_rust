use std::io::{Read, Write};

struct Problem {
    a: i64,
    b: i64,
}

impl Problem {
    fn new(a: i64, b: i64) -> Self {
        Self { a, b }
    }

    fn solve(&mut self) -> i32 {
        let mut sa = self.a.to_string();
        let mut sb = self.b.to_string();

        if sa.len() < sb.len() {
            std::mem::swap(&mut sa, &mut sb);
        }
        sb = "1".repeat(sa.len() - sb.len()) + &sb;

        let x = sa
            .chars()
            .zip(sb.chars())
            .map(|(ai, bi)| {
                (
                    ai.to_digit(10).unwrap() as i64,
                    bi.to_digit(10).unwrap() as i64,
                )
            })
            .map(|(ai, bi)| ai * bi)
            .fold(0i64, |acc, x| {
                acc * if x >= 10 { 100 } else { 10 } + x as i64
            });

        if x == self.a * self.b {
            1
        } else {
            0
        }
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let n = input.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..n {
        let a = input.next().unwrap().parse::<i64>().unwrap();
        let b = input.next().unwrap().parse::<i64>().unwrap();
        let ans = Problem::new(a, b).solve();
        writeln!(stdout, "{ans}").ok();
    }
}
