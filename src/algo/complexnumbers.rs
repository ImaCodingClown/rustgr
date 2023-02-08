#[derive(Debug, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

#[allow(non_camel_case_types)]
pub type j64 = Complex;

impl std::ops::Add for Complex {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Add<f64> for Complex {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re + rhs,
            im: self.im,
        }
    }
}

impl std::ops::Add<Complex> for f64 {
    type Output = Complex;

    #[inline]
    fn add(self, rhs: Complex) -> Complex {
        Complex {
            re: self + rhs.re,
            im: rhs.im,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl std::ops::Sub<f64> for Complex {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re - rhs,
            im: self.im,
        }
    }
}

impl std::ops::Sub<Complex> for f64 {
    type Output = Complex;

    #[inline]
    fn sub(self, rhs: Complex) -> Complex {
        Complex {
            re: self - rhs.re,
            im: 0.0 - rhs.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl std::ops::Mul<Complex> for f64 {
    type Output = Complex;

    #[inline]
    fn mul(self, rhs: Complex) -> Complex {
        Complex {
            re: self * rhs.re,
            im: self * rhs.im,
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let a = self.re;
        let b = self.im;
        let c = rhs.re;
        let d = rhs.im;
        let denominator = c.powf(2.0) + d.powf(2.0);
        Self {
            re: (a*c+b*d)/denominator,
            im: (b*c-a*d)/denominator,
        }
    }
}

impl std::ops::Div<f64> for Complex {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl std::ops::Div<Complex> for f64 {
    type Output = Complex;

    #[inline]
    fn div(self, rhs: Complex) -> Complex {
        let a = self;
        let c = rhs.re;
        let d = rhs.im;
        let denominator = c.powf(2.0) + d.powf(2.0);
        Complex {
            re: a*c/denominator,
            im: 0.0-(a*d)/denominator,
        }
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}+({}i)", self.re, self.im)
    }
}

impl Complex {
    #[inline]
    pub fn new(re: f64, im: f64) -> j64 {
        return Complex { re: re, im: im };
    }

    pub fn zero() -> j64 {
        return Complex { re: 0.0, im: 0.0 };
    }

    pub fn im() -> j64 {
        return Complex { re: 0.0, im: 1.0 };
    }
}
