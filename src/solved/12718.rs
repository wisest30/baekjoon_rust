use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let test_case = input.next().unwrap().parse::<usize>().unwrap();
    for tc in 1..=test_case {
        let n = input.next().unwrap().parse::<usize>().unwrap();
        let a = input.next().unwrap().parse::<i64>().unwrap();
        let b = input.next().unwrap().parse::<i64>().unwrap();
        let c = input.next().unwrap().parse::<i64>().unwrap();
        let d = input.next().unwrap().parse::<i64>().unwrap();
        let mut x = input.next().unwrap().parse::<i64>().unwrap();
        let mut y = input.next().unwrap().parse::<i64>().unwrap();
        let m = input.next().unwrap().parse::<i64>().unwrap();

        let mut counter = std::collections::HashMap::new();

        let v = vec![
            (0i64, 0i64),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];

        for &p in &v {
            counter.insert(p, 0);
        }

        *counter.get_mut(&(x % 3, y % 3)).unwrap() += 1;
        for _ in 1..n {
            x = (a * x + b) % m;
            y = (c * y + d) % m;

            *counter.get_mut(&(x % 3, y % 3)).unwrap() += 1;
        }

        let mut ans = 0i64;
        for &p0 in &v {
            for &p1 in &v {
                for &p2 in &v {
                    if (p0.0 + p1.0 + p2.0) % 3 == 0 && (p0.1 + p1.1 + p2.1) % 3 == 0 {
                        if p0 != p1 && p1 != p2 && p2 != p0 {
                            ans += counter.get(&p0).unwrap()
                                * counter.get(&p1).unwrap()
                                * counter.get(&p2).unwrap();
                        } else if p0 == p1 && p1 == p2 && p2 == p0 {
                            ans += counter.get(&p0).unwrap()
                                * (counter.get(&p1).unwrap() - 1)
                                * (counter.get(&p2).unwrap() - 2);
                        } else if p0 == p1 {
                            ans += counter.get(&p0).unwrap()
                                * (counter.get(&p1).unwrap() - 1)
                                * counter.get(&p2).unwrap();
                        } else if p1 == p2 {
                            ans += counter.get(&p0).unwrap()
                                * counter.get(&p1).unwrap()
                                * (counter.get(&p2).unwrap() - 1);
                        } else if p2 == p0 {
                            ans += (counter.get(&p0).unwrap() - 1)
                                * counter.get(&p1).unwrap()
                                * counter.get(&p2).unwrap();
                        } else {
                            panic!();
                        }
                    }
                }
            }
        }

        ans /= 6;
        println!("Case #{tc}: {ans}");
    }
}
