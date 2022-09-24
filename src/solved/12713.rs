use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let test_case = input.next().unwrap().parse::<usize>().unwrap();
    for tc in 1..=test_case {
        let s = input.next().unwrap().chars().collect::<Vec<char>>();
        let n = s.len();
        let total = 3usize.pow(n as u32 - 1);

        let mut cnt = 0;
        for mut subset in 0..total {
            let mut val = 0;
            let mut cur = (s[0] as u8 - '0' as u8) as i64;
            let mut last_t = 1;
            for i in 0..n - 1 {
                let t = subset % 3;
                if t != 0 {
                    val += if last_t == 1 { cur } else { -cur };
                    last_t = t;
                    cur = 0;
                }

                cur = cur * 10 + (s[i + 1] as u8 - '0' as u8) as i64;
                subset /= 3;
            }

            val += if last_t == 1 { cur } else { -cur };

            if val % 2 == 0 || val % 3 == 0 || val % 5 == 0 || val % 7 == 0 {
                cnt += 1;
            }
        }

        let ans = cnt;
        println!("Case #{}: {}", tc, ans);
    }
}
