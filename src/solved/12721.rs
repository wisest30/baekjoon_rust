use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let test_case = input.next().unwrap().parse::<usize>().unwrap();
    for tc in 1..=test_case {
        let k = input.next().unwrap().parse::<usize>().unwrap();
        let n = input.next().unwrap().parse::<usize>().unwrap();
        let indices = (0..n)
            .map(|_| input.next().unwrap().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut v = vec![0; k];
        let mut empty = (0..k).collect::<std::collections::VecDeque<_>>();
        for i in 1..=k {
            for _ in 0..i {
                let x = empty.pop_front().unwrap();
                empty.push_back(x);
            }

            let p = empty.pop_back().unwrap();
            v[p] = i;
        }

        let mut ans = String::new();
        for i in indices {
            ans.push_str(&v[i - 1].to_string());
            ans.push(' ');
        }
        println!("Case #{}: {}", tc, ans);
    }
}
