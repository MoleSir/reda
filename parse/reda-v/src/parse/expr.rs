use nom::bytes::complete::{tag, take_while1};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::{branch::alt, character::complete::char};
use nom::combinator::{map, opt};
use crate::model::{BinOpKind, ConstExpr, Expr, Range, UnaryOpKind, Value, ValueBase, VarExpr};

use super::base::{hws, identifier, integer, unsigned_int, NomResult};

macro_rules! ident_or_range_impl {
    ($t:tt) => {
        fn ident_or_range(input: &str) -> NomResult<$t> {
            let (input, name) = identifier(input)?;
        
            // 尝试解析 [ ... ]
            let (input, opt_range) = opt(delimited(
                char('['),
                alt((
                    // 范围选择: msb:lsb
                    map(
                        separated_pair(integer, char(':'), integer),
                        |(msb, lsb): (i32, i32)| Range { msb: msb as i32, lsb: lsb as i32 },
                    ),
                    // 单比特选择: [n]
                    map(integer, |n: i32| Range { msb: n as i32, lsb: n as i32 }),
                )),
                char(']'),
            ))(input)?;
        
            if let Some(range) = opt_range {
                Ok((input, $t::RangeSelect { base: name.to_string(), range }))
            } else {
                Ok((input, $t::Ident(name.to_string())))
            }
        }
    };
}

pub fn expr(input: &str) -> NomResult<Expr> {
    binop(input, 0)
}

pub fn var_expr(input: &str) -> NomResult<VarExpr> {
    ident_or_range_impl!(VarExpr);
    hws(ident_or_range)(input)
}

pub fn const_expr(input: &str) -> NomResult<ConstExpr> {
    hws(alt((
        map(integer, |n| ConstExpr::Number(n as i32)),
        map(value, |v| ConstExpr::Value(v)),
    )))(input)
}

pub fn atom(input: &str) -> NomResult<Expr> {
    ident_or_range_impl!(Expr);
    hws(alt((
        ident_or_range,
        map(integer, |n| Expr::Number(n as i32)),
        map(value, |v| Expr::Value(v)),
        delimited(char('('), hws(expr), char(')')),
    )))(input)
}

pub fn unary(input: &str) -> NomResult<Expr> {
    hws(alt((
        map(preceded(char('!'), unary), |e| {
            Expr::UnaryOp {
                op: UnaryOpKind::Not,
                expr: Box::new(e),
            }
        }),
        map(preceded(char('-'), unary), |e| {
            Expr::UnaryOp {
                op: UnaryOpKind::Neg,
                expr: Box::new(e),
            }
        }),
        atom,
    )))(input)
}

fn precedence(op: &BinOpKind) -> u8 {
    match op {
        BinOpKind::Mul => 5,
        BinOpKind::Add | BinOpKind::Sub => 4,
        BinOpKind::Lt | BinOpKind::Gt => 3,
        BinOpKind::Eq | BinOpKind::Neq => 2,
        BinOpKind::And | BinOpKind::Or => 1,
    }
}

fn op(input: &str) -> NomResult<BinOpKind> {
    hws(alt((
        map(tag("=="), |_| BinOpKind::Eq),
        map(tag("!="), |_| BinOpKind::Neq),
        map(tag("+"), |_| BinOpKind::Add),
        map(tag("-"), |_| BinOpKind::Sub),
        map(tag("*"), |_| BinOpKind::Mul),
        map(tag("&"), |_| BinOpKind::And),
        map(tag("|"), |_| BinOpKind::Or),
        map(tag("<"), |_| BinOpKind::Lt),
        map(tag(">"), |_| BinOpKind::Gt),
    )))(input)
}

fn binop(input: &str, min_prec: u8) -> NomResult<Expr> {
    let (mut input, mut lhs) = unary(input)?;
    loop {
        let (next_input, op) = match op(input) {
            Ok(res) => res,
            Err(_) => break,
        };

        let prec = precedence(&op);
        if prec < min_prec {
            break;
        }

        let next_min_prec = prec + 1;
        let (after_rhs, rhs) = binop(next_input, next_min_prec)?;
        
        lhs = Expr::BinOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };

        input = after_rhs;
    }

    Ok((input, lhs))
}

pub fn value(input: &str) -> NomResult<Value> {
    let (input, len) = hws(unsigned_int)(input)?;
    let (input, _) = char('\'')(input)?;
    let (input, kind) = alt((
        map(char('b'), |_| ValueBase::Bin),
        map(char('d'), |_| ValueBase::Dec),
        map(char('o'), |_| ValueBase::Oct),
        map(char('h'), |_| ValueBase::Hex),
    ))(input)?;

    let (input, bits) = match kind {
        ValueBase::Bin => bin_value_bits(input)?,
        ValueBase::Dec => dec_value_bits(input)?,
        ValueBase::Oct => oct_value_bits(input)?,
        ValueBase::Hex => hex_value_bits(input)?,
    };

    Ok((
        input,
        Value {
            len: len as usize,
            base: kind,
            bits,
        },
    ))
}

pub fn bin_value_bits(input: &str) -> NomResult<String> {
    map(
        take_while1(|c: char| matches!(c, '0' | '1' | 'x' | 'X' | 'z' | 'Z' | '_')),
        |s: &str| s.to_string(),
    )(input)
}

pub fn dec_value_bits(input: &str) -> NomResult<String> {
    map(
        take_while1(|c: char| matches!(c, '0'..='9' | '_')),
        |s: &str| s.to_string(),
    )(input)
}

pub fn oct_value_bits(input: &str) -> NomResult<String> {
    map(
        take_while1(|c: char| matches!(c, '0'..='7' | 'x' | 'X' | 'z' | 'Z' | '_')),
        |s: &str| s.to_string(),
    )(input)
}

pub fn hex_value_bits(input: &str) -> NomResult<String> {
    map(
        take_while1(|c: char| c.is_ascii_hexdigit() || matches!(c, 'x' | 'X' | 'z' | 'Z' | '_')),
        |s: &str| s.to_string(),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_and_ident() {
        assert_eq!(expr("123").unwrap().1, Expr::Number(123));
        assert_eq!(expr("foo").unwrap().1, Expr::Ident("foo".to_string()));
    }

    #[test]
    fn test_simple_binop() {
        let (_, e) = expr("1+2").unwrap();
        match e {
            Expr::BinOp { op, .. } => assert_eq!(op, BinOpKind::Add),
            _ => panic!("expected BinOp"),
        }
    }

    #[test]
    fn test_precedence() {
        let (_, e) = expr("1+2*3").unwrap();
        match e {
            Expr::BinOp { op: BinOpKind::Add, lhs, rhs } => {
                assert_eq!(*lhs, Expr::Number(1));
                match *rhs {
                    Expr::BinOp { op: BinOpKind::Mul, .. } => {}
                    _ => panic!("rhs should be Mul"),
                }
            }
            _ => panic!("expected Add"),
        }
    }

    #[test]
    fn test_parens() {
        let (_, e) = expr("(1+2)*3").unwrap();
        match e {
            Expr::BinOp { op: BinOpKind::Mul, .. } => {}
            _ => panic!("expected Mul at root"),
        }
    }

    #[test]
    fn test_unary() {
        let (_, e) = expr("-5").unwrap();
        match e {
            Expr::UnaryOp { op: UnaryOpKind::Neg, .. } => {}
            _ => panic!("expected Neg"),
        }

        let (_, e) = expr("!flag").unwrap();
        match e {
            Expr::UnaryOp { op: UnaryOpKind::Not, .. } => {}
            _ => panic!("expected Not"),
        }
    }
}
