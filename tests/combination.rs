struct Combination {
    v: Vec<Vec<u64>>,
}

impl Combination {
    pub fn new(n: u64) -> Combination {
        assert!(n > 0);
        assert!(n < 66); // overflow!!

        let mut v = vec![vec![0; n as usize + 1]; n as usize + 1];
        v[0][0] = 1;
        v[1][0] = 1;
        v[1][1] = 1;

        for i in 2..=n as usize {
            v[i][0] = 1;
            for j in 1..=n as usize {
                v[i][j] = v[i - 1][j - 1] + v[i - 1][j];
            }
        }

        Combination { v }
    }

    pub fn new_mod(n: u64, m: u64) -> Combination {
        assert!(n > 0);
        assert!(n < 66); // overflow!!
        assert!(m > 0);

        let mut v = vec![vec![0; n as usize + 1]; n as usize + 1];
        if m == 1 {
            return Combination { v };
        }

        v[0][0] = 1;
        v[1][0] = 1;
        v[1][1] = 1;

        for i in 2..=n as usize {
            v[i][0] = 1;
            for j in 1..=n as usize {
                v[i][j] = (v[i - 1][j - 1] + v[i - 1][j]) % m;
            }
        }

        Combination { v }
    }

    pub fn ncr(&self, n: usize, r: usize) -> u64 {
        assert!(n < self.v.len());
        assert!(r <= n);
        self.v[n][r]
    }
}

#[test]
fn test_combination() {
    let comb = Combination::new(3);
    assert_eq!(comb.ncr(3, 0), 1);
    assert_eq!(comb.ncr(3, 1), 3);
    assert_eq!(comb.ncr(3, 2), 3);
    assert_eq!(comb.ncr(3, 3), 1);

    let comb = Combination::new_mod(3, 2);
    assert_eq!(comb.ncr(3, 0), 1);
    assert_eq!(comb.ncr(3, 1), 1);
    assert_eq!(comb.ncr(3, 2), 1);
    assert_eq!(comb.ncr(3, 3), 1);
}
