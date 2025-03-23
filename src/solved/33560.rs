use std::io::{Read, Write};

struct State {
    cur_score: i32,
    cur_time: i32,
    add_score: i32,
    add_time: i32,
}

impl State {
    fn new() -> Self {
        Self {
            cur_score: 0,
            cur_time: 0,
            add_score: 1,
            add_time: 4,
        }
    }

    fn get_reward(&mut self) -> Option<usize> {
        if self.cur_score < 0 {
            panic!("wrong current score")
        } else if self.cur_score < 35 {
            None
        } else if self.cur_score < 65 {
            Some(0)
        } else if self.cur_score < 95 {
            Some(1)
        } else if self.cur_score < 125 {
            Some(2)
        } else {
            Some(3)
        }
    }

    fn half_add_score_or_inc_add_time(&mut self) {
        match self.add_score.cmp(&1) {
            std::cmp::Ordering::Less => panic!("wrong add score"),
            std::cmp::Ordering::Equal => self.add_time += 2,
            std::cmp::Ordering::Greater => self.add_score >>= 1,
        }
    }

    fn add_cur_time(&mut self, t: i32) {
        self.cur_time += t;
    }

    fn dec_add_time(&mut self) {
        match self.add_time.cmp(&1) {
            std::cmp::Ordering::Less => panic!("wrong add time"),
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => self.add_time -= 1,
        }
    }

    fn double_add_score(&mut self) {
        match self.add_score.cmp(&32) {
            std::cmp::Ordering::Less => self.add_score <<= 1,
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => panic!("wrong add score"),
        }
    }

    fn end_turn(&mut self) {
        self.cur_score += self.add_score;
        self.cur_time += self.add_time;
    }

    fn get_cur_time(&self) -> i32 {
        self.cur_time
    }
}

struct Problem {
    _n: usize,
    dice_values: Vec<i32>,
}

impl Problem {
    fn new(n: usize, dice_values: Vec<i32>) -> Self {
        Self { _n: n, dice_values }
    }

    fn solve(&self) -> String {
        let mut state = State::new();
        let mut ans = [0; 4];

        let quit_game = |state: &mut State, ans: &mut [i32]| {
            if let Some(reward) = state.get_reward() {
                ans[reward] += 1;
            }

            *state = State::new();
        };

        for &d in &self.dice_values {
            if state.get_cur_time() > 240 {
                quit_game(&mut state, &mut ans);
            }

            match d {
                1 => {
                    quit_game(&mut state, &mut ans);
                    continue;
                }
                2 => state.half_add_score_or_inc_add_time(),
                3 => (),
                4 => state.add_cur_time(56),
                5 => state.dec_add_time(),
                6 => state.double_add_score(),
                _ => panic!("wrong dice value"),
            }
            state.end_turn();
        }

        format!("{}\n{}\n{}\n{}", ans[0], ans[1], ans[2], ans[3])
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let n = input.next().unwrap().parse::<usize>().unwrap();
    let dice_values = (0..n)
        .map(|_| input.next().unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let ans = Problem::new(n, dice_values).solve();
    writeln!(stdout, "{ans}").ok();
}
