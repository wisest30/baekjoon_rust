use std::io::Read;

type FenwickTreeValueT = i64;
struct FenwickTree {
    n: usize,
    f: Vec<FenwickTreeValueT>,
}

impl FenwickTree {
    pub fn new(sz: usize) -> Self {
        Self {
            n: sz + 10,
            f: vec![0; sz + 10],
        }
    }

    pub fn update(&mut self, idx: usize, val: FenwickTreeValueT) {
        let mut i = idx;
        while i < self.f.len() {
            self.f[i] += val;
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
        self.get_pre_sum(right) - self.get_pre_sum(left)
    }

    pub fn less_equal_k(&self, k: FenwickTreeValueT) -> usize {
        let mut i = 0;
        while (1 << (i + 1)) <= self.n {
            i += 1;
        }

        let mut sum = 0;
        let mut right = 0;
        loop {
            while right + (1 << i) - 1 >= self.n {
                i -= 1;
            }
            let nxt_sum = sum + self.f[right + (1 << i) - 1];
            if nxt_sum <= k {
                sum = nxt_sum;
                right |= 1 << i;
            }

            if i == 0 || right == self.n {
                break;
            } else {
                i -= 1;
            }
        }

        right
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let test_case = input.next().unwrap().parse::<usize>().unwrap();
    for tc in 1..=test_case {
        let k = input.next().unwrap().parse::<usize>().unwrap();
        let n = input.next().unwrap().parse::<usize>().unwrap();
        let indices = (0..n)
            .map(|_| input.next().unwrap().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut ft = FenwickTree::new(k);
        for i in 0..k {
            ft.update(i, 1);
        }

        let mut v = vec![0; k];

        let mut p = 0;
        for i in 0..k {
            let ps = (ft.get_pre_sum(p) + i as i64) % ft.get_pre_sum(k);
            let x = ft.less_equal_k(ps) % k;
            v[x] = i + 1;
            ft.update(x, -1);
            p = x;
        }

        let mut ans = String::new();
        for i in indices {
            ans.push_str(&v[i - 1].to_string());
            ans.push(' ');
        }
        println!("Case #{}: {}", tc, ans);
    }
}
