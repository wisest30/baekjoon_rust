use std::collections::{HashSet, VecDeque};
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

        let mut valid = true;
        let mut ones = HashSet::<i32>::new();
        let mut q = VecDeque::<i32>::new();
        let mut v: Vec<(Option<i32>, HashSet<i32>)> = vec![(None, HashSet::new()); m];
        for i in 0..m {
            let t = input.next().unwrap().parse::<usize>().unwrap();

            for _ in 0..t {
                let p = input.next().unwrap().parse::<i32>().unwrap() - 1;
                let x = input.next().unwrap().parse::<i32>().unwrap();

                if x == 1 {
                    v[i].0 = Some(p);
                } else {
                    v[i].1.insert(p);
                }
            }

            if v[i].1.is_empty() && !ones.contains(&v[i].0.unwrap()) {
                ones.insert(v[i].0.unwrap());
                q.push_back(v[i].0.unwrap());
            }
        }

        while let Some(p) = q.pop_front() {
            for i in 0..m {
                if v[i].1.remove(&p) {
                    if v[i].1.is_empty() {
                        if let Some(np) = v[i].0 {
                            if !ones.contains(&np) {
                                ones.insert(np);
                                q.push_back(np);
                            }
                        } else {
                            valid = false;
                            break;
                        }
                    }
                }
            }

            if !valid {
                break;
            }
        }

        if !valid {
            println!("Case #{tc}: IMPOSSIBLE");
        } else {
            let mut ans = String::new();
            for i in 0..n {
                if ones.contains(&(i as i32)) {
                    ans.push_str("1 ");
                } else {
                    ans.push_str("0 ");
                }
            }
            println!("Case #{tc}: {ans}");
        }
    }
}
