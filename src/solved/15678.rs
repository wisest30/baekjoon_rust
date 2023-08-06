use std::collections::BinaryHeap;
use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let n = input.next().unwrap().parse::<usize>().unwrap();
    let d = input.next().unwrap().parse::<usize>().unwrap();
    let nums = (0..n)
        .map(|_| input.next().unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut heap = BinaryHeap::new();
    let answer = nums
        .into_iter()
        .enumerate()
        .map(|(i, i_num)| {
            let mut i_max_val = i_num;
            while let Some(&(j_max_val, j)) = heap.peek() {
                if i - j > d {
                    heap.pop();
                } else {
                    i_max_val = i_max_val.max(j_max_val + i_num);
                    break;
                }
            }
            heap.push((i_max_val, i));
            i_max_val
        })
        .max()
        .unwrap();

    println!("{}", answer);
}
