use std::io::{Read, Write};

#[derive(Clone)]
struct Shape {
    codes: Vec<Vec<char>>,
}

impl Shape {
    fn new(code_str: &str) -> Self {
        Self {
            codes: vec![code_str.chars().collect::<Vec<_>>()],
        }
    }

    fn new_none() -> Self {
        Self { codes: Vec::new() }
    }

    fn is_none(&self) -> bool {
        self.codes.is_empty()
    }

    fn concat(bottom: &Self, top: &Self) -> Self {
        if bottom.is_none() || top.is_none() {
            Self::new_none()
        } else {
            let mut ret = bottom.clone();

            let is_possible =
                |bottom_codes: &Vec<Vec<char>>, top_codes: &Vec<Vec<char>>, top_start: usize| {
                    if top_start == bottom_codes.len() {
                        return true;
                    }

                    for i in top_start..bottom_codes.len() {
                        for j in 0..(i + 1 - top_start) {
                            if j >= top_codes.len() {
                                break;
                            }
                            for k in 0..8 {
                                if bottom_codes[i][k] != '-' && top_codes[j][k] != '-' {
                                    return false;
                                }
                            }
                        }
                    }

                    true
                };

            let put = |bottom_codes: &mut Vec<Vec<char>>,
                       top_codes: &Vec<Vec<char>>,
                       top_start: usize| {
                for i in top_start..top_start + top_codes.len() {
                    let j = i - top_start;
                    if i >= bottom_codes.len() {
                        bottom_codes.push(top_codes[j].clone());
                    } else {
                        for k in 0..8 {
                            if top_codes[j][k] != '-' {
                                bottom_codes[i][k] = top_codes[j][k];
                            }
                        }
                    }
                }
                while bottom_codes.len() > 4 {
                    bottom_codes.pop();
                }
            };

            for i in 0..=ret.codes.len() {
                if is_possible(&ret.codes, &top.codes, i) {
                    put(&mut ret.codes, &top.codes, i);
                    break;
                }
            }

            ret
        }
    }

    fn to_string(&self) -> String {
        if self.is_none() {
            "None".to_string()
        } else {
            self.codes
                .iter()
                .map(|code| code.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join(":")
        }
    }

    fn erase_empty_code(&mut self) {
        while let Some((i, _)) = self
            .codes
            .iter()
            .enumerate()
            .find(|(_, code)| code.iter().filter(|&&c| c == '-').count() == 8)
        {
            self.codes.remove(i);
        }
    }

    fn split(&self) -> (Self, Self) {
        if self.is_none() {
            return (Self::new_none(), Self::new_none());
        }

        let mut left = self.clone();
        let mut right = self.clone();

        for code in &mut left.codes {
            code[4] = '-';
            code[5] = '-';
            code[6] = '-';
            code[7] = '-';
        }

        for code in &mut right.codes {
            code[0] = '-';
            code[1] = '-';
            code[2] = '-';
            code[3] = '-';
        }

        left.erase_empty_code();
        right.erase_empty_code();

        (left, right)
    }

    fn rotate(&self, degree: usize) -> Self {
        if self.is_none() {
            return Self::new_none();
        }

        let mut ret = self.clone();
        for code in &mut ret.codes {
            match degree {
                0 => {
                    *code = vec![
                        code[6], code[7], code[0], code[1], code[2], code[3], code[4], code[5],
                    ]
                }
                1 => {
                    *code = vec![
                        code[4], code[5], code[6], code[7], code[0], code[1], code[2], code[3],
                    ]
                }
                2 => {
                    *code = vec![
                        code[2], code[3], code[4], code[5], code[6], code[7], code[0], code[1],
                    ]
                }
                _ => panic!("wrong degree"),
            }
        }

        ret
    }

    fn paint(&self, color: char) -> Self {
        if self.is_none() {
            return Self::new_none();
        }

        let mut ret = self.clone();
        for code in &mut ret.codes {
            for i in (1..8).step_by(2) {
                if code[i] != '-' {
                    code[i] = color;
                }
            }
        }

        ret
    }
}

type CommandType = (usize, usize, usize, String);
struct Problem {
    _n: usize,
    _m: usize,
    init_codes: Vec<String>,
    commands: Vec<CommandType>,
}

impl Problem {
    fn new(n: usize, m: usize, init_codes: Vec<String>, commands: Vec<CommandType>) -> Self {
        Self {
            _n: n,
            _m: m,
            init_codes,
            commands,
        }
    }

    fn solve(self) -> String {
        let mut registers = self
            .init_codes
            .iter()
            .map(|code| Shape::new(code.as_str()))
            .collect::<Vec<_>>();

        while registers.len() < 100 {
            registers.push(Shape::new_none());
        }

        //let mut debug_cnt = 0;
        for command in self.commands {
            let (i, j) = (command.1 - 1, command.2 - 1);
            match command.0 {
                1 => {
                    let k = command.3.parse::<usize>().unwrap() - 1;

                    let (left, right) = registers[i].split();
                    registers[k] = left;
                    registers[j] = right;
                }
                2 => {
                    let k = command.3.parse::<usize>().unwrap() - 1;
                    registers[j] = registers[i].rotate(k)
                }
                3 => {
                    let k = command.3.parse::<usize>().unwrap() - 1;
                    registers[k] = Shape::concat(&registers[i], &registers[j])
                }
                4 => {
                    let k = command.3.chars().next().unwrap();
                    registers[j] = registers[i].paint(k)
                }
                _ => panic!("wrong command"),
            }

            // if debug_cnt == 4 || debug_cnt == 5 {
            //     for i in 0..10 {
            //         println!("{}", registers[i].to_string());
            //     }
            //     println!("===============");
            // }
            //debug_cnt += 1;
        }

        //println!("debug===========");
        registers.last().unwrap().to_string()
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
    let init_codes = (0..n)
        .map(|_| input.next().unwrap().to_string())
        .collect::<Vec<_>>();
    let commands = (0..m)
        .map(|_| {
            (
                input.next().unwrap().parse::<usize>().unwrap(),
                input.next().unwrap().parse::<usize>().unwrap(),
                input.next().unwrap().parse::<usize>().unwrap(),
                input.next().unwrap().to_string(),
            )
        })
        .collect::<Vec<_>>();

    let ans = Problem::new(n, m, init_codes, commands).solve();
    writeln!(stdout, "{ans}").ok();
}
