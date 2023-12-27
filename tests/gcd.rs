type T = i128;

fn gcd(mut x: T, mut y: T) -> T {
    while y > 0 {
        x = x % y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

#[test]
fn test_gcd() {
    assert!(gcd(1, 1) == 1);
    assert!(gcd(1, 2) == 1);
    assert!(gcd(2, 1) == 1);
    assert!(gcd(10, 15) == 5);
    assert!(gcd(100, 100) == 100);
    assert!(gcd(100, 101) == 1);
}
