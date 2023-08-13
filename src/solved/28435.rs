use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    const MOD: i64 = 1_000_000_007;
    let n = input.next().unwrap().parse::<usize>().unwrap();
    let k = input.next().unwrap().parse::<usize>().unwrap();
    let mut cnts = vec![0i64; k];
    (0..n)
        .map(|_| input.next().unwrap().parse::<usize>().unwrap())
        .for_each(|x| cnts[x % k] += 1);
    let pow2 = (0..=n)
        .scan(0i64, |acc, _| {
            *acc = if *acc == 0 { 1 } else { (*acc * 2) % MOD };
            Some(*acc)
        })
        .collect::<Vec<_>>();

    let mut ans = (0..=k / 2).fold(1i64, |acc, i| {
        let j = (k - i) % k;
        if i == j {
            acc * (cnts[i] + 1) % MOD
        } else {
            acc * (pow2[cnts[i] as usize] + pow2[cnts[j] as usize] - 1) % MOD
        }
    });

    ans = (ans - 1 - n as i64 + MOD) % MOD;

    println!("{ans}");
}
