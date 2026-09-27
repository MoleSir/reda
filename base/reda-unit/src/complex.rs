use core::fmt;
use std::{ops::{Add, Div, Mul, Sub}, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Number;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Complex {
    pub re: Number,
    pub im: Number,
}

impl Complex {
    pub fn new<N1: Into<Number>, N2: Into<Number>>(re: N1, im: N2) -> Self {
        Self { re: re.into(), im: im.into() }
    }

    pub fn parts(&self) -> (Number, Number) {
        (self.re, self.im)
    }

    pub fn conjugate(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn norm_sqr(&self) -> Number {
        self.re * self.re + self.im * self.im
    }

    pub fn abs(&self) -> Number {
        (self.re.powf(2.) + self.im.powf(2.)).sqrt()
    }

    pub fn arg(&self) -> Number {
        self.im.atan2(self.re)
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let (a, b) = self.parts();
        let (c, d) = rhs.parts();
        Complex::new(a + c, b + d)
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let (a, b) = self.parts();
        let (c, d) = rhs.parts();
        Complex::new(a - c, b - d)
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let (a, b) = self.parts();
        let (c, d) = rhs.parts();
        // (a + bj) * (c + dj) = (ac - bd) + (ad + bc)j
        Complex::new(a * c - b * d, a * d + b * c)
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let (a, b) = self.parts();
        let (c, d) = rhs.parts();
        let denom = c * c + d * d;
        if denom.is_zero() {
            panic!("Divide by zero in complex division");
        }
        let re = (a * c + b * d) / denom;
        let im = (b * c - a * d) / denom;
        Complex::new(re, im)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let re_is_zero = self.re.to_f64() == 0.0;
        let im_is_zero = self.im.to_f64() == 0.0;

        let precision = f.precision();

        match (re_is_zero, im_is_zero) {
            (true, true) => write!(f, "0"),
            (false, true) => {
                match precision {
                    Some(p) => write!(f, "{:.*}", p, self.re),
                    None => write!(f, "{}", self.re),
                }
            }
            (true, false) => {
                if self.im.to_f64() >= 0.0 {
                    match precision {
                        Some(p) => write!(f, "{:.*}j", p, self.im),
                        None => write!(f, "{}j", self.im),
                    }
                } else {
                    match precision {
                        Some(p) => write!(f, "-{:.*}j", p, -self.im),
                        None => write!(f, "-{}j", -self.im),
                    }
                }
            }
            (false, false) => {
                let re_fmt = match precision {
                    Some(p) => format!("{:.*}", p, self.re),
                    None => format!("{}", self.re),
                };
                let im_fmt = match precision {
                    Some(p) => format!("{:.*}", p, self.im),
                    None => format!("{}", self.im),
                };

                if self.im.to_f64() >= 0.0 {
                    write!(f, "{}+{}j", re_fmt, im_fmt)
                } else {
                    write!(f, "{}-{}j", re_fmt, im_fmt.trim_start_matches('-'))
                }
            }
        }
    }
}

impl FromStr for Complex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn find_real_imag_separator(s: &str) -> Option<usize> {
            let mut chars = s.char_indices().peekable();
            chars.next(); 

            while let Some((i, c)) = chars.next() {
                if (c == '+' || c == '-') && s[i+1..].contains('j') {
                    return Some(i);
                }
            }
            None
        }
        
        let s = s.trim();
        
        // '+'  '-' 
        if let Some(idx) = find_real_imag_separator(s) {
            let (real_part, imag_part) = s.split_at(idx);
            let real = real_part.trim().parse::<Number>()
                .map_err(|e| format!("Parse real part error: {}", e))?;
            let imag_str = imag_part.trim_end_matches('j');
            let imag = imag_str.parse::<Number>()
                .map_err(|e| format!("Parse imaginary part error: {}", e))?;
            return Ok(Complex { re: real, im: imag });
        }
        
        // 1.2j、3uj
        if s.ends_with('j') {
            let imag_part = &s[..s.len()-1];
            let im = imag_part.parse::<Number>()
                .map_err(|e| format!("Parse imaginary part error: {}", e))?;
            return Ok(Complex { re: Number::zero(), im });
        }

        let re = s.parse::<Number>()
            .map_err(|e| format!("Parse real number error: {}", e))?;
        Ok(Complex { re, im: Number::zero() })
    }
}

impl Serialize for Complex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Complex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Complex::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use crate::{complex, num, Suffix};

    use super::*;

    #[test]
    fn test_real_only() {
        let c = Complex::from_str("1.5").unwrap();
        assert_eq!(c.re, Number::new(1.5, Suffix::None));
        assert_eq!(c.im, Number::new(0.0, Suffix::None));

        let c = Complex::from_str("2.2u").unwrap();
        assert_eq!(c.re, Number::new(2.2, Suffix::Micro));
        assert_eq!(c.im, Number::new(0.0, Suffix::None));
    }

    #[test]
    fn test_imag_only() {
        let c = Complex::from_str("+3.3j").unwrap();
        assert_eq!(c.re, Number::new(0.0, Suffix::None));
        assert_eq!(c.im, Number::new(3.3, Suffix::None));

        let c = Complex::from_str("-5.5mj").unwrap();
        assert_eq!(c.re, Number::new(0.0, Suffix::None));
        assert_eq!(c.im, Number::new(-5.5, Suffix::Milli));

        let c = Complex::from_str("5.5mj").unwrap();
        assert_eq!(c.re, Number::new(0.0, Suffix::None));
        assert_eq!(c.im, Number::new(5.5, Suffix::Milli));
    }

    #[test]
    fn test_real_imag() {
        let c = Complex::from_str("1.1+2.2j").unwrap();
        assert_eq!(c.re, Number::new(1.1, Suffix::None));
        assert_eq!(c.im, Number::new(2.2, Suffix::None));

        let c = Complex::from_str("-3.0-4.4uj").unwrap();
        assert_eq!(c.re, Number::new(-3.0, Suffix::None));
        assert_eq!(c.im, Number::new(-4.4, Suffix::Micro));

        let c = Complex::from_str("10.5-7.5nj").unwrap();
        assert_eq!(c.re, Number::new(10.5, Suffix::None));
        assert_eq!(c.im, Number::new(-7.5, Suffix::Nano));
    }

    #[test]
    fn test_error_cases() {
        assert!(Complex::from_str("hello").is_err());
        assert!(Complex::from_str("1.2+badj").is_err());
        assert!(Complex::from_str("1.2+3.3").is_err());
        assert!(Complex::from_str("j3.3").is_err());
    }

    #[test]
    fn test_creation() {
        let c = Complex { re: num!(3.0), im: num!(4.0) };
        let _ = complex!(3.0, 4.0);
        assert_eq!(c.re, num!(3.0));
        assert_eq!(c.im, num!(4.0));
    }

    #[test]
    fn test_equality() {
        let a = Complex { re: num!(1.0), im: num!(2.0) };
        let b = Complex { re: num!(1.0), im: num!(2.0) };
        let c = Complex { re: num!(1.0), im: num!(3.0) };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_addition() {
        let a = Complex { re: num!(1.0), im: num!(2.0) };
        let b = Complex { re: num!(3.0), im: num!(4.0) };
        let sum = Complex { re: num!(4.0), im: num!(6.0) };
        assert_eq!(a + b, sum);
    }

    #[test]
    fn test_multiplication() {
        let a = Complex { re: num!(1.0), im: num!(2.0) };
        let b = Complex { re: num!(3.0), im: num!(4.0) };
        // (1 + 2i)(3 + 4i) = (3 - 8) + (4 + 6)i = -5 + 10i
        let product = Complex { re: num!(-5.0), im: num!(10.0) };
        assert_eq!(a * b, product);
    }

    #[test]
    fn test_conjugate() {
        let a = Complex { re: num!(5.0), im: num!(-7.0) };
        let conj = Complex { re: num!(5.0), im: num!(7.0) };
        assert_eq!(a.conjugate(), conj);
    }

    #[test]
    fn test_magnitude_squared() {
        let c = Complex { re: num!(3.0), im: num!(4.0) };
        assert_eq!(c.norm_sqr(), num!(25.0));
    }

    #[test]
    fn test_serialize_deserialize_complex_real_only() {
        let c = Complex::from_str("3.3u").unwrap();
        let json = serde_json::to_string(&c).unwrap();
        assert_eq!(json, "\"3.3u\"");
        let parsed: Complex = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, c);
    }

    #[test]
    fn test_serialize_deserialize_complex_imag_only() {
        let c = Complex::from_str("2.2mj").unwrap();
        let json = serde_json::to_string(&c).unwrap();
        assert_eq!(json, "\"2.2mj\"");
        let parsed: Complex = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, c);
    }

    #[test]
    fn test_serialize_deserialize_complex_full() {
        let c = Complex::from_str("1.5+2.5uj").unwrap();
        let json = serde_json::to_string(&c).unwrap();
        assert_eq!(json, "\"1.5+2.5uJ\"".replace("J", "j")); // 复数内部序列化成小写 j
        let parsed: Complex = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, c);
    }
}