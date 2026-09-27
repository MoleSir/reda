use reda_unit::{Complex, Number, Unit, UnitComplex, UnitNumber};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Real(Number),
    Complex(Complex),
}

impl Value {
    pub fn real<V: Into<Number>>(v: V) -> Self {
        Self::Real(v.into())
    }

    pub fn complex<V1: Into<Number>, V2: Into<Number>>(re: V1, im: V2) -> Self {
        Self::Complex(Complex::new(re.into(), im.into()))
    }
}

impl Value {
    pub fn extract_numbers(values: &[Self]) -> Option<Vec<Number>> {
        let mut numbers = Vec::new(); 
        for v in values.iter() {
            match v {
                Value::Real(r) => numbers.push(*r),
                Value::Complex(_) => return None,
            }
        }
        Some(numbers)
    }

    pub fn extract_complexs(values: &[Self]) -> Option<Vec<Complex>> {
        let mut complexs = Vec::new(); 
        for v in values.iter() {
            match v {
                Value::Real(_) => return None,
                Value::Complex(c) => complexs.push(*c),
            }
        }
        Some(complexs)
    }

    pub fn extract_units<U: Unit>(values: &[Self]) -> Option<Vec<UnitNumber<U>>> {
        let mut numbers = Vec::new(); 
        for v in values.iter() {
            match v {
                Value::Real(r) => numbers.push(UnitNumber::new(*r)),
                Value::Complex(_) => return None,
            }
        }
        Some(numbers)
    }

    pub fn extract_unit_complexs<U: Unit>(values: &[Self]) -> Option<Vec<UnitComplex<U>>> {
        let mut numbers = Vec::new(); 
        for v in values.iter() {
            match v {
                Value::Real(_) => return None,
                Value::Complex(c) => numbers.push(UnitComplex::new(c.re, c.im)),
            }
        }
        Some(numbers)
    }
}