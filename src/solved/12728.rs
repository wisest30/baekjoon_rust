use std::io::Read;

type ElemT = i64;
pub fn matrix_mul(a: &Vec<Vec<ElemT>>, b: &Vec<Vec<ElemT>>, mod_: ElemT) -> Vec<Vec<ElemT>> {
    let n = a.len();
    assert!(n > 0);

    let m = a[0].len();
    assert!(m > 0);
    assert!(b.len() == m);

    let l = b[0].len();
    assert!(l > 0);

    let mut ret = vec![vec![0; l]; n];
    for i in 0..n {
        for j in 0..l {
            for k in 0..m {
                ret[i][j] += a[i][k] * b[k][j] % mod_;
                ret[i][j] %= mod_;
            }
        }
    }

    ret
}

type ExpT = i64;
pub fn matrix_exp(a: &Vec<Vec<ElemT>>, e: ExpT, mod_: ElemT) -> Vec<Vec<ElemT>> {
    assert!(e > 0);
    if e == 1 {
        a.clone()
    } else if e % 2 == 1 {
        matrix_mul(&matrix_exp(a, e - 1, mod_), a, mod_)
    } else {
        let aa = matrix_exp(&a, e / 2, mod_);
        matrix_mul(&aa, &aa, mod_)
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let test_case = input.next().unwrap().parse::<usize>().unwrap();

    const MOD: i64 = 1000;
    let a = vec![vec![3i64, 5], vec![1, 3]];
    for tc in 1..=test_case {
        let n = input.next().unwrap().parse::<i64>().unwrap();
        let an = matrix_exp(&a, n, MOD);
        let ans = (2 * an[0][0] + MOD - 1) % MOD;
        println!("Case #{tc}: {:03}", ans);
    }
}
