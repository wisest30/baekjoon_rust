use std::{
    collections::VecDeque,
    io::{Read, Write},
};

#[derive(Debug, Clone)]
struct NegativeSegment<'a> {
    a: &'a Vec<i64>,
    elems: Vec<i64>,
    idx: usize,
}

impl NegativeSegment<'_> {
    pub fn len(&self) -> usize {
        self.elems.len()
    }
    pub fn left(&self) -> i64 {
        if self.idx > 0 {
            self.a[self.idx - 1]
        } else {
            0
        }
    }
    pub fn right(&self) -> i64 {
        if self.idx + self.len() < self.a.len() {
            self.a[self.idx + self.len()]
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
    let test_cases = input.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..test_cases {
        let n = input.next().unwrap().parse::<usize>().unwrap();
        let a = (0..n)
            .map(|_| input.next().unwrap().parse::<i64>().unwrap())
            .filter(|&x| x != 0)
            .collect::<Vec<_>>();
        let n = a.len();

        let mut pos_cnt = 0;
        let mut ns = Vec::new();
        for i in 0..n {
            if a[i] > 0 {
                pos_cnt += 1;
            } else if a[i] < 0 {
                if i == 0 || a[i - 1] > 0 {
                    ns.push(NegativeSegment {
                        a: &a,
                        elems: Vec::new(),
                        idx: i,
                    });
                }
                ns.last_mut().unwrap().elems.push(a[i]);
            }
        }

        if pos_cnt > ns.len() {
            writeln!(stdout, "YES").ok();
            continue;
        }

        let mut can_remove = Vec::new();
        for i in 0..ns.len() {
            let mut left = ns[i].left();
            let mut right = ns[i].right();
            let mut dq = VecDeque::from(ns[i].elems.clone());
            while !dq.is_empty() && left + dq[0] > 0 {
                left += dq.pop_front().unwrap();
            }
            while !dq.is_empty() && right + dq.back().unwrap() > 0 {
                right += dq.pop_back().unwrap();
            }
            if dq.is_empty() {
                can_remove.push(ns[i].clone());
            }
        }

        if pos_cnt == ns.len() && !can_remove.is_empty() {
            writeln!(stdout, "YES").ok();
        } else if pos_cnt + 1 == ns.len() {
            if can_remove.len() > 2 {
                writeln!(stdout, "YES").ok();
            } else if can_remove.len() == 2 {
                if can_remove[0].idx + can_remove[0].elems.len() + 1 == can_remove[1].idx {
                    let mut left = can_remove[0].left();
                    let mut mid = can_remove[0].right();
                    let mut right = can_remove[1].right();
                    let mut dq0 = VecDeque::from(can_remove[0].elems.clone());
                    let mut dq1 = VecDeque::from(can_remove[1].elems.clone());

                    while !dq0.is_empty() && left + dq0[0] > 0 {
                        left += dq0.pop_front().unwrap();
                    }
                    while !dq1.is_empty() && right + dq1.back().unwrap() > 0 {
                        right += dq1.pop_back().unwrap();
                    }
                    while !dq0.is_empty() && mid + dq0.back().unwrap() > 0 {
                        mid += dq0.pop_back().unwrap();
                    }
                    while !dq1.is_empty() && mid + dq1.back().unwrap() > 0 {
                        mid += dq1.pop_back().unwrap();
                    }

                    if dq0.is_empty() && dq1.is_empty() {
                        writeln!(stdout, "YES").ok();
                    } else {
                        writeln!(stdout, "NO").ok();
                    }
                } else {
                    writeln!(stdout, "YES").ok();
                }
            } else {
                writeln!(stdout, "NO").ok();
            }
        } else {
            writeln!(stdout, "NO").ok();
        }
    }
}
