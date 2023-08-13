use std::fmt;
use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let m = input.next().unwrap().parse::<usize>().unwrap();
    let q = input.next().unwrap().parse::<usize>().unwrap();
    let mut row = vec![0; n];
    let mut col = vec![0; m];
    (0..q).for_each(|_| {
        let x = input.next().unwrap().parse::<usize>().unwrap();
        let y = input.next().unwrap().parse::<usize>().unwrap();
        let z = input.next().unwrap().parse::<i32>().unwrap();
        if x == 1 {
            row[y - 1] += z;
        } else {
            col[y - 1] += z;
        }
    });

    let mut ans = String::new();
    (0..n).for_each(|i| {
        (0..m).for_each(|j| {
            fmt::write(&mut ans, format_args!("{} ", row[i] + col[j])).ok();
        });
        fmt::write(&mut ans, format_args!("\n")).ok();
    });

    print!("{}", ans);
}
