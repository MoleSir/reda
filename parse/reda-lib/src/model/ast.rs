use std::{error::Error, fmt::Display, str::FromStr};
use crate::error::{LibError, LibErrorContext};
use super::LibExpr;

#[derive(Debug, Clone, PartialEq)]
pub enum LibAst {
    LibGroup {
        kind: String,
        params: Vec<String>,
        items: Vec<LibAst>,
    },
    Attr {
        key: String,
        value: LibValue,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LibValue {
    Bool(bool),
    Float(f64),
    String(String),
    Ident(String),
    Vec(Vec<LibValue>),
}

impl LibAst {
    pub fn group(kind: impl Into<String>, params: impl Into<Vec<String>>, items: impl Into<Vec<LibAst>>) -> Self {
        Self::LibGroup {
            kind: kind.into(),
            params: params.into(),
            items: items.into(),
        }
    }

    pub fn group_single_param(kind: impl Into<String>, param: impl Into<String>, items: impl Into<Vec<LibAst>>) -> Self {
        Self::LibGroup {
            kind: kind.into(),
            params: vec![param.into()],
            items: items.into(),
        }
    }

    pub fn attr(key: impl Into<String>, value: impl Into<LibValue>) -> Self {
        Self::Attr { key: key.into(), value: value.into() }
    }
}

impl LibValue {
    pub fn kind_str(&self) -> &'static str {
        match self {
            Self::Bool(_) => "bool",
            Self::Float(_) => "float",
            Self::Ident(_) => "ident",
            Self::String(_) => "string",
            Self::Vec(_) => "vector",
        }
    }

    pub fn bool(&self) -> Result<bool, LibError> {
        if let Self::Bool(value) = self {
            Ok(*value)
        } else {
            Err(LibError::ValueKindUnmatch("bool", self.kind_str()))
        }
    }

    pub fn float(&self) -> Result<f64, LibError> {
        if let Self::Float(value) = self {
            Ok(*value)
        } else {
            Err(LibError::ValueKindUnmatch("float", self.kind_str()))
        }
    }

    pub fn floats(&self) -> Result<Vec<f64>, LibError> {
        if let Self::String(value) = self {
            let values = value
                .split(',')
                .map(|token| token.trim().parse::<f64>().map_err(|e| LibError::ParseFloat(e)))
                .collect::<Result<Vec<_>, _>>()
                .context(|| "parse float from strings")?;
            Ok(values)
        } else {
            Err(LibError::ValueKindUnmatch("floats", self.kind_str()))
        }
    }

    pub fn expr(&self) -> Result<LibExpr, LibError> {
        if let Self::String(value) = self {
            let expr = LibExpr::from_str(value)?;
            Ok(expr)
        } else {
            Err(LibError::ValueKindUnmatch("expr", self.kind_str()))
        }
    }

    pub fn ident(&self) -> Result<&str, LibError> {
        if let Self::Ident(value) = self {
            Ok(value)
        } else {
            Err(LibError::ValueKindUnmatch("ident", self.kind_str()))
        }
    }

    pub fn literal(&self) -> Result<&str, LibError> {
        match self {
            Self::Ident(value) => Ok(value),
            Self::String(value) => Ok(value),
            _ => Err(LibError::ValueKindUnmatch("literal", self.kind_str()))
        }
    }

    pub fn string(&self) -> Result<&str, LibError> {
        if let Self::String(value) = self {
            Ok(value)
        } else {
            Err(LibError::ValueKindUnmatch("string", self.kind_str()))
        }
    }

    pub fn vector(&self) -> Result<&[LibValue], LibError> {
        if let Self::Vec(value) = self {
            Ok(value)
        } else {
            Err(LibError::ValueKindUnmatch("vector", self.kind_str()))
        }
    }

    pub fn parse<T>(&self) -> Result<T, LibError> 
    where 
        T: FromStr,
        T::Err: Error,
    {
        let s = self.ident()?;
        T::from_str(s).map_err(|e| LibError::Parse(e.to_string()))
    }
}

impl LibValue {
    pub fn into_bool(self) -> Result<bool, LibError> {
        if let Self::Bool(value) = self {
            Ok(value)
        } else {
            Err(LibError::ValueKindUnmatch("bool", self.kind_str()))
        }
    }

    pub fn into_float(self) -> Result<f64, LibError> {
        if let Self::Float(value) = self {
            Ok(value)
        } else {
            Err(LibError::ValueKindUnmatch("float", self.kind_str()))
        }
    }

    pub fn into_floats(self) -> Result<Vec<f64>, LibError> {
        self.floats()
    }

    pub fn into_ident(self) -> Result<String, LibError> {
        if let Self::Ident(value) = self {
            Ok(value)
        } else {
            Err(LibError::ValueKindUnmatch("ident", self.kind_str()))
        }
    }

    pub fn into_string(self) -> Result<String, LibError> {
        if let Self::String(value) = self {
            Ok(value)
        } else {
            Err(LibError::ValueKindUnmatch("string", self.kind_str()))
        }
    }

    pub fn into_literal(self) -> Result<String, LibError> {
        match self {
            Self::Ident(value) => Ok(value),
            Self::String(value) => Ok(value),
            _ => Err(LibError::ValueKindUnmatch("literal", self.kind_str()))
        }
    }

    pub fn into_vector(self) -> Result<Vec<LibValue>, LibError> {
        if let Self::Vec(value) = self {
            Ok(value)
        } else {
            Err(LibError::ValueKindUnmatch("vector", self.kind_str()))
        }
    }
}

impl LibValue {
    pub fn is_complex(&self) -> bool {
        if let Self::Vec(_) = self {
            true
        } else {
            false
        }
    }
}

impl Display for LibValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(b) => if *b { write!(f, "ture") } else { write!(f, "false") },
            Self::Float(v) => write!(f, "{}", *v),
            Self::Ident(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Vec(values) => {
                write!(f, "(")?;
                for (i, v) in values.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, ")")
            }
        }
    }
}