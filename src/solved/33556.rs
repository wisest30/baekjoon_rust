use std::io::{Read, Write};

enum JavaString {
    Null,
    Str(String),
}

impl JavaString {
    fn new(input: String) -> Self {
        if input == "null" {
            Self::Null
        } else {
            Self::Str(input)
        }
    }

    fn equals(&self, other: &JavaString) -> Option<bool> {
        match self {
            JavaString::Null => None,
            JavaString::Str(s) => match other {
                JavaString::Null => Some(false),
                JavaString::Str(t) => Some(s == t),
            },
        }
    }

    fn equals_ignore_case(&self, other: &JavaString) -> Option<bool> {
        match self {
            JavaString::Null => None,
            JavaString::Str(s) => match other {
                JavaString::Null => Some(false),
                JavaString::Str(t) => Some(s.to_ascii_lowercase() == t.to_ascii_lowercase()),
            },
        }
    }
}

struct Problem {
    a: String,
    b: String,
}

impl Problem {
    fn new(a: String, b: String) -> Self {
        Self { a, b }
    }

    fn solve(&mut self) -> String {
        let a = JavaString::new(self.a.clone());
        let b = JavaString::new(self.b.clone());

        fn res_to_str(res: Option<bool>) -> &'static str {
            match res {
                Some(true) => "true",
                Some(false) => "false",
                None => "NullPointerException",
            }
        }

        let ans0 = res_to_str(a.equals(&b));
        let ans1 = res_to_str(a.equals_ignore_case(&b));
        format!("{ans0}\n{ans1}")
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let a = input.next().unwrap().to_string();
    let b = input.next().unwrap().to_string();
    let ans = Problem::new(a, b).solve();
    writeln!(stdout, "{ans}").ok();
}
