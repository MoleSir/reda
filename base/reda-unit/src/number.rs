use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::str::FromStr;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suffix {
    Giga,  // 1e9
    Mega,  // 1e6
    Kilo,  // 1e3
    None,  // 1.0
    Milli, // 1e-3
    Micro, // 1e-6
    Nano,  // 1e-9
    Pico,  // 1e-12
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Number {
    pub value: f64,
    pub suffix: Suffix,
}

impl Suffix {
    pub const fn factor(&self) -> f64 {
        match self {
            Suffix::Giga => 1e9,
            Suffix::Mega => 1e6,
            Suffix::Kilo => 1e3,
            Suffix::None => 1.0,
            Suffix::Milli => 1e-3,
            Suffix::Micro => 1e-6,
            Suffix::Nano => 1e-9,
            Suffix::Pico => 1e-12,
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            Suffix::Giga => "G",
            Suffix::Mega => "M",
            Suffix::Kilo => "K",
            Suffix::None => "",
            Suffix::Milli => "m",
            Suffix::Micro => "u",
            Suffix::Nano => "n",
            Suffix::Pico => "p",
        }
    }
}

impl FromStr for Suffix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "G" => Ok(Suffix::Giga),
            "M" => Ok(Suffix::Mega),
            "K" => Ok(Suffix::Kilo),
            "" => Ok(Suffix::None),
            "m" => Ok(Suffix::Milli),
            "u" => Ok(Suffix::Micro),
            "n" => Ok(Suffix::Nano),
            "p" => Ok(Suffix::Pico),
            _ => Err(())
        }
    }
}

const PREFIX_VALUE_TABLE: [(Suffix, f64); 8] = [
    (Suffix::Giga, 1e9),
    (Suffix::Mega, 1e6),
    (Suffix::Kilo, 1e3),
    (Suffix::None, 1.0),
    (Suffix::Milli, 1e-3),
    (Suffix::Micro, 1e-6),
    (Suffix::Nano, 1e-9),
    (Suffix::Pico, 1e-12),
];

const PREFIX_TABLE: [(Suffix, &'static str); 8] = [
    (Suffix::Giga, "G"),
    (Suffix::Mega, "M"),
    (Suffix::Kilo, "K"),
    (Suffix::Kilo, "k"),
    (Suffix::Milli, "m"),
    (Suffix::Micro, "u"),
    (Suffix::Nano, "n"),
    (Suffix::Pico, "p"),
];

impl Number {
    pub const fn new(value: f64, suffix: Suffix) -> Self {
        Number { value, suffix }
    }

    pub const fn to_f64(&self) -> f64 {
        self.value * self.suffix.factor()
    }

    pub fn from_f64<F: Into<f64>>(val: F) -> Self {
        let val = val.into();
        let abs = val.abs();
        for (suffix, factor) in PREFIX_VALUE_TABLE.iter() {
            if abs >= *factor {
                return Number::new(val / factor, *suffix);
            }
        }

        Number::new(val, Suffix::None)
    }

    pub fn zero() -> Self {
        Self::new(0.0, Suffix::None)
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0.
    }

    pub fn is_nan(self) -> bool {
        self.value.is_nan()
    }

    pub fn is_finite(self) -> bool {
        self.value.is_finite()
    }

    pub fn powf(self, exp: f64) -> Self {
        Self::new(self.value.powf(exp), self.suffix)
    }

    pub fn atan2(self, other: Number) -> Self {
        Self::new(self.to_f64().atan2(other.to_f64()), self.suffix)
    }
}

macro_rules! impl_f64_like_method {
    ($f:ident) => {
        #[inline]
        pub fn $f(&self) -> Self {
            Self::new(self.value.$f(), self.suffix)
        }
    };
}

impl Number {
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

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "{:.*}{}", p, self.value, self.suffix.name())
        } else {
            write!(f, "{}{}", self.value, self.suffix.name())
        }
    }
}

impl FromStr for Number {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        for (suffix, suffix_str) in PREFIX_TABLE.iter() {
            if s.ends_with(suffix_str) {
                let num_str = &s[..s.len() - suffix_str.len()];
                let val: f64 = num_str.trim().parse()
                    .map_err(|e| format!("Parse number '{}' error for '{}'", num_str, e))?;
                return Ok(Number::new(val, *suffix));
            }
        }
        // As none suffix
        let val: f64 = s.parse().map_err(|e| format!("Parse number '{}' error for '{}'", s, e))?;
        return Ok(Number::new(val, Suffix::None));
    }
}


macro_rules! impl_from {
    ($t:ty) => {
        impl From<$t> for Number {
            fn from(value: $t) -> Self {
                Self {
                    value: value as f64,
                    suffix: Suffix::None
                }
            }
        }
    };
}

impl_from!(f64);
impl_from!(f32);
impl_from!(u32);
impl_from!(i32);

impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Number {
        Number::from_f64(self.to_f64() + rhs.to_f64())
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Number) -> Number {
        Number::from_f64(self.to_f64() - rhs.to_f64())
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Number) -> Number {
        Number::from_f64(self.to_f64() * rhs.to_f64())
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, rhs: Number) -> Number {
        Number::from_f64(self.to_f64() / rhs.to_f64())
    }
}

impl Add<f64> for Number {
    type Output = Number;
    fn add(self, rhs: f64) -> Number {
        Number::from_f64(self.to_f64() + rhs)
    }
}

impl Sub<f64> for Number {
    type Output = Number;
    fn sub(self, rhs: f64) -> Number {
        Number::from_f64(self.to_f64() - rhs)
    }
}

impl Mul<f64> for Number {
    type Output = Number;
    fn mul(self, rhs: f64) -> Number {
        Number::from_f64(self.to_f64() * rhs)
    }
}

impl Div<f64> for Number {
    type Output = Number;
    fn div(self, rhs: f64) -> Number {
        Number::from_f64(self.to_f64() / rhs)
    }
}

impl Add<Number> for f64 {
    type Output = Number;
    fn add(self, rhs: Number) -> Number {
        Number::from_f64(self + rhs.to_f64())
    }
}

impl Sub<Number> for f64 {
    type Output = Number;
    fn sub(self, rhs: Number) -> Number {
        Number::from_f64(self - rhs.to_f64())
    }
}

impl Mul<Number> for f64 {
    type Output = Number;
    fn mul(self, rhs: Number) -> Number {
        Number::from_f64(self * rhs.to_f64())
    }
}

impl Div<Number> for f64 {
    type Output = Number;
    fn div(self, rhs: Number) -> Number {
        Number::from_f64(self / rhs.to_f64())
    }
}

impl Rem<Number> for Number {
    type Output = Number;
    fn rem(self, rhs: Number) -> Self::Output {
        Number::from_f64(self.to_f64() % rhs.to_f64())
    }
}

impl Rem<f64> for Number {
    type Output = Number;
    fn rem(self, rhs: f64) -> Self::Output {
        Number::from_f64(self.to_f64() % rhs)
    }
}

impl Rem<Number> for f64 {
    type Output = Number;
    fn rem(self, rhs: Number) -> Self::Output {
        Number::from_f64(self % rhs.to_f64())
    }
}

impl Neg for Number {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            value: -self.value,
            suffix: self.suffix
        }
    }
}

impl PartialEq<f64> for Number {
    fn eq(&self, other: &f64) -> bool {
        self.to_f64() == *other
    }
}

impl PartialEq<Number> for f64 {
    fn eq(&self, other: &Number) -> bool {
        other == self
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_f64().partial_cmp(&other.to_f64())
    }
}

impl PartialOrd<f64> for Number {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.to_f64().partial_cmp(other)
    }
}

impl PartialOrd<Number> for f64 {
    fn partial_cmp(&self, other: &Number) -> Option<Ordering> {
        self.partial_cmp(&other.to_f64())
    }
}

impl Eq for Number {}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Number {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Number::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use crate::num;

    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_prefix_factor_and_suffix() {
        assert_eq!(Suffix::Giga.factor(), 1e9);
        assert_eq!(Suffix::Micro.name(), "u");
        assert_eq!(Suffix::from_str("K"), Ok(Suffix::Kilo));
        assert_eq!(Suffix::from_str("z"), Err(()));
    }

    #[test]
    fn test_number_new_and_to_f64() {
        let n = Number::new(3.3, Suffix::Kilo);
        assert_eq!(n.to_f64(), 3300.0);
    }

    #[test]
    fn test_number_from_f64() {
        let n = Number::from_f64(1e-6);
        assert_eq!(n.suffix, Suffix::Micro);
        assert!((n.value - 1.0).abs() < 1e-6);

        let n2 = Number::from_f64(1e3);
        assert_eq!(n2.suffix, Suffix::Kilo);
    }

    #[test]
    fn test_number_from_str() {
        let a = Number::from_str("3.3K").unwrap();
        assert_eq!(a.suffix, Suffix::Kilo);
        assert!((a.value - 3.3).abs() < 1e-6);

        let b = Number::from_str("2.2u").unwrap();
        assert_eq!(b.suffix, Suffix::Micro);
        assert!((b.value - 2.2).abs() < 1e-6);

        let c = Number::from_str("100").unwrap();
        assert_eq!(c.suffix, Suffix::None);
        assert_eq!(c.value, 100.0);

        assert!(Number::from_str("3.3X").is_err());
    }

    #[test]
    fn test_display() {
        let a = Number::new(1.23456, Suffix::Milli);
        let s = format!("{}", a);
        assert_eq!(s, "1.23456m");
    }

    #[test]
    fn test_number_arithmetic() {
        let a = Number::new(3.3, Suffix::Kilo); // 3300
        let b = Number::new(2.2, Suffix::Micro); // 2.2e-6

        let c = a + b;
        assert!((c.to_f64() - 3300.0000022).abs() < 1e-6);

        let d = a - b;
        assert!((d.to_f64() - 3299.9999978).abs() < 1e-6);

        let e = a * b;
        assert!((e.to_f64() - 3300.0 * 2.2e-6).abs() < 1e-9);

        let f = a / b;
        assert!((f.to_f64() - (3300.0 / 2.2e-6)).abs() < 1e-3);

        let g = num!(7.3) % num!(2.0);
        assert_eq!(g, 7.3 % 2.0);
    }

    #[test]
    fn test_number_f64_arithmetic() {
        let a = Number::new(3.3, Suffix::Kilo); // 3300
        let b = a + 1.0;
        assert!((b.to_f64() - 3301.0).abs() < 1e-6);
    }

    #[test]
    fn test_num_macro() {
        use crate::num;
        let a = num!(3.3 k);
        assert_eq!(a.suffix, Suffix::Kilo);
        assert_eq!(a.value, 3.3);

        let b = num!(1.0 u);
        assert_eq!(b.suffix, Suffix::Micro);
        assert_eq!(b.value, 1.0);

        let c = num!(100);
        assert_eq!(c.suffix, Suffix::None);
        assert_eq!(c.value, 100.0);

        let c = 100;
        let c = num!(c);
        assert_eq!(c.value, 100.0);
    }

    use serde_json;

    #[test]
    fn test_serialize_number() {
        let n = Number::new(1.5, Suffix::Milli);
        let json = serde_json::to_string(&n).unwrap();
        assert_eq!(json, "\"1.5m\"");
    }

    #[test]
    fn test_deserialize_number() {
        let json = "\"2.2u\"";
        let n: Number = serde_json::from_str(json).unwrap();
        assert_eq!(n, Number::new(2.2, Suffix::Micro));
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let original = Number::new(3.3, Suffix::Nano);
        let json = serde_json::to_string(&original).unwrap();
        let parsed: Number = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_deserialize_invalid_number() {
        let json = "\"bad_number\"";
        let result: Result<Number, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_number_no_suffix() {
        let json = "\"42.0\"";
        let n: Number = serde_json::from_str(json).unwrap();
        assert_eq!(n, Number::new(42.0, Suffix::None));
    }
}