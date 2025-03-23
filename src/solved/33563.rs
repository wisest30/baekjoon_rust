use std::io::{Read, Write};

struct Problem {
    n: usize,
    registers: Vec<Vec<char>>,
    target: String,
}

impl Problem {
    fn new(n: usize, mut registers: Vec<Vec<char>>, target: String) -> Self {
        registers.resize(100, Vec::new());

        Self {
            n,
            registers,
            target,
        }
    }

    fn sub_solve(&mut self, commands: &mut Vec<String>, t: Vec<char>) -> bool {
        const INVALID_POS: usize = 100usize;

        let mut stack = self.n + 2;

        let mut pos = vec![0usize; 0];
        for i in (0..8).step_by(2) {
            if t[i] == '-' {
                continue;
            }

            let uncolor = t[i + 1] == 'u';
            let mut j = INVALID_POS;
            let mut k = INVALID_POS;
            for y in 0..self.n {
                for x in (0..8).step_by(2) {
                    if self.registers[y][x] == t[i] && (!uncolor || self.registers[y][x + 1] == 'u')
                    {
                        j = y;
                        k = x;
                        break;
                    }
                }

                if j != INVALID_POS {
                    break;
                }
            }

            if j == INVALID_POS {
                return false;
            }

            match k {
                0 => {
                    commands.push(format!("2 {} {} 2", j + 1, stack));
                    stack += 1;
                    commands.push(format!("2 {} {} 2", stack - 1, stack));
                    stack += 1;
                }
                2 => {
                    commands.push(format!("2 {} {stack} 3", j + 1));
                    stack += 1;
                }
                4 => {
                    commands.push(format!("2 {} {stack} 2", j + 1));
                    stack += 1;
                }
                6 => {
                    commands.push(format!("2 {} {stack} 1", j + 1));
                    stack += 1;
                }
                _ => panic!("wrong index"),
            }

            commands.push(format!("1 {} {} {}", stack - 1, stack, stack + 1));
            stack += 2;

            commands.push(format!("2 {} {} 1", stack - 1, stack));
            stack += 1;

            commands.push(format!("1 {} {} {}", stack - 1, stack, stack + 1));
            stack += 2;

            match i {
                0 => {
                    commands.push(format!("2 {} {} 3", stack - 1, stack));
                    stack += 1;
                }
                2 => {
                    commands.push(format!("2 {} {} 2", stack - 1, stack));
                    stack += 1;
                    commands.push(format!("2 {} {} 2", stack - 1, stack));
                    stack += 1;
                }
                4 => {
                    commands.push(format!("2 {} {} 1", stack - 1, stack));
                    stack += 1;
                }
                6 => {
                    commands.push(format!("2 {} {} 2", stack - 1, stack));
                    stack += 1;
                }
                _ => panic!("wrong index"),
            }

            if !uncolor {
                commands.push(format!("4 {} {} {}", stack - 1, stack, t[i + 1]));
                stack += 1;
            }

            pos.push(stack - 1);
        }

        if pos.is_empty() {
            panic!("empty pos");
        }

        if pos.len() == 1 {
            commands.push(format!("2 {} {} 2", pos[0], stack));
            stack += 1;
            commands.push(format!("2 {} {} 2", stack - 1, self.n + 1));
        } else {
            for i in 0..pos.len() - 1 {
                if i == 0 {
                    commands.push(format!("3 {} {} {}", pos[i], pos[i + 1], stack));
                    stack += 1;
                } else {
                    commands.push(format!("3 {} {} {}", stack - 1, pos[i + 1], stack));
                    stack += 1;
                }
            }

            commands.push(format!("2 {} {} 2", stack - 1, stack));
            stack += 1;
            commands.push(format!("2 {} {} 2", stack - 1, self.n + 1));
        }

        self.registers[self.n] = t;
        self.n += 1;

        true
    }

    fn solve(&mut self) -> String {
        let mut commands = vec![String::new(); 0];
        let targets = self
            .target
            .split(':')
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let target_cnt = targets.len();
        for t in targets {
            let success = self.sub_solve(&mut commands, t);
            if !success {
                return "-1".to_string();
            }
        }

        let pos = (0..target_cnt)
            .map(|i| self.n - i)
            .rev()
            .collect::<Vec<_>>();

        let mut stack = self.n + 1;
        if pos.len() == 1 {
            commands.push(format!("2 {} 99 2", pos[0]));
            commands.push(format!("2 99 100 2"));
        } else {
            for i in 0..pos.len() - 1 {
                if i == 0 {
                    commands.push(format!("3 {} {} {}", pos[i], pos[i + 1], stack));
                    stack += 1;
                } else {
                    commands.push(format!("3 {} {} {}", stack - 1, pos[i + 1], stack));
                    stack += 1;
                }
            }

            commands.push(format!("2 {} {} 2", stack - 1, stack));
            stack += 1;
            commands.push(format!("2 {} {} 2", stack - 1, 100));
        }

        let commands_str = commands.join("\n");
        format!("{}\n{}", commands.len(), commands_str)
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let n = input.next().unwrap().parse::<usize>().unwrap();
    let registers = (0..n)
        .map(|_| input.next().unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let target = input.next().unwrap().to_string();

    let ans = Problem::new(n, registers, target).solve();
    writeln!(stdout, "{ans}").ok();
}
