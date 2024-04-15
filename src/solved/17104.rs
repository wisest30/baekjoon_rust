use std::io::{Read, Write};
use std::ops::{Add, Div, Mul, Sub};

const MAX_N: usize = 1usize << 20;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Complex<T> {
    real: T,
    im: T,
}

impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            im: self.im + other.im,
        }
    }
}

impl<T> Sub for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            im: self.im - other.im,
        }
    }
}

impl<T> Mul for Complex<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real * other.real - self.im * other.im,
            im: self.real * other.im + self.im * other.real,
        }
    }
}

impl<T> Div for Complex<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let p = other.real * other.real + other.im * other.im;
        Self {
            real: (self.real * other.real + self.im * other.im) / p,
            im: (self.im * other.real - self.real * other.im) / p,
        }
    }
}

impl<T> Complex<T> {
    fn new(real: T, im: T) -> Complex<T> {
        Self { real, im }
    }
}

// reference : https://cp-algorithms.com/algebra/fft.html
fn fft(mut a: Vec<Complex<f64>>, invert: bool) -> Vec<Complex<f64>> {
    use std::f64::consts::PI;

    fn reverse(num: usize, lg_n: usize) -> usize {
        (0..lg_n)
            .filter(|i| (num & (1 << i)) != 0)
            .map(|i| 1 << (lg_n - 1 - i))
            .fold(0usize, |acc, x| acc ^ x)
    }

    let mut lg_n = 0usize;
    while (1 << lg_n) < a.len() {
        lg_n += 1;
    }

    if a.len() < (1 << lg_n) {
        a.resize(1 << lg_n, Complex::new(0.0, 0.0));
    }

    for i in 0..a.len() {
        let j = reverse(i, lg_n);
        if i < j {
            (a[i], a[j]) = (a[j], a[i]);
        }
    }

    let mut len = 2;
    while len <= a.len() {
        let ang = 2.0 * PI / len as f64 * if invert { -1.0 } else { 1.0 };
        let w = Complex::new(f64::cos(ang), f64::sin(ang));
        for i in (0..a.len()).step_by(len) {
            let mut w_pow = Complex::new(1.0, 0.0);
            for j in 0..len / 2 {
                let (u, v) = (a[i + j], a[i + j + len / 2] * w_pow);
                (a[i + j], a[i + j + len / 2]) = (u + v, u - v);
                w_pow = w_pow * w;
            }
        }

        len <<= 1;
    }

    if invert {
        let a_len = a.len() as f64;
        for x in &mut a {
            x.real /= a_len;
            x.im /= a_len;
        }
    }

    a
}

struct Problem {
    c: Vec<i64>,
}

pub fn get_primes(n: usize) -> Vec<i64> {
    let mut v = vec![true; n];
    let mut ret = Vec::new();
    for i in 2..n {
        if v[i] {
            ret.push(i as i64);
            for j in (2 * i..n).step_by(i) {
                v[j] = false;
            }
        }
    }

    ret
}

impl Problem {
    fn new() -> Self {
        Problem {
            c: Self::generate_c(),
        }
    }

    fn generate_c() -> Vec<i64> {
        let mut is_prime = vec![true; MAX_N];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..MAX_N {
            if is_prime[i] {
                for j in (2 * i..MAX_N).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }

        let mut a = (0..MAX_N / 2)
            .map(|i| Complex::new(if is_prime[2 * i + 1] { 1.0 } else { 0.0 }, 0.0))
            .collect::<Vec<_>>();
        a.resize(MAX_N, Complex::new(0.0, 0.0));

        let a_fft = fft(a, false);
        let c_fft = a_fft.into_iter().map(|ai| ai * ai).collect::<Vec<_>>();
        let c = fft(c_fft, true);
        c.into_iter()
            .map(|c| c.real.round() as i64)
            .map(|x| x / 2 + x % 2)
            .collect::<Vec<_>>()
    }

    fn solve(&self, ns: Vec<usize>) -> String {
        ns.into_iter()
            .map(|n| n / 2 - 1)
            .map(|n| if n == 1 { 1 } else { self.c[n] }.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let t = input.next().unwrap().parse::<usize>().unwrap();
    let ns = (0..t)
        .map(|_| input.next().unwrap().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let ans = Problem::new().solve(ns);

    writeln!(stdout, "{}", ans).ok();
}
