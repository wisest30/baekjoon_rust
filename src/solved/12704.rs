use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let test_case = input.next().unwrap().parse::<usize>().unwrap();
    for tc in 1..=test_case {
        let m = input.next().unwrap().parse::<usize>().unwrap();
        let v = input.next().unwrap().parse::<i32>().unwrap();
        let tree = (0..m)
            .map(|i| {
                // 0 : or, 1 : and, 2 : leaf
                if i < (m - 1) / 2 {
                    (
                        input.next().unwrap().parse::<i32>().unwrap(),
                        input.next().unwrap().parse::<i32>().unwrap(),
                    )
                } else {
                    (2, input.next().unwrap().parse::<i32>().unwrap())
                }
            })
            .collect::<Vec<_>>();

        let mut dp = vec![(0i32, 0i32); m];
        for i in (0..m).rev() {
            dp[i] = match tree[i].0 {
                0 => {
                    let ori = (
                        dp[i * 2 + 1].0.saturating_add(dp[i * 2 + 2].0),
                        std::cmp::min(dp[i * 2 + 1].1, dp[i * 2 + 2].1),
                    );

                    let change = if tree[i].1 == 0 {
                        (i32::MAX, i32::MAX)
                    } else {
                        (
                            std::cmp::min(dp[i * 2 + 1].0, dp[i * 2 + 2].0).saturating_add(1),
                            dp[i * 2 + 1]
                                .1
                                .saturating_add(dp[i * 2 + 2].1)
                                .saturating_add(1),
                        )
                    };

                    (
                        std::cmp::min(ori.0, change.0),
                        std::cmp::min(ori.1, change.1),
                    )
                }
                1 => {
                    let ori = (
                        std::cmp::min(dp[i * 2 + 1].0, dp[i * 2 + 2].0),
                        dp[i * 2 + 1].1.saturating_add(dp[i * 2 + 2].1),
                    );

                    let change = if tree[i].1 == 0 {
                        (i32::MAX, i32::MAX)
                    } else {
                        (
                            dp[i * 2 + 1]
                                .0
                                .saturating_add(dp[i * 2 + 2].0)
                                .saturating_add(1),
                            std::cmp::min(dp[i * 2 + 1].1, dp[i * 2 + 2].1).saturating_add(1),
                        )
                    };

                    (
                        std::cmp::min(ori.0, change.0),
                        std::cmp::min(ori.1, change.1),
                    )
                }
                2 => match tree[i].1 {
                    0 => (0, i32::MAX),
                    1 => (i32::MAX, 0),
                    _ => panic!(),
                },
                _ => panic!(),
            };
        }

        let ans = if v == 0 { dp[0].0 } else { dp[0].1 };
        if ans == i32::MAX {
            println!("Case #{}: IMPOSSIBLE", tc);
        } else {
            println!("Case #{}: {}", tc, ans);
        }
    }
}
