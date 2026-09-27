/// https://github.com/zao111222333/liberty-db/blob/main/src/units.rs

use core::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::EnumString, strum::Display, strum::IntoStaticStr, strum::EnumIter)]
pub enum LibTimeUnit {
  /// 1ps, 1e-12
  #[strum(serialize = "1ps")]
  _1ps,
  /// 10ps, 1e-11
  #[strum(serialize = "10ps")]
  _10ps,
  /// 100ps, 1e-10
  #[strum(serialize = "100ps")]
  _100ps,
  /// 1ns, 1e-9
  #[default]
  #[strum(serialize = "1ns")]
  _1ns,
}

impl LibTimeUnit {
  #[inline]
  #[must_use]
  pub const fn value(&self) -> f64 {
    match self {
      Self::_1ps => 1E-12_f64,
      Self::_10ps => 1E-11_f64,
      Self::_100ps => 1E-10_f64,
      Self::_1ns => 1E-9_f64,
    }
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::EnumString, strum::Display, strum::IntoStaticStr, strum::EnumIter)]
pub enum LibVoltageUnit {
  /// 1mV, 1e-3
  #[strum(serialize = "1mV")]
  _1mV,
  /// 10mV, 1e-2
  #[strum(serialize = "10mV")]
  _10mV,
  /// 100mV, 1e-1
  #[strum(serialize = "100mV")]
  _100mV,
  /// 1V, 1e0
  #[default]
  #[strum(serialize = "1V")]
  _1V,
}

impl LibVoltageUnit {
  #[inline]
  #[must_use]
  pub const fn value(&self) -> f64 {
    match self {
      Self::_1mV => 1E-3_f64,
      Self::_10mV => 1E-2_f64,
      Self::_100mV => 1E-1_f64,
      Self::_1V => 1E0_f64,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::EnumString, strum::Display, strum::IntoStaticStr, strum::EnumIter)]
pub enum LibCurrentUnit {
  /// 1uA, 1e-6
  #[strum(serialize = "1uA")]
  _1uA,
  /// 10uA, 1e-5
  #[strum(serialize = "10uA")]
  _10uA,
  /// 100uA, 1e-4
  #[strum(serialize = "100uA")]
  _100uA,
  /// 1mA, 1e-3
  #[strum(serialize = "1mA")]
  _1mA,
  /// 10mA, 1e-2
  #[strum(serialize = "10mA")]
  _10mA,
  /// 100mA, 1e-1
  #[strum(serialize = "100mA")]
  _100mA,
  /// 1A, 1e0
  #[strum(serialize = "1A")]
  _1A,
}

impl LibCurrentUnit {
  #[inline]
  #[must_use]
  pub const fn value(&self) -> f64 {
    match self {
      Self::_1uA => 1E-6_f64,
      Self::_10uA => 1E-5_f64,
      Self::_100uA => 1E-4_f64,
      Self::_1mA => 1E-3_f64,
      Self::_10mA => 1E-2_f64,
      Self::_100mA => 1E-1_f64,
      Self::_1A => 1E0_f64,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::EnumString, strum::Display, strum::IntoStaticStr, strum::EnumIter)]
pub enum LibPullingResistanceUnit {
  /// 1ohm, 1
  #[strum(serialize = "1ohm")]
  _1ohm,
  /// 10ohm, 10
  #[strum(serialize = "10ohm")]
  _10ohm,
  /// 100ohm, 100
  #[strum(serialize = "100ohm")]
  _100ohm,
  /// 1kohm, 1000
  #[strum(serialize = "1kohm")]
  _1kohm,
}

impl LibPullingResistanceUnit {
  #[inline]
  #[must_use]
  pub const fn value(&self) -> f64 {
    match self {
      Self::_1ohm => 1E0_f64,
      Self::_10ohm => 1E1_f64,
      Self::_100ohm => 1E2_f64,
      Self::_1kohm => 1E3_f64,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum LibCapacitiveLoadUnit {
  FF(f64),
  PF(f64),
}

impl Default for LibCapacitiveLoadUnit {
  #[inline]
  fn default() -> Self {
    Self::_1pf
  }
}

impl fmt::Display for LibCapacitiveLoadUnit {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::FF(val) => write!(f, "{val} ff"),
      Self::PF(val) => write!(f, "{val} pf"),
    }
  }
}

impl LibCapacitiveLoadUnit {
  #[expect(non_upper_case_globals)]
  pub const _1pf: Self = Self::PF(1.0);
  #[expect(non_upper_case_globals)]
  pub const _1ff: Self = Self::FF(1.0);
  #[inline]
  #[must_use]
  #[expect(clippy::float_arithmetic)]
  pub fn value(&self) -> f64 {
    match self {
      Self::FF(val) => val * 1e-15,
      Self::PF(val) => val * 1e-12,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::EnumString, strum::Display, strum::IntoStaticStr, strum::EnumIter)]
pub enum LibLeakagePowerUnit {
  /// 1pW, 1e-12
  #[strum(serialize = "1pW")]
  _1pW,
  /// 10pW, 1e-11
  #[strum(serialize = "10pW")]
  _10pW,
  /// 100pW, 1e-10
  #[strum(serialize = "100pW")]
  _100pW,
  /// 1nW, 1e-9
  #[strum(serialize = "1nW")]
  _1nW,
  /// 10nW, 1e-8
  #[strum(serialize = "10nW")]
  _10nW,
  /// 100nW, 1e-7
  #[strum(serialize = "100nW")]
  _100nW,
  /// 1uW, 1e-6
  #[strum(serialize = "1uW")]
  _1uW,
  /// 10uW, 1e-5
  #[strum(serialize = "10uW")]
  _10uW,
  /// 100uW, 1e-4
  #[strum(serialize = "100uW")]
  _100uW,
  /// 1mW, 1e-3
  #[strum(serialize = "1mW")]
  _1mW,
  /// 10mW, 1e-2
  #[strum(serialize = "10mW")]
  _10mW,
  /// 100mW, 1e-1
  #[strum(serialize = "100mW")]
  _100mW,
  /// 1W, 1e0
  #[strum(serialize = "1W")]
  _1W,
}

impl LibLeakagePowerUnit {
  #[inline]
  #[must_use]
  pub const fn value(&self) -> f64 {
    match self {
      Self::_1pW => 1E-12_f64,
      Self::_10pW => 1E-11_f64,
      Self::_100pW => 1E-10_f64,
      Self::_1nW => 1E-9_f64,
      Self::_10nW => 1E-8_f64,
      Self::_100nW => 1E-7_f64,
      Self::_1uW => 1E-6_f64,
      Self::_10uW => 1E-5_f64,
      Self::_100uW => 1E-4_f64,
      Self::_1mW => 1E-3_f64,
      Self::_10mW => 1E-2_f64,
      Self::_100mW => 1E-1_f64,
      Self::_1W => 1E-0_f64,
    }
  }
}