use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let test_case = input.next().unwrap().parse::<usize>().unwrap();
    for tc in 1..=test_case {
        let _ = input.next().unwrap().parse::<usize>().unwrap();
        let k = input.next().unwrap().parse::<usize>().unwrap();
        let l = input.next().unwrap().parse::<usize>().unwrap();
        let mut v = (0..l)
            .map(|_| input.next().unwrap().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        v.sort_unstable();

        let mut ans = 0;
        for i in 0..l {
            let x = v.pop().unwrap();
            ans += x * (1 + i / k);
        }

        println!("Case #{}: {}", tc, ans);
    }
}
