use std::collections::BTreeMap;
use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();

    for tc in 1..=n {
        const MOD: i64 = 1000000007;
        let n = input.next().unwrap().parse::<usize>().unwrap();
        let m = input.next().unwrap().parse::<usize>().unwrap();
        let x = input.next().unwrap().parse::<i64>().unwrap();
        let y = input.next().unwrap().parse::<i64>().unwrap();
        let z = input.next().unwrap().parse::<i64>().unwrap();
        let mut a = (0..m)
            .map(|_| input.next().unwrap().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let mut b = Vec::new();
        for i in 0..n {
            b.push(a[i % m]);
            a[i % m] = (x * a[i % m] + y * (i as i64 + 1)) % z
        }

        let mut map: BTreeMap<i64, i64> = BTreeMap::new();
        for &x in &b {
            *map.entry(x).or_insert(0i64) +=
                map.iter().filter(|p| *p.0 < x).map(|p| p.1).sum::<i64>() % MOD + 1;
            *map.get_mut(&x).unwrap() %= MOD;
        }

        let ans: i64 = map.values().into_iter().sum::<i64>() % MOD;
        println!("Case #{}: {}", tc, ans);
    }
}
