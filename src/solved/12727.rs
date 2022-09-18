use std::io::Read;

struct Combination {
    v: Vec<Vec<u64>>,
}

impl Combination {
    pub fn new_mod(n: u64, m: u64) -> Combination {
        let mut v = vec![vec![0; n as usize + 1]; n as usize + 1];
        if m == 1 {
            return Combination { v };
        }

        v[0][0] = 1;
        v[1][0] = 1;
        v[1][1] = 1;

        for i in 2..=n as usize {
            v[i][0] = 1;
            for j in 1..=n as usize {
                v[i][j] = (v[i - 1][j - 1] + v[i - 1][j]) % m;
            }
        }

        Combination { v }
    }

    pub fn ncr(&self, n: usize, r: usize) -> u64 {
        self.v[n][r]
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let test_case = input.next().unwrap().parse::<usize>().unwrap();

    const MOD: u64 = 10000;
    let comb = Combination::new_mod(30, MOD);
    for tc in 1..=test_case {
        let n = input.next().unwrap().parse::<usize>().unwrap();
        let mut p = 0u64;
        for i in (0..=n).step_by(2) {
            p += comb.ncr(n, i) * 3u64.pow((n - i) as u32) * 5u64.pow(i as u32 / 2);
            p %= MOD;
        }
        p *= 2;
        p -= 1;

        let ans = p % 1000;
        println!("Case #{tc}: {:03}", ans);
    }
}
