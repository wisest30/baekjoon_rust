use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let test_case = input.next().unwrap().parse::<usize>().unwrap();
    for tc in 1..=test_case {
        let n = input.next().unwrap().parse::<usize>().unwrap();
        let mut v0 = (0..n)
            .map(|_| input.next().unwrap().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let mut v1 = (0..n)
            .map(|_| input.next().unwrap().parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        v0.sort_unstable();
        v1.sort_unstable();

        let mut v0 = v0.into_iter().collect::<VecDeque<_>>();
        let mut v1 = v1.into_iter().collect::<VecDeque<_>>();
        let mut ret = 0;
        for _ in 0..n {
            if v0.front().unwrap() * v1.back().unwrap() < 0 {
                ret += v0.pop_front().unwrap() * v1.pop_back().unwrap();
            } else if v0.back().unwrap() * v1.front().unwrap() < 0 {
                ret += v0.pop_back().unwrap() * v1.pop_front().unwrap();
            } else {
                ret += v0.pop_back().unwrap() * v1.pop_front().unwrap();
            }
        }

        println!("Case #{tc}: {ret}");
    }
}
