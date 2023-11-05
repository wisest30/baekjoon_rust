pub fn get_minimum_prime_factors(n: i64) -> Vec<i64> {
    let mut minimum_prime_factor = vec![0; (n + 1) as usize];
    let mut primes = Vec::new();
    for i in 2..=n {
        if minimum_prime_factor[i as usize] == 0 {
            minimum_prime_factor[i as usize] = i;
            primes.push(i);
        }
        for &prime in primes.iter() {
            let j = i * prime;
            if j > n || prime > minimum_prime_factor[i as usize] {
                break;
            }
            minimum_prime_factor[j as usize] = prime;
        }
    }

    minimum_prime_factor
}

pub fn mobius_function(n: i64) -> Vec<i64> {
    let minimum_prime_factor = get_minimum_prime_factors(n);
    let mut mobius_function = vec![0; (n + 1) as usize];
    mobius_function[1] = 1;
    for i in 2..=n {
        let j = i / minimum_prime_factor[i as usize];
        if minimum_prime_factor[i as usize] == minimum_prime_factor[j as usize] {
            mobius_function[i as usize] = 0;
        } else {
            mobius_function[i as usize] = -mobius_function[j as usize];
        }
    }
    mobius_function
}

#[test]
fn test_mobius_function() {
    let v = mobius_function(100);
    assert_eq!(
        v,
        vec![
            0, 1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1, 0, 1, 1, -1, 0, 0,
            1, 0, 0, -1, -1, -1, 0, 1, 1, 1, 0, -1, 1, 1, 0, -1, -1, -1, 0, 0, 1, -1, 0, 0, 0, 1,
            0, -1, 0, 1, 0, 1, 1, -1, 0, -1, 1, 0, 0, 1, -1, -1, 0, 1, -1, -1, 0, -1, 1, 0, 0, 1,
            -1, -1, 0, 0, 1, -1, 0, 1, 1, 1, 0, -1, 0, 1, 0, 1, 1, 1, 0, -1, 0, 0, 0
        ]
    );
}
