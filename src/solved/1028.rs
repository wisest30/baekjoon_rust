use std::io::{Read, Write};

struct Cell {
    val: i32,
    cache_ld: usize,
    cache_rd: usize,
}

impl Cell {
    pub fn new(&val: &char) -> Self {
        let val = if val == '0' { 0 } else { 1 };
        Self {
            val,
            cache_ld: val as usize,
            cache_rd: val as usize,
        }
    }

    pub fn get_val(&self) -> i32 {
        self.val
    }

    pub fn set_left_down_count(&mut self, count: usize) {
        self.cache_ld = count;
    }

    pub fn get_left_down_count(&self) -> usize {
        self.cache_ld
    }

    pub fn set_right_down_count(&mut self, count: usize) {
        self.cache_rd = count;
    }

    pub fn get_right_down_count(&self) -> usize {
        self.cache_rd
    }

    pub fn min_cached_count(&self) -> usize {
        self.get_left_down_count().min(self.get_right_down_count())
    }
}

struct Board {
    row: usize,
    col: usize,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(board: Vec<Vec<char>>) -> Self {
        let row = board.len();
        let col = board[0].len();
        let mut cells = board
            .iter()
            .map(|r| r.iter().map(Cell::new).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for y in (0..row - 1).rev() {
            for x in 0..col {
                if x > 0 && cells[y][x].get_val() == 1 {
                    let prev_val = cells[y + 1][x - 1].get_left_down_count();
                    cells[y][x].set_left_down_count(prev_val + 1);
                }
                if x + 1 < col && cells[y][x].get_val() == 1 {
                    let prev_val = cells[y + 1][x + 1].get_right_down_count();
                    cells[y][x].set_right_down_count(prev_val + 1);
                }
            }
        }

        Self { row, col, cells }
    }

    pub fn ans(&self) -> usize {
        let mut ans = 0;
        (0..self.row).for_each(|y| {
            (0..self.col).for_each(|x| {
                (ans + 1..=self.cells[y][x].min_cached_count()).for_each(|len| {
                    if self.has_left_point(y, x, len)
                        && self.has_right_point(y, x, len)
                        && self.has_down_point(y, len)
                        && self.left_point(y, x, len).get_right_down_count() >= len
                        && self.right_point(y, x, len).get_left_down_count() >= len
                    {
                        ans = len;
                    }
                })
            })
        });

        ans
    }

    fn has_left_point(&self, y: usize, x: usize, len: usize) -> bool {
        x >= len - 1 && y + len - 1 < self.row
    }

    fn left_point(&self, y: usize, x: usize, len: usize) -> &Cell {
        &self.cells[y + len - 1][x - len + 1]
    }

    fn has_right_point(&self, y: usize, x: usize, len: usize) -> bool {
        x + len - 1 < self.col && y + len - 1 < self.row
    }

    fn right_point(&self, y: usize, x: usize, len: usize) -> &Cell {
        &self.cells[y + len - 1][x + len - 1]
    }

    fn has_down_point(&self, y: usize, len: usize) -> bool {
        y + 2 * (len - 1) < self.row
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let r = input.next().unwrap().parse::<usize>().unwrap();
    let _ = input.next().unwrap().parse::<usize>().unwrap();
    let board = (0..r)
        .map(|_| input.next().unwrap().chars().map(|c| c).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let board = Board::new(board);

    writeln!(stdout, "{}", board.ans()).ok();
}
