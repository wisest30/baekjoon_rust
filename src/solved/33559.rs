use std::io::{Read, Write};

struct Problem {
    _n: usize,
    a: Vec<i32>,
    b: Vec<i32>,
}

impl Problem {
    fn new(n: usize, a: Vec<i32>, b: Vec<i32>) -> Self {
        Self { _n: n, a, b }
    }

    fn solve(&mut self) -> String {
        let mut common_cnt = 0;
        let mut common_elem = Vec::new();
        let mut diff_elem_a = Vec::new();
        let mut diff_elem_b = Vec::new();

        self.a.sort_unstable();
        self.b.sort_unstable();

        while !self.a.is_empty() && !self.b.is_empty() {
            let ai = self.a.pop().unwrap();
            let bi = self.b.pop().unwrap();
            match ai.cmp(&bi) {
                std::cmp::Ordering::Less => {
                    self.a.push(ai);
                    diff_elem_b.push(bi);
                }

                std::cmp::Ordering::Equal => {
                    common_cnt += 1;
                    common_elem.push(ai);
                }
                std::cmp::Ordering::Greater => {
                    diff_elem_a.push(ai);
                    self.b.push(bi);
                }
            }
        }

        diff_elem_a.extend(self.a.iter());
        diff_elem_b.extend(self.b.iter());

        let mut a = Vec::new();
        a.extend(common_elem.iter());
        a.extend(diff_elem_a.iter());

        let mut b = Vec::new();
        b.extend(common_elem.iter());
        b.extend(diff_elem_b.iter());

        fn vec_to_string(v: &[i32]) -> String {
            v.iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        }

        format!(
            "{}\n{}\n{}",
            common_cnt,
            vec_to_string(&a),
            vec_to_string(&b)
        )
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let n = input.next().unwrap().parse::<usize>().unwrap();
    let a = (0..n)
        .map(|_| input.next().unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let b = (0..n)
        .map(|_| input.next().unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let ans = Problem::new(n, a, b).solve();
    writeln!(stdout, "{ans}").ok();
}
