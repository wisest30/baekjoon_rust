use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let test_case_count = input.next().unwrap().parse::<usize>().unwrap();
    for test_case in 1..=test_case_count {
        let n = input.next().unwrap().parse::<usize>().unwrap();
        let books = (0..n)
            .map(|_| input.next().unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut odds = Vec::new();
        let mut evens = Vec::new();
        for &book in &books {
            if book % 2 == 0 {
                evens.push(book);
            } else {
                odds.push(book);
            }
        }

        odds.sort_unstable();
        evens.sort_unstable();

        let mut odds_iter = odds.into_iter();
        let mut evens_iter = evens.into_iter().rev();
        let mut ans = String::new();
        for book in books {
            if book % 2 == 0 {
                ans.push_str(&evens_iter.next().unwrap().to_string());
            } else {
                ans.push_str(&odds_iter.next().unwrap().to_string());
            }

            ans.push(' ');
        }

        println!("Case #{}: {}", test_case, ans);
    }
}
