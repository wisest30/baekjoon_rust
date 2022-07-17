use std::fmt::Write;

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    let nums = (0..n)
        .map(|_| {
            stdin.read_line(&mut buf).unwrap();
            let num = buf.trim().parse::<usize>().unwrap();
            buf.clear();

            num - 1
        })
        .collect::<Vec<_>>();

    let mut ans = Vec::new();
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            let mut vis_list = vec![i];
            visited[i] = true;
            while let Some(&j) = vis_list.last() {
                let j = nums[j];
                if !visited[j] {
                    vis_list.push(j);
                    visited[j] = true;
                } else {
                    if i == j {
                        ans.extend(vis_list);
                    } else {
                        for k in vis_list {
                            visited[k] = false;
                        }
                    }

                    break;
                }
            }
        }
    }

    ans.sort_unstable();

    writeln!(&mut buf, "{}", ans.len()).unwrap();
    ans.into_iter().for_each(|i| {
        writeln!(&mut buf, "{}", i + 1).unwrap();
    });

    println!("{}", buf);
}
