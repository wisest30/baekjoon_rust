type T = i128;

fn eulers_phi(n: T, divs: &[T]) -> T {
    divs.iter().fold(n, |acc, &d| acc / d * (d - 1))
}

#[test]
fn test_eulers_phi() {
    assert_eq!(eulers_phi(1, &[]), 1);
    assert_eq!(eulers_phi(2, &[2]), 1);
    assert_eq!(eulers_phi(3, &[3]), 2);
    assert_eq!(eulers_phi(4, &[2]), 2);
    assert_eq!(eulers_phi(5, &[5]), 4);
    assert_eq!(eulers_phi(6, &[2, 3]), 2);
    assert_eq!(eulers_phi(7, &[7]), 6);
    assert_eq!(eulers_phi(8, &[2]), 4);
    assert_eq!(eulers_phi(9, &[3]), 6);
    assert_eq!(eulers_phi(10, &[2, 5]), 4);
    assert_eq!(eulers_phi(11, &[11]), 10);
    assert_eq!(eulers_phi(12, &[2, 3]), 4);
    assert_eq!(eulers_phi(13, &[13]), 12);
    assert_eq!(eulers_phi(14, &[2, 7]), 6);
    assert_eq!(eulers_phi(15, &[3, 5]), 8);
    assert_eq!(eulers_phi(16, &[2]), 8);
    assert_eq!(eulers_phi(17, &[17]), 16);
    assert_eq!(eulers_phi(18, &[2, 3]), 6);
    assert_eq!(eulers_phi(19, &[19]), 18);
    assert_eq!(eulers_phi(20, &[2, 5]), 8);
    assert_eq!(eulers_phi(21, &[3, 7]), 12);
    assert_eq!(eulers_phi(22, &[2, 11]), 10);
    assert_eq!(eulers_phi(23, &[23]), 22);
    assert_eq!(eulers_phi(24, &[2, 3]), 8);
    assert_eq!(eulers_phi(25, &[5]), 20);
    assert_eq!(eulers_phi(26, &[2, 13]), 12);
    assert_eq!(eulers_phi(27, &[3]), 18);
    assert_eq!(eulers_phi(28, &[2, 7]), 12);
    assert_eq!(eulers_phi(29, &[29]), 28);
    assert_eq!(eulers_phi(100, &[2, 5]), 40);
}
