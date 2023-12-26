type T = i128;

fn pow_mod(mut x: T, mut exp: T, m: T) -> T {
    let mut ret = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            ret = ret * x % m;
        }
        x = x * x % m;
        exp >>= 1;
    }
    ret
}

// Sorenson과 Webster는 N이 3,317,011,064,679,887,385,961,981보다 작을 때,
// a가 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41일 때만 검증하면 된다고 밝힌 바 있다.
fn miller_rabin(n: T) -> bool {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
    if primes.contains(&n) {
        return true;
    }
    if n == 1 || n % 2 == 0 {
        return false;
    }

    primes.iter().all(|&prime| {
        let mut s = 0;
        let mut d = n - 1;
        while d % 2 == 0 {
            d /= 2;
            s += 1;
        }
        let x = pow_mod(prime, d, n);
        if x == 1 || x + 1 == n {
            return true;
        }
        for _ in 0..s - 1 {
            let x = x * x % n;
            if x + 1 == n {
                return true;
            }
        }
        false
    })
}

#[test]
fn test_miller_rabin() {
    assert!(!miller_rabin(1));
    assert!(miller_rabin(2));
    assert!(miller_rabin(3));
    assert!(!miller_rabin(4));
    assert!(miller_rabin(5));
    assert!(!miller_rabin(6));
    assert!(miller_rabin(7));
    assert!(!miller_rabin(8));
    assert!(!miller_rabin(9));
    assert!(!miller_rabin(10));
    assert!(miller_rabin(11));
    assert!(miller_rabin(1_000_000_007));
    assert!(!miller_rabin(1_000_000_008));
}
