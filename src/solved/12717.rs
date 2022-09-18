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

        let mut v_x = vec![x];
        let mut v_y = vec![y];

        for _ in 1..n {
            x = (a * x + b) % m;
            y = (c * y + d) % m;

            v_x.push(x);
            v_y.push(y);
        }

        let mut ans = 0;
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    let sx = v_x[i] + v_x[j] + v_x[k];
                    let sy = v_y[i] + v_y[j] + v_y[k];

                    if sx % 3 == 0 && sy % 3 == 0 {
                        ans += 1;
                    }
                }
            }
        }

        println!("Case #{tc}: {ans}");
    }
}
