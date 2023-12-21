type T = i64;
struct SumLazySegTree {
    n: usize,
    val: Vec<T>,
    lazy: Vec<T>,
}

impl SumLazySegTree {
    fn new(n: usize) -> Self {
        SumLazySegTree {
            n,
            val: vec![0; 4 * n + 20],
            lazy: vec![0; 4 * n + 20],
        }
    }

    fn push(&mut self, i: usize, l: usize, r: usize) {
        self.val[i] += (r - l) as T * self.lazy[i];
        if l + 1 != r {
            self.lazy[i * 2 + 1] += self.lazy[i];
            self.lazy[i * 2 + 2] += self.lazy[i];
        }
        self.lazy[i] = 0;
    }

    fn pull(&mut self, i: usize) {
        self.val[i] = self.val[i * 2 + 1] + self.val[i * 2 + 2];
    }

    fn upd_impl(&mut self, i: usize, l: usize, r: usize, left: usize, right: usize, x: T) {
        self.push(i, l, r);
        if r <= left || right <= l {
            return;
        } else if left <= l && r <= right {
            self.lazy[i] = x;
            self.push(i, l, r);
            return;
        }
        let m = (l + r) / 2;
        self.upd_impl(i * 2 + 1, l, m, left, right, x);
        self.upd_impl(i * 2 + 2, m, r, left, right, x);
        self.pull(i);
    }

    fn get_impl(&mut self, i: usize, l: usize, r: usize, left: usize, right: usize) -> T {
        self.push(i, l, r);
        if left <= l && r <= right {
            return self.val[i];
        }
        let m = (l + r) / 2;
        let mut ret = 0;
        if left < m {
            ret += self.get_impl(i * 2 + 1, l, m, left, right);
        }
        if right > m {
            ret += self.get_impl(i * 2 + 2, m, r, left, right);
        }
        ret
    }

    pub fn upd(&mut self, left: usize, right: usize, x: T) {
        self.upd_impl(0, 0, self.n, left, right, x);
    }

    pub fn get(&mut self, left: usize, right: usize) -> T {
        self.get_impl(0, 0, self.n, left, right)
    }
}

#[test]
fn test_sum_lazy_seg_tree() {
    let mut t = SumLazySegTree::new(10);
    t.upd(0, 3, 1);
    t.upd(3, 6, -1);
    t.upd(6, 10, 2);

    assert_eq!(t.get(0, 1), 1);
    assert_eq!(t.get(0, 3), 3);
    assert_eq!(t.get(0, 5), 1);
    assert_eq!(t.get(2, 7), 0);

    t.upd(0, 3, -1);
    t.upd(3, 6, 1);
    t.upd(6, 10, -2);

    assert_eq!(t.get(0, 1), 0);
    assert_eq!(t.get(0, 10), 0);
}
