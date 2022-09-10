use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let test_case = input.next().unwrap().parse::<usize>().unwrap();
    for tc in 1..=test_case {
        let n = input.next().unwrap().parse::<usize>().unwrap();
        let m = input.next().unwrap().parse::<usize>().unwrap();

        let mut v = vec![vec![]; m];
        for i in 0..m {
            let t = input.next().unwrap().parse::<usize>().unwrap();
            v[i] = (0..t)
                .map(|_| {
                    (
                        input.next().unwrap().parse::<i32>().unwrap() - 1,
                        input.next().unwrap().parse::<i32>().unwrap(),
                    )
                })
                .collect::<Vec<_>>();
        }

        let mut ans: i32 = -1;
        for subset in 0..(1 << n) {
            let mut total_valid = true;
            for customer in &v {
                let mut valid = false;
                for e in customer {
                    let x = (subset >> e.0) & 1;
                    if x == e.1 {
                        valid = true;
                        break;
                    }
                }

                if !valid {
                    total_valid = false;
                    break;
                }
            }

            if total_valid && (ans == -1 || subset.count_ones() < ans.count_ones()) {
                ans = subset;
            }
        }

        let ans = if ans == -1 {
            "IMPOSSIBLE".to_string()
        } else {
            let mut buf = String::new();
            for i in 0..n {
                if (ans & (1 << i)) == 0 {
                    buf.push_str("0 ");
                } else {
                    buf.push_str("1 ");
                }
            }

            buf
        };

        println!("Case #{tc}: {ans}");
    }
}
