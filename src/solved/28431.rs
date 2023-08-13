use std::io::Read;
use std::ops::BitXor;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let ans = (0..5)
        .map(|_| input.next().unwrap().parse::<i32>().unwrap())
        .fold(0, i32::bitxor);

    println!("{ans}");
}
