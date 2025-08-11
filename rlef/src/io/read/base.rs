use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, one_of};
use nom::combinator::{map_res, opt, recognize, value};
use nom::multi::{many0, many1};
use nom::character::complete::space0;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use std::str;
use std::str::FromStr;

use super::LefReadRes;

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> LefReadRes<'a, O> 
where
    F: FnMut(&'a str) -> LefReadRes<'a, O> 
{
    delimited(multispace0, inner, multispace0)
}

// typical string
// ie. abcdef, de234, jkl_mn, ...
pub fn identifier(input: &str) -> LefReadRes<&str> {
    let inner = recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ));
    ws(inner)(input)
}

// parse string that is surrounded by " and ".
// ie, "abc", "def"
pub fn qstring(input: &str) -> LefReadRes<&str> {
    let inner = delimited(char('"'), is_not("\""), char('"'));
    ws(inner)(input)
}

// unsigned integer number
// ie, 100, 350
pub fn unsigned_int(input: &str) -> LefReadRes<u32> {
    let str_parser = recognize(digit1);
    let num_parser = map_res(
        str_parser,
        |res: &str| u32::from_str(res)
    );
    ws(num_parser)(input)
}

// parse signed floating number
// The following is modified from the Python parser by Valentin Lorentz (ProgVal).
pub fn float(input: &str) -> LefReadRes<f64> {
    ws(map_res(
        alt((
            // Case one: 42. and 42.42
            recognize(tuple((opt(char('-')), decimal, char('.'), opt(decimal)))),
            recognize(tuple((opt(char('-')), decimal))), // case two: integer as float number
        )),
        |res: &str| f64::from_str(res),
    ))(input)
}

pub fn decimal(input: &str) -> LefReadRes<&str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

pub fn pt(input: &str) -> LefReadRes<(f64, f64)> {
    separated_pair(float, space0, float)(input)
}

pub fn rect(input: &str) -> LefReadRes<((f64, f64), (f64, f64))> {
    tuple((tuple((float, float)), tuple((float, float))))(input)
}

pub fn pt_list(input: &str) -> LefReadRes<Vec<(f64, f64)>> {
    many1(pt)(input)
}

pub fn lef_comment(input: &str) -> LefReadRes<()> {
    value(
        (),
        preceded(ws(tag("#")), take_until("VERSION"))
    )(input)
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        let res = decimal("012ds").unwrap();
        assert_eq!(res.0, "ds");
        assert_eq!(res.1, "012");
    }

    #[test]
    fn test_float() {
        let res = float("1.2323 hhh").unwrap();
        assert_eq!(res.0, "hhh");
        assert_eq!(res.1, 1.2323);
    }

    #[test]
    fn test_unsigned_int() {
        let res = unsigned_int("1231 hhh").unwrap();
        assert_eq!(res.0, "hhh");
        assert_eq!(res.1, 1231);
    }

    #[test]
    fn test_tstring() {
        let res = identifier("hello world!").unwrap();
        assert_eq!(res.0, "world!");
        assert_eq!(res.1, "hello");
    }

    #[test]
    fn test_qstring() {
        let res = qstring("\"hello world\"").unwrap();
        assert_eq!(res.0, "");
        assert_eq!(res.1, "hello world");
    }
}