use crate::Number;
use std::{cmp::Ordering, ops::{Add, Div, Mul, Neg, Rem, Sub}};
use paste::paste;
use crate::unit::units::*;

use super::{Unit, UnitNumber};

/// output = lhs * rhs
#[macro_export]
macro_rules! impl_mul {
    ($output:ty, $lhs:ty, $rhs:ty) => {
        paste! {
            impl std::ops::Mul<crate::UnitNumber<[<$rhs Unit>]>> for crate::UnitNumber<[<$lhs Unit>]> {
                type Output = crate::UnitNumber<[<$output Unit>]>;
                fn mul(self, rhs: crate::UnitNumber<[<$rhs Unit>]>) -> Self::Output {
                    let result = self.number * rhs.number;
                    crate::UnitNumber::new(result)
                }
            }
        }
    };
}

/// output = lhs / rhs
#[macro_export]
macro_rules! impl_div {
    ($output:ty, $lhs:ty, $rhs:ty) => {
        paste! {
            impl std::ops::Div<crate::UnitNumber<[<$rhs Unit>]>> for crate::UnitNumber<[<$lhs Unit>]> {
                type Output = crate::UnitNumber<[<$output Unit>]>;
                fn div(self, rhs: crate::UnitNumber<[<$rhs Unit>]>) -> Self::Output {
                    let result = self.number / rhs.number;
                    crate::UnitNumber::new(result)
                }
            }
        }
    };
}

/// output = lhs * rhs
/// lhs = output / rhs
/// rhs = output / lhs
#[macro_export]
macro_rules! define_rule {
    ($output:ty, $lhs:ty, $rhs:ty) => {
        impl_mul!($output, $lhs, $rhs);
        impl_div!($lhs, $output, $rhs);
        impl_div!($rhs, $output, $lhs);
    };
}

define_rule!(Voltage, Resistance, Current);    // V = R × I
define_rule!(Power, Voltage, Current);         // P = V × I
define_rule!(Energy, Power, Time);             // E = P × t
define_rule!(Charge, Capacitance, Voltage);    // Q = C × V
define_rule!(Charge, Current, Time);           // Q = C × V
define_rule!(Current, Charge, Time);           // Q = I × t
define_rule!(Length, Velocity, Time);          // S = V × T
 
define_rule!(Power, Force, Velocity);          // P = F × v
define_rule!(Energy, Force, Length);           // E = F × d
define_rule!(Force, Pressure, Area);           // F = P × A

define_rule!(MagneticFlux, FluxDensity, Area); // Φ = B × A
define_rule!(MagneticFlux, Voltage, Time);     // Φ = V × t

impl_mul!(Area, Length, Length);
impl_div!(Length, Area, Length);

impl Frequency {
    pub fn to_period(&self) -> Time {
        Time::new(1. / self.number)
    }
}

impl Time {
    pub fn to_frquency(&self) -> Frequency {
        Frequency::new(1. / self.number)
    }
}

impl Mul<Time> for Frequency {
    type Output = Number;
    fn mul(self, rhs: Time) -> Self::Output {
        self.value() * rhs.value()
    }
}

impl Mul<Frequency> for Time {
    type Output = Number;
    fn mul(self, rhs: Frequency) -> Self::Output {
        self.value() * rhs.value()
    }
}

impl<U: Unit> Div<UnitNumber<U>> for UnitNumber<U> {
    type Output = Number;
    fn div(self, rhs: UnitNumber<U>) -> Self::Output {
        self.value() / rhs.value()
    }
}

// UnitNumber<U> = UnitNumber<U> + UnitNumber<U>
impl<U: Unit> Add<UnitNumber<U>> for UnitNumber<U> {
    type Output = UnitNumber<U>;
    fn add(self, rhs: UnitNumber<U>) -> Self::Output {
        let n = self.number + rhs.number;
        Self::Output::new(n)
    }
}

// UnitNumber<U> = UnitNumber<U> - UnitNumber<U>
impl<U: Unit> Sub<UnitNumber<U>> for UnitNumber<U> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let n = self.number - rhs.number;
        Self::Output::new(n)
    }
}

// UnitNumber<U> = - UnitNumber<U>
impl<U: Unit> Neg for UnitNumber<U> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        UnitNumber::new(self.number.neg())
    }
}

// UnitNumber<U> = UnitNumber<U> * Number
impl<U: Unit> Mul<Number> for UnitNumber<U> {
    type Output = Self;
    fn mul(self, rhs: Number) -> Self {
        let lhs_val = self.number.to_f64();
        let rhs_val = rhs.to_f64();
        Self::new(Number::from_f64(lhs_val * rhs_val))
    }
}

// UnitNumber<U> = Number * UnitNumber<U>
impl<U: Unit> Mul<UnitNumber<U>> for Number {
    type Output = UnitNumber<U>;
    fn mul(self, rhs: UnitNumber<U>) -> UnitNumber<U> {
        rhs * self
    }
}

// UnitNumber<U> = UnitNumber<U> * f64
impl<U: Unit> Mul<f64> for UnitNumber<U> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        let lhs_val = self.number.to_f64();
        Self::new(Number::from_f64(lhs_val * rhs))
    }
}

// UnitNumber<U> = Number * UnitNumber<U>
impl<U: Unit> Mul<UnitNumber<U>> for f64 {
    type Output = UnitNumber<U>;
    fn mul(self, rhs: UnitNumber<U>) -> UnitNumber<U> {
        rhs * self
    }
}

// UnitNumber<U> = UnitNumber<U> / Number
impl<U: Unit> Div<Number> for UnitNumber<U> {
    type Output = Self;
    fn div(self, rhs: Number) -> Self {
        let lhs_val = self.number.to_f64();
        let rhs_val = rhs.to_f64();
        Self::new(Number::from_f64(lhs_val / rhs_val))
    }
}

// UnitNumber<U> = UnitNumber<U> / f64
impl<U: Unit> Div<f64> for UnitNumber<U> {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        let lhs_val = self.number.to_f64();
        Self::new(Number::from_f64(lhs_val / rhs))
    }
}

impl<U: Unit> Rem<UnitNumber<U>> for UnitNumber<U> {
    type Output = Self;
    fn rem(self, rhs: UnitNumber<U>) -> Self::Output {
        Self::new(self.number % rhs.number)
    }
}

//==================== Cmp and Eq =========================//

impl<U: Unit> PartialOrd for UnitNumber<U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.number.partial_cmp(&other.number)
    }
}

impl<U: Unit> Ord for UnitNumber<U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

