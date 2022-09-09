use std::collections::BinaryHeap;
use std::io::Read;

fn time_to_int(time: &str) -> i32 {
    let (hour, min) = time.split_once(':').unwrap();
    let hour = hour.parse::<i32>().unwrap();
    let min = min.parse::<i32>().unwrap();

    hour * 60 + min
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_terminator('\n');
    let test_case = input.next().unwrap().parse::<usize>().unwrap();
    for tc in 1..=test_case {
        let t = input.next().unwrap().parse::<i32>().unwrap();
        let (na, nb) = input.next().unwrap().split_once(' ').unwrap();
        let na = na.parse::<usize>().unwrap();
        let nb = nb.parse::<usize>().unwrap();

        const AT_A: i32 = 0;
        const AT_B: i32 = 1;
        let mut v = (0..na + nb)
            .map(|i| {
                let (start, end) = input.next().unwrap().split_once(' ').unwrap();
                let start = time_to_int(start);
                let end = time_to_int(end);

                (start, end, if i < na { AT_A } else { AT_B })
            })
            .collect::<Vec<_>>();

        v.sort();

        let mut heap_a: BinaryHeap<i32> = BinaryHeap::new();
        let mut heap_b: BinaryHeap<i32> = BinaryHeap::new();

        let mut ans = (0, 0);
        for e in v {
            let (start, end, at) = e;
            match at {
                AT_A => match heap_a.peek() {
                    Some(&time) if -time <= start => {
                        let _ = heap_a.pop().unwrap();
                        heap_b.push(-(end + t));
                    }
                    _ => {
                        ans.0 += 1;
                        heap_b.push(-(end + t));
                    }
                },
                AT_B => match heap_b.peek() {
                    Some(&time) if -time <= start => {
                        let _ = heap_b.pop().unwrap();
                        heap_a.push(-(end + t));
                    }
                    _ => {
                        ans.1 += 1;
                        heap_a.push(-(end + t));
                    }
                },
                _ => panic!(),
            }
        }

        println!("Case #{tc}: {} {}", ans.0, ans.1);
    }
}
