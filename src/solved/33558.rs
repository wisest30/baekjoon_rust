use std::io::{Read, Write};

enum Method {
    ClampToEdge,
    Repeat,
    MirroredRepeat,
}

impl Method {
    fn from_str(s: &str) -> Self {
        match s {
            "clamp-to-edge" => Self::ClampToEdge,
            "repeat" => Self::Repeat,
            "mirrored-repeat" => Self::MirroredRepeat,
            _ => panic!("wrong method"),
        }
    }
}

struct Problem {
    n: usize,
    m: usize,
    u: usize,
    v: usize,
    texture: Vec<Vec<char>>,
    method: Method,
}

impl Problem {
    fn new(
        n: usize,
        m: usize,
        u: usize,
        v: usize,
        texture: Vec<Vec<char>>,
        method: Method,
    ) -> Self {
        Self {
            n,
            m,
            u,
            v,
            texture,
            method,
        }
    }

    fn solve_clamp_to_edge(&self) -> Vec<Vec<char>> {
        let mut ret = vec![vec![0 as char; self.m]; self.n];
        for y in 0..self.n {
            for x in 0..self.m {
                ret[y][x] = if y >= self.u && x >= self.v {
                    *self.texture.last().unwrap().last().unwrap()
                } else if y >= self.u {
                    self.texture.last().unwrap()[x]
                } else if x >= self.v {
                    *self.texture[y].last().unwrap()
                } else {
                    self.texture[y][x]
                };
            }
        }

        ret
    }
    fn solve_repeat(&self) -> Vec<Vec<char>> {
        let mut ret = vec![vec![0 as char; self.m]; self.n];
        for y in 0..self.n {
            for x in 0..self.m {
                ret[y][x] = self.texture[y % self.u][x % self.v];
            }
        }

        ret
    }
    fn solve_mirrored_repeat(&self) -> Vec<Vec<char>> {
        let mut ret = vec![vec![0 as char; self.m]; self.n];
        for y in 0..self.n {
            for x in 0..self.m {
                let mirror_vertical = y / self.u % 2 == 1;
                let mirror_horizontal = x / self.v % 2 == 1;

                let ty = if mirror_vertical {
                    self.u - 1 - y % self.u
                } else {
                    y % self.u
                };
                let tx = if mirror_horizontal {
                    self.v - 1 - x % self.v
                } else {
                    x % self.v
                };

                ret[y][x] = self.texture[ty][tx];
            }
        }

        ret
    }

    fn solve(&self) -> String {
        match self.method {
            Method::ClampToEdge => self.solve_clamp_to_edge(),
            Method::Repeat => self.solve_repeat(),
            Method::MirroredRepeat => self.solve_mirrored_repeat(),
        }
        .iter()
        .map(|v| v.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let n = input.next().unwrap().parse::<usize>().unwrap();
    let m = input.next().unwrap().parse::<usize>().unwrap();
    let u = input.next().unwrap().parse::<usize>().unwrap();
    let v = input.next().unwrap().parse::<usize>().unwrap();
    let texture = (0..u)
        .map(|_| input.next().unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let method = Method::from_str(input.next().unwrap());

    let ans = Problem::new(n, m, u, v, texture, method).solve();
    writeln!(stdout, "{ans}").ok();
}
