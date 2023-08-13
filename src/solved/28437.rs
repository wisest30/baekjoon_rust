use std::collections::HashSet;
use std::io::Read;

fn get_factors(n: i32) -> Vec<i32> {
    let mut factors = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            factors.push(i);
            if i != n / i {
                factors.push(n / i);
            }
        }
        i += 1;
    }

    factors
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let a = (0..n)
        .map(|_| input.next().unwrap().parse::<i32>().unwrap())
        .collect::<HashSet<_>>();

    let mut dp = vec![0i64; 1000001];
    for i in 1..=100000 {
        if a.contains(&i) {
            dp[i as usize] = 1;
        }

        get_factors(i)
            .iter()
            .filter(|&&factor| factor != 1)
            .for_each(|&factor| {
                dp[i as usize] += dp[(i / factor) as usize];
            });
    }

    let q = input.next().unwrap().parse::<usize>().unwrap();
    (0..q)
        .map(|_| input.next().unwrap().parse::<usize>().unwrap())
        .map(|target| dp[target])
        .for_each(|ans| print!("{ans} "));
}
