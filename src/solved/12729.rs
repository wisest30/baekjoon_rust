use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_terminator('\n');
    let test_case = input.next().unwrap().parse::<usize>().unwrap();
    for tc in 1..=test_case {
        let n = input.next().unwrap().parse::<usize>().unwrap();
        let engines = (0..n).map(|_| input.next().unwrap()).collect::<Vec<_>>();
        let q = input.next().unwrap().parse::<usize>().unwrap();
        let queries = (0..q).map(|_| input.next().unwrap()).collect::<Vec<_>>();

        let mut map = engines
            .into_iter()
            .map(|engine| (engine, 0usize))
            .collect::<HashMap<_, _>>();
        for query in queries {
            let min = map
                .iter()
                .filter(|(key, _)| **key != query)
                .map(|(_, &val)| val)
                .min()
                .unwrap();

            *map.get_mut(&query).unwrap() = min + 1;
        }

        let ans = map.into_iter().map(|(_, val)| val).min().unwrap();
        println!("Case #{tc}: {ans}");
    }
}
