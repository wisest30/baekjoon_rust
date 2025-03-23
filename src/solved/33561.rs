use std::io::{Read, Write};

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

struct Problem {
    board: Vec<Vec<i32>>,
    d: Vec<i32>,
}

impl Problem {
    fn new(board: Vec<Vec<i32>>, d: Vec<i32>) -> Self {
        Self { board, d }
    }

    fn solve(&mut self) -> i32 {
        self.d.sort_unstable();
        self.d.reverse();

        let d_sum = PSum::new(&self.d);
        let board_sum = PSum2D::new(&self.board);

        let n = self.board.len();
        let board_is_zero = (0..n)
            .map(|y| {
                (0..n)
                    .map(|x| if self.board[y][x] == 0 { 1 } else { 0 })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let board_is_zero_sum = PSum2D::new(&board_is_zero);

        let solve_len = |len: usize| {
            let mut max_val = 0;
            for y in 0..n {
                if y + len > n {
                    break;
                }

                for x in 0..n {
                    if x + len > n {
                        break;
                    }

                    let zero_cnt = board_is_zero_sum.get((y, x), (y + len, x + len));
                    if zero_cnt > self.d.len() as i32 {
                        continue;
                    }

                    max_val = max_val.max(
                        board_sum.get((y, x), (y + len, x + len)) + d_sum.get(0, zero_cnt as usize),
                    );
                }
            }

            max_val
        };

        (1..=n).map(|len| solve_len(len)).max().unwrap()
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let n = input.next().unwrap().parse::<usize>().unwrap();
    let board = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| input.next().unwrap().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let k = input.next().unwrap().parse::<usize>().unwrap();
    let d = (0..k)
        .map(|_| input.next().unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let ans = Problem::new(board, d).solve();
    writeln!(stdout, "{ans}").ok();
}
