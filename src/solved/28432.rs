use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let words = (0..n).map(|_| input.next().unwrap()).collect::<Vec<_>>();

    let p = words
        .iter()
        .enumerate()
        .find_map(|(i, &w)| if w == "?" { Some(i) } else { None })
        .unwrap();
    let first = words.get(p - 1).map(|w| w.chars().last().unwrap());
    let last = words.get(p + 1).map(|w| w.chars().next().unwrap());

    let m = input.next().unwrap().parse::<usize>().unwrap();
    let ans = (0..m)
        .map(|_| input.next().unwrap())
        .filter(|w| !words.contains(w))
        .find(|&w| {
            (first.is_none() || w.chars().next().unwrap() == first.unwrap())
                && (last.is_none() || w.chars().last().unwrap() == last.unwrap())
        })
        .unwrap();

    println!("{ans}");
}
