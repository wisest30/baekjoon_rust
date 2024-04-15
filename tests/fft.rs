use std::ops::{Add, Div, Mul, Sub};

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

fn fft_multiply(mut a: Vec<Complex<f64>>, mut b: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let mut n = 1usize;
    while n < a.len() + b.len() {
        n <<= 1;
    }
    a.resize(n, Complex::new(0.0, 0.0));
    b.resize(n, Complex::new(0.0, 0.0));

    let a_fft = fft(a, false);
    let b_fft = fft(b, false);
    let c_fft = a_fft
        .into_iter()
        .zip(b_fft.into_iter())
        .map(|(ai, bi)| ai * bi)
        .collect::<Vec<_>>();

    fft(c_fft, true)
}

#[test]
fn test_fft() {
    let a = vec![
        Complex::new(1.0, 0.0),
        Complex::new(2.0, 0.0),
        Complex::new(3.0, 0.0),
    ];

    let b = vec![
        Complex::new(4.0, 0.0),
        Complex::new(5.0, 0.0),
        Complex::new(6.0, 0.0),
    ];

    let mut c = fft_multiply(a, b);
    c.iter_mut().for_each(|c| {
        c.real = c.real.round();
        c.im = c.im.round();
    });

    assert_eq!(
        c,
        vec![
            Complex { real: 4.0, im: 0.0 },
            Complex {
                real: 13.0,
                im: 0.0
            },
            Complex {
                real: 28.0,
                im: 0.0
            },
            Complex {
                real: 27.0,
                im: 0.0
            },
            Complex {
                real: 18.0,
                im: 0.0
            },
            Complex { real: 0.0, im: 0.0 },
            Complex { real: 0.0, im: 0.0 },
            Complex { real: 0.0, im: 0.0 }
        ]
    );
}
