use std::collections::HashMap;
use reda_unit::Number;

use crate::ToSpice;

#[derive(Debug, Clone)]
pub struct Model {
    pub name: String, 
    pub kind: ModelKind, 
    pub parameters: HashMap<String, Number>,
}

impl Model {
    pub fn diode<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            kind: ModelKind::Diode,
            parameters: Default::default(),
        }
    }

    pub fn npn<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            kind: ModelKind::NPN,
            parameters: Default::default(),
        }
    }

    pub fn pnp<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            kind: ModelKind::PNP,
            parameters: Default::default(),
        }
    }

    pub fn pmos<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            kind: ModelKind::PMos,
            parameters: Default::default(),
        }
    }

    pub fn nmos<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            kind: ModelKind::NMos,
            parameters: Default::default(),
        }
    }

    pub fn parameter<K: Into<String>, V: Into<Number>>(&mut self, key: K, val: V) {
        self.parameters.insert(key.into(), val.into());
    }
}

impl ToSpice for Model {
    fn to_spice(&self) -> String {
        let mut s = format!(".MODEL {} {} (", self.name, self.kind.to_str());
        for (key, val) in self.parameters.iter() {
            s.push_str(&key);
            s.push('=');
            s.push_str(&val.to_spice());
        }
        s.push(')');
        s
    }
}

#[derive(Debug, Clone, )]
pub enum ModelKind {
    Diode,
    NPN,
    PNP,
    PMos,
    NMos,
}

impl ModelKind {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Diode => "D",
            Self::NPN => "NPN",
            Self::PNP => "PNP",
            Self::NMos => "NMOS",
            Self::PMos => "PMOS",
        }
    }
}