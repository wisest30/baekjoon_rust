fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    let heights = (0..n)
        .map(|_| {
            stdin.read_line(&mut buf).unwrap();
            let h = buf.trim().parse::<i64>().unwrap();
            buf.clear();
            h
        })
        .collect::<Vec<_>>();

    let mut left = vec![0i64; n];
    let mut right = vec![n as i64; n];
    let mut stack = Vec::new();
    for i in 0..n {
        while let Some(&j) = stack.last() {
            if heights[j] < heights[i] {
                break;
            } else {
                right[j] = i as i64;
                stack.pop();
            }
        }

        if let Some(&j) = stack.last() {
            left[i] = j as i64 + 1;
        }

        stack.push(i);
    }

    let ans = (0..n)
        .map(|i| (right[i] - left[i]) * heights[i])
        .max()
        .unwrap();

    println!("{}", ans);
}
