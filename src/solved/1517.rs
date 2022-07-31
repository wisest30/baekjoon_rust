use std::collections::{BTreeSet, HashMap};
use std::io::Read;
struct Fenwick {
    f: Vec<usize>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self { f: vec![0; n] }
    }

    fn get(&self, mut i: usize) -> usize {
        let mut ret = 0;
        loop {
            ret += self.f[i];
            if i & (i + 1) == 0 {
                break ret;
            } else {
                i = (i & (i + 1)) - 1;
            }
        }
    }

    fn inc(&mut self, mut i: usize) {
        while i < self.f.len() {
            self.f[i] += 1;
            i |= i + 1;
        }
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let mut nums = (0..n)
        .map(|_| input.next().unwrap().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let set = nums.iter().copied().collect::<BTreeSet<_>>();
    let mut map = HashMap::new();
    for num in set {
        map.insert(num, map.len());
    }
    for i in 0..n {
        nums[i] = *map.get(&nums[i]).unwrap();
    }

    let mut ret = 0;
    let mut fw = Fenwick::new(n + 1);
    for num in nums.into_iter().rev() {
        if num > 0 {
            ret += fw.get(num - 1);
        }

        fw.inc(num);
    }

    println!("{}", ret);
}
