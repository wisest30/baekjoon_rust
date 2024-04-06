// a : 감소, b : 증가, x[i] : i번째 segment 시작 폐구간
struct ConvexHullTrick {
    a: Vec<i64>,
    b: Vec<i64>,
    x: Vec<f64>,
}

impl ConvexHullTrick {
    fn new() -> Self {
        Self {
            a: Vec::new(),
            b: Vec::new(),
            x: Vec::new(),
        }
    }

    fn add_line(&mut self, a: i64, b: i64) {
        if self.x.is_empty() {
            self.x.push(0.0);
        } else {
            assert_ne!(a, *self.a.last().unwrap());
            let mut x = (b - self.b.last().unwrap()) as f64 / (self.a.last().unwrap() - a) as f64;
            while !self.x.is_empty() && x < *self.x.last().unwrap() {
                self.a.pop();
                self.b.pop();
                self.x.pop();
                if !self.x.is_empty() {
                    assert_ne!(a, *self.a.last().unwrap());
                    x = (b - self.b.last().unwrap()) as f64 / (self.a.last().unwrap() - a) as f64;
                }
            }
            self.x.push(x);
        }

        self.a.push(a);
        self.b.push(b);
    }

    fn find_y(&self, x: i64) -> i64 {
        assert!(!self.x.is_empty());

        let mut l = 0;
        let mut r = self.x.len();
        while l + 1 < r {
            let m = (l + r) / 2;
            if self.x[m] <= x as f64 {
                l = m;
            } else {
                r = m;
            }
        }

        let l = l as usize;
        self.a[l] * x + self.b[l]
    }
}

#[test]
fn test_convex_hull_trick() {
    let mut cht = ConvexHullTrick::new();
    cht.add_line(200, 300);
    cht.add_line(100, 10300);
    cht.add_line(30, 14300);

    assert_eq!(cht.a, vec![200, 30]);
    assert_eq!(cht.b, vec![300, 14300]);
    assert_eq!(cht.x[0] as i64, 0);
    assert_eq!(cht.x[1] as i64, 82);
    assert_eq!(cht.find_y(80), 16300);
}
