use std::io::Read;

const MOD: i64 = 1_000_000_007;

type FenwickTreeValueT = i64;
struct FenwickTree {
    f: Vec<FenwickTreeValueT>,
}

impl FenwickTree {
    pub fn new(sz: usize) -> Self {
        Self {
            f: vec![0; sz + 10],
        }
    }

    pub fn update(&mut self, idx: usize, val: FenwickTreeValueT) {
        let mut i = idx;
        while i < self.f.len() {
            self.f[i] += val;
            self.f[i] %= MOD;
            i |= i + 1;
        }
    }

    // sum of [0, right)
    pub fn get_pre_sum(&self, right: usize) -> FenwickTreeValueT {
        if right == 0 {
            0
        } else {
            let mut ret = 0;
            let mut i = right - 1;
            loop {
                ret += self.f[i];
                ret %= MOD;
                if i & (i + 1) == 0 {
                    break;
                } else {
                    i = (i & (i + 1)) - 1;
                }
            }

            ret
        }
    }

    // sum of [left, right)
    pub fn get_sum(&self, left: usize, right: usize) -> FenwickTreeValueT {
        (self.get_pre_sum(right) - self.get_pre_sum(left) + MOD) % MOD
    }
}

fn compress(v: Vec<i64>) -> Vec<i64> {
    use std::collections::HashMap;

    let mut sorted = v.clone();
    sorted.sort_unstable();
    let mut map: HashMap<i64, i64> = HashMap::new();
    for key in &sorted {
        if !map.contains_key(key) {
            map.insert(*key, map.len() as i64);
        }
    }

    v.iter()
        .map(|key| *map.get(key).unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let test_case = input.next().unwrap().parse::<usize>().unwrap();
    for tc in 1..=test_case {
        let n = input.next().unwrap().parse::<usize>().unwrap();
        let m = input.next().unwrap().parse::<usize>().unwrap();
        let x = input.next().unwrap().parse::<i64>().unwrap();
        let y = input.next().unwrap().parse::<i64>().unwrap();
        let z = input.next().unwrap().parse::<i64>().unwrap();

        let mut a = (0..m)
            .map(|_| input.next().unwrap().parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let signs = (0..n)
            .map(|i| {
                let cur = a[i % m];
                std::mem::replace(&mut a[i % m], (x * cur + y * (i as i64 + 1)) % z)
            })
            .collect::<Vec<_>>();

        let signs = compress(signs);

        let mut ft = FenwickTree::new(n);

        signs.into_iter().for_each(|sign| {
            let prev_cnt = ft.get_pre_sum(sign as usize);
            ft.update(sign as usize, 1);
            ft.update(sign as usize, prev_cnt);
        });

        let ans = ft.get_sum(0, n);
        println!("Case #{}: {}", tc, ans);
    }
}
