pub fn linear_sieve_eratosthenes(n: i64) -> Vec<i64> {
    let mut minimum_prime_factor = vec![0; (n + 1) as usize];
    let mut ret = Vec::new();
    for i in 2..=n {
        if minimum_prime_factor[i as usize] == 0 {
            minimum_prime_factor[i as usize] = i;
            ret.push(i);
        }
        for &prime in ret.iter() {
            let j = i * prime;
            if j > n || prime > minimum_prime_factor[i as usize] {
                break;
            }
            minimum_prime_factor[j as usize] = prime;
        }
    }

    ret
}

#[test]
fn test_linear_sieve_eratosthenes() {
    let v = linear_sieve_eratosthenes(100);
    assert_eq!(
        v,
        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97
        ]
    );
}
