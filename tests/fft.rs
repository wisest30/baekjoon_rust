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

// assert size of 'a' is 2 ^ (n + 1). {original vector size} <= 2 ^ n.
// x0 : 1의 n 제곱근.
// reference : https://m.blog.naver.com/kks227/221633584963
fn fft(mut a: Vec<Complex<f64>>, w: Complex<f64>) -> Vec<Complex<f64>> {
    if a.len() == 1 {
        return a;
    }

    let even = (0..a.len()).step_by(2).map(|i| a[i]).collect::<Vec<_>>();
    let odd = (1..a.len()).step_by(2).map(|i| a[i]).collect::<Vec<_>>();

    let even_fft = fft(even, w * w);
    let odd_fft = fft(odd, w * w);

    let mut w_pow = Complex::new(1.0, 0.0);
    let a_len = a.len();
    for i in 0..a.len() / 2 {
        a[i] = even_fft[i] + w_pow * odd_fft[i];
        a[i + a_len / 2] = even_fft[i] - w_pow * odd_fft[i];
        w_pow = w_pow * w;
    }

    a
}

fn idft(a: Vec<Complex<f64>>, w: Complex<f64>) -> Vec<Complex<f64>> {
    let mut a = fft(a, Complex::new(1.0, 0.0) / w);
    let p = Complex::new(a.len() as f64, 0.0);
    for i in 0..a.len() {
        a[i] = a[i] / p;
    }

    a
}

fn fft_multiply(a: &Vec<Complex<f64>>, b: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    use std::f64::consts::PI;

    let ori_sz = a.len().max(b.len());
    let mut sz = 1usize;
    while sz < ori_sz {
        sz <<= 1;
    }
    sz <<= 1;

    let mut a = a.clone();
    a.resize(sz, Complex::new(0.0, 0.0));
    let mut b = b.clone();
    b.resize(sz, Complex::new(0.0, 0.0));

    let n = sz as f64;
    let w = Complex::new(f64::cos(2.0 * PI / n), f64::sin(2.0 * PI / n));
    let a_fft = fft(a, w);
    let b_fft = fft(b, w);
    let c_fft = a_fft
        .into_iter()
        .zip(b_fft.into_iter())
        .map(|(ai, bi)| ai * bi)
        .collect::<Vec<_>>();

    idft(c_fft, w)
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

    let mut c = fft_multiply(&a, &b);
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
