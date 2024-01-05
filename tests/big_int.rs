#[derive(Debug, PartialEq)]
struct BigInt {
    sign: bool,
    digits: Vec<u64>,
}

impl BigInt {
    fn new(val: i64) -> Self {
        Self {
            sign: val >= 0,
            digits: vec![val.abs() as u64],
        }
    }
}

// TODO: implement

#[test]
fn test_big_int() {
    let a = BigInt::new(123);
    let b = BigInt::new(123);

    assert_eq!(a, b);
}
