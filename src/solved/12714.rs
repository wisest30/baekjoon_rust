use std::io::Read;

const DP_SZ: usize = 2 * 3 * 5 * 7;
fn recur(s: &Vec<char>, cur: usize, dp: &mut Vec<Vec<i64>>) {
    if dp[cur].is_empty() {
        dp[cur] = vec![0; DP_SZ];

        if cur == s.len() {
            dp[cur][0] = 1;
            return;
        }

        let mut val2 = 0i64;
        let mut val3 = 0i64;
        let mut val5 = 0i64;
        let mut val7 = 0i64;
        for nxt in cur + 1..=s.len() {
            val2 = val2 * 10 + s[nxt - 1].to_digit(10).unwrap() as i64;
            val2 %= 2;
            val3 = val3 * 10 + s[nxt - 1].to_digit(10).unwrap() as i64;
            val3 %= 3;
            val5 = val5 * 10 + s[nxt - 1].to_digit(10).unwrap() as i64;
            val5 %= 5;
            val7 = val7 * 10 + s[nxt - 1].to_digit(10).unwrap() as i64;
            val7 %= 7;
            recur(s, nxt, dp);

            for i in 0..DP_SZ {
                let n2 = i as i64 % 2;
                let n3 = i as i64 / 2 % 3;
                let n5 = i as i64 / 2 / 3 % 5;
                let n7 = i as i64 / 2 / 3 / 5 % 7;

                let p2 = (n2 + val2) % 2;
                let p3 = (n3 + val3) % 3;
                let p5 = (n5 + val5) % 5;
                let p7 = (n7 + val7) % 7;
                let j = p7 * 5 * 3 * 2 + p5 * 3 * 2 + p3 * 2 + p2;
                dp[cur][j as usize] += dp[nxt][i];

                if cur > 0 {
                    let p2 = (n2 - val2 % 2 + 2) % 2;
                    let p3 = (n3 - val3 % 3 + 3) % 3;
                    let p5 = (n5 - val5 % 5 + 5) % 5;
                    let p7 = (n7 - val7 % 7 + 7) % 7;
                    let j = p7 * 5 * 3 * 2 + p5 * 3 * 2 + p3 * 2 + p2;
                    dp[cur][j as usize] += dp[nxt][i];
                }
            }
        }
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let test_case = input.next().unwrap().parse::<usize>().unwrap();
    for tc in 1..=test_case {
        let s = input.next().unwrap().chars().collect::<Vec<char>>();

        let mut dp = vec![Vec::new(); s.len() + 1];
        recur(&s, 0, &mut dp);
        let mut ans = 3i64.pow(s.len() as u32 - 1);
        for i in 0..DP_SZ {
            let n2 = i as i64 % 2;
            let n3 = i as i64 / 2 % 3;
            let n5 = i as i64 / 2 / 3 % 5;
            let n7 = i as i64 / 2 / 3 / 5 % 7;

            if n2 > 0 && n3 > 0 && n5 > 0 && n7 > 0 {
                ans -= dp[0][i];
            }
        }

        println!("Case #{}: {}", tc, ans);
    }
}
