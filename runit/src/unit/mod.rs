mod units;
mod ops;

use core::fmt;
use std::{fmt::Debug, marker::PhantomData, str::FromStr};
use crate::Number;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
pub use units::*;

pub trait Unit : PartialEq + Eq + Clone + Copy + Debug {
    fn name() -> &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitNumber<U> {
    number: Number,
    unit: PhantomData<U>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitComplex<U> {
    re: UnitNumber<U>,
    im: UnitNumber<U>,
    unit: PhantomData<U>,
}

impl<U: Unit> UnitNumber<U> {
    pub fn new<N: Into<Number>>(number: N) -> Self {
        Self { number: number.into(), unit: PhantomData }
    }

    pub fn to_f64(&self) -> f64 {
        self.number.to_f64()
    }

    pub fn value(&self) -> Number {
        self.number
    }

    pub fn is_nan(self) -> bool {
        self.number.is_nan()
    }

    pub fn is_finite(self) -> bool {
        self.number.is_finite()
    }

    pub fn powf(self, exp: f64) -> Self {
        Self::new(self.number.powf(exp))
    }

    pub fn atan2(self, other: Self) -> Number {
        self.number.atan2(other.number)
    }
}

macro_rules! impl_f64_like_method {
    ($f:ident) => {
        pub fn $f(&self) -> Self {
            Self::new(self.number.$f())
        }
    };
}

impl<U: Unit> UnitNumber<U> {
    impl_f64_like_method!(abs);
    impl_f64_like_method!(ceil);
    impl_f64_like_method!(floor);
    impl_f64_like_method!(round);
    impl_f64_like_method!(trunc);
    impl_f64_like_method!(fract);

    impl_f64_like_method!(sqrt);
    impl_f64_like_method!(exp);
    impl_f64_like_method!(ln);
    impl_f64_like_method!(log10);
    impl_f64_like_method!(log2);
    impl_f64_like_method!(recip);
    
    impl_f64_like_method!(sin);
    impl_f64_like_method!(cos);
    impl_f64_like_method!(tan);
    impl_f64_like_method!(asin);
    impl_f64_like_method!(acos);
    impl_f64_like_method!(atan);

    impl_f64_like_method!(sinh);
    impl_f64_like_method!(cosh);
    impl_f64_like_method!(tanh);

    impl_f64_like_method!(to_degrees);
    impl_f64_like_method!(to_radians);
}

impl<U: Unit> UnitComplex<U> {
    pub fn new<N1: Into<UnitNumber<U>>, N2: Into<UnitNumber<U>>>(re: N1, im: N2) -> Self {
        Self { re: re.into(), im: im.into(), unit: PhantomData }
    }

    pub fn parts(&self) -> (UnitNumber<U>, UnitNumber<U>) {
        (self.re, self.im)
    }

    pub fn conjugate(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
            unit: self.unit
        }
    }

    pub fn abs(&self) -> UnitNumber<U> {
        let abs_value = (self.re.number.powf(2.) + self.im.number.powf(2.)).sqrt();
        UnitNumber::new(abs_value)
    }

    pub fn arg(&self) -> Number {
        self.im.atan2(self.re)
    }
}

impl<U: Unit> fmt::Display for UnitNumber<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "{:.*}{}", p, self.number, U::name())
        } else {
            write!(f, "{}{}", self.number, U::name())
        }
    }
}

impl<U: Unit> fmt::Display for UnitComplex<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "{:.*} + {:.*}j", p, self.re, p, self.im)
        } else {
            write!(f, "{} + {}j", self.re, self.im)
        }
    }
}

impl<U: Unit> FromStr for UnitNumber<U> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.ends_with(U::name()) {
            let number_len = s.len() - U::name().len();
            let number_str = &s[..number_len];
            let number: Number = FromStr::from_str(number_str)?;
            Ok(Self::new(number))
        } else {
            Err(format!("Expect end with '{}'", U::name()))
        }
    }
}

impl<U: Unit> Serialize for UnitNumber<U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

impl<'de, U: Unit> Deserialize<'de> for UnitNumber<U> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl<U: Unit> From<Number> for UnitNumber<U> {
    fn from(value: Number) -> Self {
        Self::new(value)
    }
}

macro_rules! impl_from {
    ($t:ty) => {
        impl<U: Unit> From<$t> for UnitNumber<U> {
            fn from(value: $t) -> Self {
                Self::new(Number::from(value as f64))    
            }
        }
    };
}

impl_from!(f64);
impl_from!(f32);
impl_from!(u32);
impl_from!(i32);
