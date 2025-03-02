type ElemT = i32;
struct PSum {
    v: Vec<ElemT>,
}

impl PSum {
    fn new(a: &Vec<ElemT>) -> Self {
        let mut v = vec![0; a.len() + 1];
        for i in 0..a.len() {
            v[i + 1] = v[i] + a[i];
        }
        Self { v }
    }

    fn get(&self, i: usize, j: usize) -> ElemT {
        self.v[j] - self.v[i]
    }
}

struct PSum2D {
    v: Vec<Vec<ElemT>>,
}

impl PSum2D {
    fn new(a: &Vec<Vec<ElemT>>) -> Self {
        let mut v = vec![vec![0; a[0].len() + 1]; a.len() + 1];
        for y in 0..a.len() {
            for x in 0..a[0].len() {
                v[y + 1][x + 1] = v[y + 1][x] + v[y][x + 1] - v[y][x] + a[y][x];
            }
        }

        Self { v }
    }

    fn get(&self, (y0, x0): (usize, usize), (y1, x1): (usize, usize)) -> ElemT {
        self.v[y1][x1] - self.v[y1][x0] - self.v[y0][x1] + self.v[y0][x0]
    }
}

// TODO Test
