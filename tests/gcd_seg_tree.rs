type T = i64;

fn gcd(mut x: T, mut y: T) -> T {
    while y > 0 {
        x = x % y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

struct GcdSegTree {
    n: usize,
    val: Vec<T>,
}

impl GcdSegTree {
    pub fn new(n: usize) -> Self {
        GcdSegTree {
            n,
            val: vec![1; 4 * n + 10],
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
            self.val[i] = gcd(self.val[i * 2 + 1], self.val[i * 2 + 2]);
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
                gcd(
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
fn test_gcd_seg_tree() {
    let mut t = GcdSegTree::new(6);
    t.set(0, 2);
    t.set(1, 4);
    t.set(2, 8);
    t.set(3, 12);
    t.set(4, 9);

    assert_eq!(t.get(0, 2), 2);
    assert_eq!(t.get(1, 2), 4);
    assert_eq!(t.get(1, 4), 4);
    assert_eq!(t.get(3, 5), 3);
    assert_eq!(t.get(5, 6), 1);
    assert_eq!(t.get(0, 5), 1);
}
