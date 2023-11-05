pub struct Combination2 {
    mo: i64, // mod should be prime number and mod > n
    fact: Vec<i64>,
}

impl Combination2 {
    pub fn new(n: usize, mo: i64) -> Self {
        let mut fact = vec![0; n + 1];
        fact[0] = 1;
        for i in 1..=n {
            fact[i] = (fact[i - 1] * i as i64) % mo;
        }

        Self { mo, fact }
    }

    fn pow_mod(&self, mut x: i64, mut y: i64) -> i64 {
        let mut ret = 1;
        while y > 0 {
            if y & 1 == 1 {
                ret = (ret * x) % self.mo;
            }
            x = (x * x) % self.mo;
            y >>= 1;
        }
        ret
    }

    pub fn ncr(&self, n: usize, r: usize) -> i64 {
        let mut ret = self.fact[n];
        ret = (ret * self.pow_mod(self.fact[n - r], self.mo - 2)) % self.mo;
        ret = (ret * self.pow_mod(self.fact[r], self.mo - 2)) % self.mo;
        ret
    }
}

#[test]
fn test_combination2() {
    let comb = Combination2::new(4, 7);
    assert_eq!(comb.ncr(4, 0), 1);
    assert_eq!(comb.ncr(4, 1), 4);
    assert_eq!(comb.ncr(4, 2), 6);
    assert_eq!(comb.ncr(4, 3), 4);
    assert_eq!(comb.ncr(4, 4), 1);

    let comb = Combination2::new(4, 5);
    assert_eq!(comb.ncr(4, 0), 1);
    assert_eq!(comb.ncr(4, 1), 4);
    assert_eq!(comb.ncr(4, 2), 1);
    assert_eq!(comb.ncr(4, 3), 4);
    assert_eq!(comb.ncr(4, 4), 1);
}
