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

#[test]
fn test_pow_mod() {
    assert!(pow_mod(2, 3, 10) == 8);
    assert!(pow_mod(2, 10, 1025) == 1024);
    assert!(pow_mod(2, 10, 1000) == 24);
}
