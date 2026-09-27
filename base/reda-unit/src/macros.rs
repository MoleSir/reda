
#[macro_export]
macro_rules! num {
    ($val:literal k) => {
        $crate::Number::new($val as f64, $crate::Suffix::Kilo)
    };
    ($val:literal K) => {
        $crate::Number::new($val as f64, $crate::Suffix::Kilo)
    };
    ($val:literal M) => {
        $crate::Number::new($val as f64, $crate::Suffix::Mega)
    };
    ($val:literal G) => {
        $crate::Number::new($val as f64, $crate::Suffix::Giga)
    };
    ($val:literal m) => {
        $crate::Number::new($val as f64, $crate::Suffix::Milli)
    };
    ($val:literal u) => {
        $crate::Number::new($val as f64, $crate::Suffix::Micro)
    };
    ($val:literal n) => {
        $crate::Number::new($val as f64, $crate::Suffix::Nano)
    };
    ($val:literal p) => {
        $crate::Number::new($val as f64, $crate::Suffix::Pico)
    };
    ($val:literal) => {
        $crate::Number::new($val as f64, $crate::Suffix::None)
    };

    ($var:ident k) => {
        $crate::Number::new($var as f64, $crate::Suffix::Kilo)
    };
    ($var:ident K) => {
        $crate::Number::new($var as f64, $crate::Suffix::Kilo)
    };
    ($var:ident M) => {
        $crate::Number::new($var as f64, $crate::Suffix::Mega)
    };
    ($var:ident G) => {
        $crate::Number::new($var as f64, $crate::Suffix::Giga)
    };
    ($var:ident m) => {
        $crate::Number::new($var as f64, $crate::Suffix::Milli)
    };
    ($var:ident u) => {
        $crate::Number::new($var as f64, $crate::Suffix::Micro)
    };
    ($var:ident n) => {
        $crate::Number::new($var as f64, $crate::Suffix::Nano)
    };
    ($var:ident p) => {
        $crate::Number::new($var as f64, $crate::Suffix::Pico)
    };
    ($var:ident) => {
        $crate::Number::new($var as f64, $crate::Suffix::None)
    };
}

#[macro_export]
macro_rules! complex {
    ($re:literal $re_suffix:ident , $im:literal $im_suffix:ident) => {
        $crate::Complex::new($crate::num!($re $re_suffix), $crate::num!($im $im_suffix))
    };

    ($re:literal $re_suffix:ident , $im:literal ) => {
        $crate::Complex::new($crate::num!($re $re_suffix), $crate::num!($im))
    };

    ($re:literal , $im:literal $im_suffix:ident) => {
        $crate::Complex::new($crate::num!($re), $crate::num!($im $im_suffix))
    };

    ($re:literal , $im:literal ) => {
        $crate::Complex::new($crate::num!($re), $crate::num!($im))
    };
}

#[macro_export]
macro_rules! v {
    ($($t:tt)*) => {
        $crate::Voltage::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! i {
    ($($t:tt)*) => {
        $crate::Current::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! r {
    ($($t:tt)*) => {
        $crate::Resistance::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! c {
    ($($t:tt)*) => {
        $crate::Capacitance::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! l {
    ($($t:tt)*) => {
        $crate::Inductance::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! q {
    ($($t:tt)*) => {
        $crate::Charge::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! p {
    ($($t:tt)*) => {
        $crate::Power::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! e {
    ($($t:tt)*) => {
        $crate::Energy::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! t {
    ($($t:tt)*) => {
        $crate::Time::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! f {
    ($($t:tt)*) => {
        $crate::Frequency::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! m {
    ($($t:tt)*) => {
        $crate::Length::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! a {
    ($($t:tt)*) => {
        $crate::Area::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! fo {
    ($($t:tt)*) => {
        $crate::Force::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! pr {
    ($($t:tt)*) => {
        $crate::Pressure::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! mf {
    ($($t:tt)*) => {
        $crate::MagneticFlux::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! fd {
    ($($t:tt)*) => {
        $crate::FluxDensity::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! g {
    ($($t:tt)*) => {
        $crate::Conductance::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! vel {
    ($($t:tt)*) => {
        $crate::Velocity::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! acc {
    ($($t:tt)*) => {
        $crate::Accel::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! temp {
    ($($t:tt)*) => {
        $crate::Temperature::new($crate::num!($($t)*))
    };
}

#[macro_export]
macro_rules! ang {
    ($($t:tt)*) => {
        $crate::Angle::new($crate::num!($($t)*))
    };
}
