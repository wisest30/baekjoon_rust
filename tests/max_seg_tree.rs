type T = i64;
struct MaxSegTree {
    n: usize,
    val: Vec<T>,
}

impl MaxSegTree {
    pub fn new(n: usize) -> Self {
        MaxSegTree {
            n,
            val: vec![T::MIN; 4 * n + 10],
        }
    }

    fn set_(&mut self, i: usize, l: usize, r: usize, pos: usize, x: T) {
        if l + 1 == r {
            self.val[i] = x;
        } else {
            let m = (l + r) / 2;
            if pos < m {
                self.set_(i * 2 + 1, l, m, pos, x);
            } else {
                self.set_(i * 2 + 2, m, r, pos, x);
            }
            self.val[i] = std::cmp::max(self.val[i * 2 + 1], self.val[i * 2 + 2]);
        }
    }

    pub fn set(&mut self, pos: usize, x: T) {
        self.set_(0, 0, self.n, pos, x);
    }

    fn get_(&self, i: usize, l: usize, r: usize, left: usize, right: usize) -> T {
        if left <= l && r <= right {
            self.val[i]
        } else {
            let m = (l + r) / 2;

            if right <= m {
                self.get_(i * 2 + 1, l, m, left, right)
            } else if m <= left {
                self.get_(i * 2 + 2, m, r, left, right)
            } else {
                std::cmp::max(
                    self.get_(i * 2 + 1, l, m, left, right),
                    self.get_(i * 2 + 2, m, r, left, right),
                )
            }
        }
    }

    pub fn get(&self, left: usize, right: usize) -> T {
        self.get_(0, 0, self.n, left, right)
    }
}

#[test]
fn test_max_seg_tree() {
    let mut t = MaxSegTree::new(10);
    t.set(0, 1);
    t.set(2, 2);
    t.set(4, 3);
    t.set(6, 4);
    t.set(8, 5);

    assert_eq!(t.get(0, 1), 1);
    assert_eq!(t.get(1, 2), T::MIN);
    assert_eq!(t.get(0, 10), 5);
    assert_eq!(t.get(4, 6), 3);
}
