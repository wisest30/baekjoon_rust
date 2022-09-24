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

#[test]
fn test_fenwick_tree() {
    let mut ft = FenwickTree::new(10);

    ft.update(0, 2);
    ft.update(2, 3);
    assert_eq!(ft.get_sum(0, 3), 5);
    assert_eq!(ft.get_sum(1, 3), 3);
    assert_eq!(ft.get_sum(2, 4), 3);
    assert_eq!(ft.get_sum(3, 4), 0);
    assert_eq!(ft.less_equal_k(4), 2);
}
