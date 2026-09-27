mod ast;
mod library;
mod cell;
mod pin;
mod expr;
mod unit;
pub use ast::*;
pub use library::*;
pub use cell::*;
pub use pin::*;
pub use expr::*;
pub use unit::*;

use std::{collections::HashMap, error::Error, str::FromStr};
use crate::error::{LibError, LibErrorContext};

pub trait Model {
    fn attrs(&self) -> &HashMap<String, LibValue>;
    fn groups(&self) -> &[LibGroup];
    fn attr(&self, name: &str) -> Option<&LibValue> {
        self.attrs().get(name)
    }
    fn attr_must(&self, name: &str) -> Result<&LibValue, LibError> {
        self.attr(name).ok_or_else(|| LibError::MissAttr(name.to_string()))
    }
    fn kind_groups(&self, kind: &str) -> impl IntoIterator<Item = &LibGroup> {
        self.groups().into_iter().filter(move |g| g.kind == kind)
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct LibGroup {
    pub kind: String,
    pub params: Vec<String>,

    pub attrs: HashMap<String, LibValue>,
    pub groups: Vec<LibGroup>,
}

impl LibGroup {
    pub fn new<S: Into<String>, P: Into<Vec<String>>>(kind: S, params: P) -> Self {
        Self {
            kind: kind.into(),
            params: params.into(),
            ..Default::default()
        }
    }
}

impl Model for LibGroup {
    fn attrs(&self) -> &HashMap<String, LibValue> {
        &self.attrs
    }

    fn groups(&self) -> &[LibGroup] {
        &self.groups
    }
}

impl LibGroup {
    pub fn expect_name(&mut self) -> Result<String, LibError> {
        if self.params.len() != 1 {
            Err(LibError::LibraryParamLenUnmatch)?
        }
        Ok(self.params[0].clone())
    }

    pub fn attr_bool_opt<S: Into<String>>(&self, attr_name: &str, ctx: impl Fn() -> S) -> Result<Option<bool>, LibError> {
        match self.attr(attr_name) {
            Some(v) => Ok(Some(v.bool().context(ctx)?)),
            None => Ok(None),
        }
    }

    pub fn attr_float_opt<S: Into<String>>(&self, attr_name: &str, ctx: impl Fn() -> S) -> Result<Option<f64>, LibError> {
        match self.attr(attr_name) {
            Some(v) => Ok(Some(v.float().context(ctx)?)),
            None => Ok(None),
        }
    }

    pub fn attr_string_opt<S: Into<String>>(&self, attr_name: &str, ctx: impl Fn() -> S) -> Result<Option<&str>, LibError> {
        match self.attr(attr_name) {
            Some(v) => Ok(Some(v.string().context(ctx)?)),
            None => Ok(None),
        }
    }

    pub fn attr_ident_opt<S: Into<String>>(&self, attr_name: &str, ctx: impl Fn() -> S) -> Result<Option<&str>, LibError> {
        match self.attr(attr_name) {
            Some(v) => Ok(Some(v.ident().context(ctx)?)),
            None => Ok(None),
        }
    }

    pub fn attr_literal_opt<S: Into<String>>(&self, attr_name: &str, ctx: impl Fn() -> S) -> Result<Option<&str>, LibError> {
        match self.attr(attr_name) {
            Some(v) => Ok(Some(v.literal().context(ctx)?)),
            None => Ok(None),
        }
    }

    pub fn attr_vector_opt<S, F>(&self, attr_name: &str, ctx: &F) -> Result<Option<&[LibValue]>, LibError>  
    where 
        S: Into<String>,
        F: Fn() -> S
    {
        match self.attr(attr_name) {
            Some(v) => Ok(Some(v.vector().context(ctx)?)),
            None => Ok(None),
        }
    }

    pub fn attr_expr_opt<S: Into<String>>(&self, attr_name: &str, ctx: &impl Fn() -> S) -> Result<Option<LibExpr>, LibError> {
        match self.attr(attr_name) {
            Some(v) => {
                let s = v.string().context(ctx)?;
                let expr = LibExpr::from_str(s).context(ctx)?;
                Ok(Some(expr))
            }
            None => Ok(None),
        }
    }

    pub fn attr_floats_arr_opt<const N: usize, S: Into<String>>(&self, attr_name: &str, ctx: &impl Fn() -> S) -> Result<Option<[f64; N]>, LibError> {
        match self.attr(attr_name) {
            Some(v) => {
                let vs = v.vector().context(ctx)?;
                if vs.len() != N {
                    Err(LibError::Parse(format!("expect float len {}, got {}", N, vs.len())))?;
                }
                let mut fs = [0.0; N];
                for i in 0..N {
                    let v = &vs[i];
                    let f = v.float().context(ctx)?;
                    fs[i] = f;
                }
                Ok(Some(fs))
            }
            None => Ok(None),
        }
    }

    pub fn attr_floats_opt<S: Into<String>>(&self, attr_name: &str, ctx: &impl Fn() -> S) -> Result<Option<Vec<f64>>, LibError> {
        match self.attr(attr_name) {
            Some(v) => {
                let vs = v.vector().context(ctx)?;
                if vs.len() != 1 {
                    Err(LibError::Parse(format!("expect one string element in {}", attr_name)))?;
                }
                let vs = &vs[0];
                let values = vs.floats().context(ctx)?;
                Ok(Some(values))
            }
            None => Ok(None),
        }
    }

    pub fn attr_parse_opt<T, S, F>(&self, attr_name: &str, ctx: &F) -> Result<Option<T>, LibError> 
    where 
        T: FromStr,
        T::Err: Error,
        S: Into<String>,
        F: Fn() -> S
    {
        match self.attr(attr_name) {
            Some(v) => {
                let s = v.ident().context(ctx)?;
                let v = T::from_str(s).map_err(|e| LibError::Parse(e.to_string())).context(ctx)?;
                Ok(Some(v))
            }
            None => Ok(None),
        }
    }

    pub fn attr_parse_string_opt<T, S, F>(&self, attr_name: &str, ctx: &F) -> Result<Option<T>, LibError> 
    where 
        T: FromStr,
        T::Err: Error,
        S: Into<String>,
        F: Fn() -> S
    {
        match self.attr(attr_name) {
            Some(v) => {
                let s = v.string().context(ctx)?;
                let v = T::from_str(s).map_err(|e| LibError::Parse(e.to_string())).context(ctx)?;
                Ok(Some(v))
            }
            None => Ok(None),
        }
    }

    pub fn attr_bool<S: Into<String>>(&self, attr_name: &str, ctx: impl Fn() -> S) -> Result<bool, LibError> {
        let v = self.attr_bool_opt(attr_name, ctx)?;
        v.ok_or_else(|| LibError::MissAttr(attr_name.into()))
    }

    pub fn attr_float<S: Into<String>>(&self, attr_name: &str, ctx: impl Fn() -> S) -> Result<f64, LibError> {
        let v = self.attr_float_opt(attr_name, ctx)?;
        v.ok_or_else(|| LibError::MissAttr(attr_name.into()))
    }

    pub fn attr_string<S: Into<String>>(&self, attr_name: &str, ctx: impl Fn() -> S) -> Result<&str, LibError> {
        let v = self.attr_string_opt(attr_name, ctx)?;
        v.ok_or_else(|| LibError::MissAttr(attr_name.into()))
    }

    pub fn attr_ident<S: Into<String>>(&self, attr_name: &str, ctx: impl Fn() -> S) -> Result<&str, LibError> {
        let v = self.attr_ident_opt(attr_name, ctx)?;
        v.ok_or_else(|| LibError::MissAttr(attr_name.into()))
    }

    pub fn attr_lliteral<S: Into<String>>(&self, attr_name: &str, ctx: impl Fn() -> S) -> Result<&str, LibError> {
        let v = self.attr_ident_opt(attr_name, ctx)?;
        v.ok_or_else(|| LibError::MissAttr(attr_name.into()))
    }

    pub fn attr_expr<S: Into<String>>(&self, attr_name: &str, ctx: &impl Fn() -> S) -> Result<LibExpr, LibError> {
        let v = self.attr_expr_opt(attr_name, ctx)?;
        v.ok_or_else(|| LibError::MissAttr(attr_name.into()))
    }

    pub fn attr_parse<T, S, F>(&self, attr_name: &str, ctx: &F) -> Result<T, LibError> 
    where 
        T: FromStr,
        T::Err: Error,
        S: Into<String>,
        F: Fn() -> S
    {
        let v = self.attr_parse_opt(attr_name, ctx)?;
        v.ok_or_else(|| LibError::MissAttr(attr_name.into()))
    }

    pub fn attr_parse_string<T, S, F>(&self, attr_name: &str, ctx: &F) -> Result<T, LibError> 
    where 
        T: FromStr,
        T::Err: Error,
        S: Into<String>,
        F: Fn() -> S
    {
        let v = self.attr_parse_string_opt(attr_name, ctx)?;
        v.ok_or_else(|| LibError::MissAttr(attr_name.into()))
    }

    pub fn attr_vector<S, F>(&self, attr_name: &str, ctx: &F) -> Result<&[LibValue], LibError>  
    where 
        S: Into<String>,
        F: Fn() -> S
    {
        let v = self.attr_vector_opt(attr_name, ctx)?;
        v.ok_or_else(|| LibError::MissAttr(attr_name.into())) 
    }
}
