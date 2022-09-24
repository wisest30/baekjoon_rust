pub fn get_primes(n: usize) -> Vec<i64> {
    let mut v = vec![true; n];
    let mut ret = Vec::new();
    for i in 2..n {
        if v[i] {
            ret.push(i as i64);
            for j in (2 * i..n).step_by(i) {
                v[j] = false;
            }
        }
    }

    ret
}

#[test]
fn test_get_primes() {
    let v = get_primes(100);
    assert_eq!(
        v,
        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97
        ]
    );
}
