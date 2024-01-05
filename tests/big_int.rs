#[derive(Debug, PartialEq, Eq, Clone)]
struct BigInt {
    sign: bool,
    val: Vec<u64>,
}

impl BigInt {
    fn new(val: i64) -> Self {
        Self {
            sign: val >= 0,
            val: vec![val.abs() as u64],
        }
    }
}

fn cmp_vec(v0: &Vec<u64>, v1: &Vec<u64>) -> std::cmp::Ordering {
    if v0.len() > v1.len() {
        return std::cmp::Ordering::Greater;
    } else if v0.len() < v1.len() {
        return std::cmp::Ordering::Less;
    }

    let mut iter0 = v0.iter().rev();
    let mut iter1 = v1.iter().rev();

    loop {
        match (iter0.next(), iter1.next()) {
            (Some(x), Some(y)) => match x.cmp(y) {
                std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            },
            (None, None) => return std::cmp::Ordering::Equal,
            _ => panic!("unreachable"),
        }
    }
}

impl std::cmp::PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.sign == other.sign {
            match cmp_vec(&self.val, &other.val) {
                std::cmp::Ordering::Less => {
                    if self.sign {
                        Some(std::cmp::Ordering::Less)
                    } else {
                        Some(std::cmp::Ordering::Greater)
                    }
                }
                std::cmp::Ordering::Equal => Some(std::cmp::Ordering::Equal),
                std::cmp::Ordering::Greater => {
                    if self.sign {
                        Some(std::cmp::Ordering::Greater)
                    } else {
                        Some(std::cmp::Ordering::Less)
                    }
                }
            }
        } else if self.sign {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

impl std::ops::Neg for BigInt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            sign: !self.sign,
            val: self.val,
        }
    }
}

impl std::ops::Add for BigInt {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        if self.sign == rhs.sign {
            match self.val.len().cmp(&rhs.val.len()) {
                std::cmp::Ordering::Less => self.val.resize(rhs.val.len(), 0),
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => rhs.val.resize(self.val.len(), 0),
            }

            let mut carry = 0u128;
            for i in 0..self.val.len() {
                let sum = self.val[i] as u128 + rhs.val[i] as u128 + carry;
                self.val[i] = sum as u64;
                carry = sum >> 64;
            }
            if carry > 0 {
                self.val.push(carry as u64);
            }
            self
        } else if self.sign {
            rhs.sign = true;
            self - rhs
        } else {
            self.sign = true;
            rhs - self
        }
    }
}

impl std::ops::Sub for BigInt {
    type Output = Self;

    fn sub(mut self, mut rhs: Self) -> Self::Output {
        if self.sign == rhs.sign {
            match cmp_vec(&self.val, &rhs.val) {
                std::cmp::Ordering::Less => return -(rhs - self),
                std::cmp::Ordering::Equal => return Self::new(0),
                std::cmp::Ordering::Greater => (),
            }

            match self.val.len().cmp(&rhs.val.len()) {
                std::cmp::Ordering::Less => self.val.resize(rhs.val.len(), 0),
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => rhs.val.resize(self.val.len(), 0),
            }

            let mut borrow = 0i128;
            for i in 0..self.val.len() {
                let diff = self.val[i] as i128 - rhs.val[i] as i128 - borrow;
                self.val[i] = diff as u64;
                borrow = if diff < 0 { 1 } else { 0 };
            }

            while self.val.len() > 1 && self.val.last().unwrap() == &0 {
                self.val.pop();
            }
            self
        } else if self.sign {
            rhs.sign = true;
            self + rhs
        } else {
            self.sign = true;
            -(self + rhs)
        }
    }
}

// TODO: implement

#[test]
fn test_big_int() {
    let a = BigInt::new(123);
    let b = BigInt::new(-123);
    assert_eq!(a, -b);

    let a = BigInt::new(123);
    let b = BigInt::new(123);
    let c = BigInt::new(246);
    assert_eq!(a + b, c);

    let a = BigInt::new(i64::MAX);
    assert_eq!(
        a.clone() + a.clone() + a.clone(),
        BigInt {
            sign: true,
            val: vec![i64::MAX as u64 - 2, 1],
        }
    );

    let a = BigInt::new(246);
    let b = BigInt::new(123);
    let c = BigInt::new(123);
    assert_eq!(a - b, c);

    let a = BigInt::new(246);
    let b = BigInt::new(247);
    let c = BigInt::new(-1);
    assert_eq!(a - b, c);

    let a = BigInt::new(i64::MAX);
    assert_eq!(
        a.clone() - (a.clone() + a.clone() + a.clone() + a.clone()),
        -(a.clone() + a.clone() + a.clone())
    );
}
