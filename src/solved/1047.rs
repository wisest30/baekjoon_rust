use std::io::{Read, Write};

struct Tree {
    x: usize,
    y: usize,
    cnt: usize,
}

impl Tree {
    pub fn new(y: usize, x: usize, cnt: usize) -> Self {
        Self { x, y, cnt }
    }

    pub fn remain(&self, rectangle: &Rectangle) -> bool {
        rectangle.x0 <= self.x
            && self.x <= rectangle.x1
            && rectangle.y0 <= self.y
            && self.y <= rectangle.y1
    }

    pub fn get_cell_count(&self) -> usize {
        self.cnt
    }
}

struct Rectangle {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl Rectangle {
    pub fn new(x0: usize, y0: usize, x1: usize, y1: usize) -> Self {
        Self { x0, y0, x1, y1 }
    }

    pub fn max_remain_tree_count(&self, sorted_trees: &Vec<Tree>) -> usize {
        let mut cell_count = 0;
        let mut remain_trees = Vec::new();
        for tree in sorted_trees {
            if tree.remain(self) {
                remain_trees.push(tree);
            } else {
                cell_count += tree.get_cell_count();
            }
        }

        while !remain_trees.is_empty() && cell_count < self.need_cell_count() {
            cell_count += remain_trees.pop().unwrap().get_cell_count();
        }

        // 모든 tree 가 벌목되어도 cell_count 가 부족한 경우 0을 리턴
        remain_trees.len()
    }

    fn need_cell_count(&self) -> usize {
        if self.x0 == self.x1 && self.y0 == self.y1 {
            1
        } else if self.x0 == self.x1 {
            self.y1 - self.y0 + 1
        } else if self.y0 == self.y1 {
            self.x1 - self.x0 + 1
        } else {
            2 * (self.x1 - self.x0 + 1) + 2 * (self.y1 - self.y0 - 1)
        }
    }
}

struct RectangleGenerator {
    rectangles: Vec<Rectangle>,
    sorted_trees: Vec<Tree>,
}

impl RectangleGenerator {
    pub fn new(sorted_trees: Vec<Tree>) -> Self {
        let mut xs = sorted_trees.iter().map(|tree| tree.x).collect::<Vec<_>>();
        xs.sort_unstable();
        let mut ys = sorted_trees.iter().map(|tree| tree.y).collect::<Vec<_>>();
        ys.sort_unstable();

        let mut rectangles = Vec::new();
        for x0 in 0..xs.len() {
            for x1 in x0..xs.len() {
                for y0 in 0..ys.len() {
                    for y1 in y0..ys.len() {
                        rectangles.push(Rectangle::new(xs[x0], ys[y0], xs[x1], ys[y1]));
                    }
                }
            }
        }

        Self {
            rectangles,
            sorted_trees,
        }
    }

    pub fn ans(&self) -> usize {
        self.sorted_trees.len()
            - self
                .rectangles
                .iter()
                .map(|rectangle| rectangle.max_remain_tree_count(&self.sorted_trees))
                .max()
                .unwrap()
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let mut sorted_trees = (0..n)
        .map(|_| {
            let x = input.next().unwrap().parse::<usize>().unwrap();
            let y = input.next().unwrap().parse::<usize>().unwrap();
            let cnt = input.next().unwrap().parse::<usize>().unwrap();

            Tree::new(x, y, cnt)
        })
        .collect::<Vec<_>>();
    sorted_trees.sort_unstable_by(|a, b| a.cnt.cmp(&b.cnt));

    let rectangle_generator = RectangleGenerator::new(sorted_trees);
    writeln!(stdout, "{}", rectangle_generator.ans()).ok();
}
