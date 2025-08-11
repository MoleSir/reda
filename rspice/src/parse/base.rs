use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, line_ending, not_line_ending, one_of, space0};
use nom::combinator::{map_res, opt, recognize};
use nom::error::VerboseError;
use nom::combinator::map;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::{Err, IResult, Parser};
use std::str;
use std::str::FromStr;

use runit::{Angle, Capacitance, Current, Frequency, Inductance, Number, Resistance, Suffix, Time, Voltage};

pub type NomResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>; 

/// Convert Error to Failure
pub trait ToFailure<T, E> {
    fn to_failure(self) -> Result<T, nom::Err<E>>;
}

impl<T, E> ToFailure<T, E> for Result<T, nom::Err<E>> {
    fn to_failure(self) -> Result<T, nom::Err<E>> {
        self.map_err(|e| match e {
            nom::Err::Error(e) => nom::Err::Failure(e),
            other => other,
        })
    }
}

// /// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
// /// trailing whitespace, returning the output of `inner`.
// pub fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> NomResult<'a, O> 
// where
//     F: FnMut(&'a str) -> NomResult<'a, O> 
// {
//     delimited(multispace0, inner, multispace0)
// }

/// '\n' is not space, but '\n+' is a space!
pub fn smart_space0(input: &str) -> NomResult<()> {
    let mut i = input;
    while !i.is_empty() {
        let bytes = i.as_bytes();
        match bytes[0] {
            b' ' | b'\t' | b'\r' => {
                i = &i[1..];
            }
            b'\n' => {
                if i.len() >= 2 && i.as_bytes()[1] == b'+' {
                    i = &i[2..];
                    while i.starts_with(' ') || i.starts_with('\t') {
                        i = &i[1..];
                    }
                } else {
                    break;
                }
            }
            _ => break,
        }
    }
    Ok((i, ()))
}

pub fn hws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> NomResult<'a, O>
where
    F: FnMut(&'a str) -> NomResult<'a, O>,
{
    delimited(smart_space0, inner, smart_space0)
}

macro_rules! wrap_parser {
    ($parser:expr, $input:expr, $ctx:expr) => {
        match $parser {
            Ok(v) => Ok(v),
            Err(Err::Incomplete(n)) => Err(Err::Incomplete(n)),
            Err(Err::Error(_)) => Err(Err::Error(nom::error::VerboseError {
                errors: [($input, nom::error::VerboseErrorKind::Context($ctx))].into(),
            })),
            Err(Err::Failure(_)) => Err(Err::Failure(nom::error::VerboseError {
                errors: [($input, nom::error::VerboseErrorKind::Context($ctx))].into(),
            })),
        }
    };
}

// typical string
// ie. abcdef, de234, jkl_mn, ...
pub fn identifier(input: &str) -> NomResult<&str> {
    pub fn _identifier(input: &str) -> NomResult<&str> {
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"), tag(".")))),
        ))(input)
    }
    wrap_parser!(_identifier(input), input, "identifier")
}

// node string
// ie. abcdef, de234, jkl_mn, ...
pub fn node(input: &str) -> NomResult<&str> {
    pub fn _node(input: &str) -> NomResult<&str> {
        recognize(many1(alt((alphanumeric1, tag("_"), tag(".")))))(input)
    }
    wrap_parser!(_node(input), input, "node")
}

// unsigned integer number
// ie, 100, 350
pub fn unsigned_int(input: &str) -> NomResult<u32> {
    pub fn _unsigned_int(input: &str) -> NomResult<u32> {
        let str_parser = recognize(digit1);
        map_res(
            str_parser,
            |res: &str| u32::from_str(res)
        )(input)
    }
    wrap_parser!(_unsigned_int(input), input, "unsigned_int")
}

// parse signed floating number
// The following is modified from the Python parser by Valentin Lorentz (ProgVal).
pub fn float(input: &str) -> NomResult<f64> {
    pub fn _float(input: &str) -> NomResult<f64> {
        map_res(
            alt((
                // Case one: 42. and 42.42
                recognize(tuple((opt(char('-')), decimal, char('.'), opt(decimal)))),
                recognize(tuple((opt(char('-')), decimal))), // case two: integer as float number
            )),
            |res: &str| f64::from_str(res),
        )(input)
    }
    wrap_parser!(_float(input), input, "float")
}

fn decimal(input: &str) -> NomResult<&str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

pub fn number(input: &str) -> NomResult<Number> {
    pub fn _number(input: &str) -> NomResult<Number> {
        let (input, value) = float(input)?;
        let (input, suffix) = opt(alt((
            tag_no_case("g").map(|_| Suffix::Mega),
            tag_no_case("meg").map(|_| Suffix::Mega),
            tag_no_case("k").map(|_| Suffix::Kilo),
            tag_no_case("m").map(|_| Suffix::Milli),
            tag_no_case("u").map(|_| Suffix::Micro),
            tag_no_case("n").map(|_| Suffix::Nano),
            tag_no_case("p").map(|_| Suffix::Pico),
        )))(input)?;
    
        Ok((
            input,
            Number {
                value,
                suffix: suffix.unwrap_or(Suffix::None),
            },
        ))
    }
    wrap_parser!(_number(input), input, "expect number")
}

pub fn time_number(input: &str) -> NomResult<Time> {
    pub fn _time_number(input: &str) -> NomResult<Time> {
        let (input, value) = float(input)?;
        let (input, suffix) = opt(alt((
            tag_no_case("gs").map(|_| Suffix::Mega),
            tag_no_case("megs").map(|_| Suffix::Mega),
            tag_no_case("ks").map(|_| Suffix::Kilo),
            tag_no_case("ms").map(|_| Suffix::Milli),
            tag_no_case("us").map(|_| Suffix::Micro),
            tag_no_case("ns").map(|_| Suffix::Nano),
            tag_no_case("ps").map(|_| Suffix::Pico),
            tag_no_case("g").map(|_| Suffix::Mega),
            tag_no_case("meg").map(|_| Suffix::Mega),
            tag_no_case("k").map(|_| Suffix::Kilo),
            tag_no_case("m").map(|_| Suffix::Milli),
            tag_no_case("u").map(|_| Suffix::Micro),
            tag_no_case("n").map(|_| Suffix::Nano),
            tag_no_case("p").map(|_| Suffix::Pico),
            tag_no_case("s").map(|_| Suffix::None),
        )))(input)?;
    
        Ok((
            input,
            Number {
                value,
                suffix: suffix.unwrap_or(Suffix::None),
            }.into(),
        ))
    }
    wrap_parser!(_time_number(input), input, "expect time number")
}

pub fn voltage_number(input: &str) -> NomResult<Voltage> {
    pub fn _voltage_number(input: &str) -> NomResult<Voltage> {
        let (input, value) = float(input)?;
        let (input, suffix) = opt(alt((
            tag_no_case("gv").map(|_| Suffix::Mega),
            tag_no_case("megv").map(|_| Suffix::Mega),
            tag_no_case("kv").map(|_| Suffix::Kilo),
            tag_no_case("mv").map(|_| Suffix::Milli),
            tag_no_case("uv").map(|_| Suffix::Micro),
            tag_no_case("nv").map(|_| Suffix::Nano),
            tag_no_case("pv").map(|_| Suffix::Pico),
            tag_no_case("g").map(|_| Suffix::Mega),
            tag_no_case("meg").map(|_| Suffix::Mega),
            tag_no_case("k").map(|_| Suffix::Kilo),
            tag_no_case("m").map(|_| Suffix::Milli),
            tag_no_case("u").map(|_| Suffix::Micro),
            tag_no_case("n").map(|_| Suffix::Nano),
            tag_no_case("p").map(|_| Suffix::Pico),
            tag_no_case("v").map(|_| Suffix::None),
        )))(input)?;
    
        Ok((
            input,
            Number {
                value,
                suffix: suffix.unwrap_or(Suffix::None),
            }.into(),
        ))
    }
    wrap_parser!(_voltage_number(input), input, "expect voltage number")
}

pub fn current_number(input: &str) -> NomResult<Current> {
    pub fn _current_number(input: &str) -> NomResult<Current> {
        let (input, value) = float(input)?;
        let (input, suffix) = opt(alt((
            tag_no_case("gA").map(|_| Suffix::Mega),
            tag_no_case("megA").map(|_| Suffix::Mega),
            tag_no_case("kA").map(|_| Suffix::Kilo),
            tag_no_case("mA").map(|_| Suffix::Milli),
            tag_no_case("uA").map(|_| Suffix::Micro),
            tag_no_case("nA").map(|_| Suffix::Nano),
            tag_no_case("pA").map(|_| Suffix::Pico),
            tag_no_case("g").map(|_| Suffix::Mega),
            tag_no_case("meg").map(|_| Suffix::Mega),
            tag_no_case("k").map(|_| Suffix::Kilo),
            tag_no_case("m").map(|_| Suffix::Milli),
            tag_no_case("u").map(|_| Suffix::Micro),
            tag_no_case("n").map(|_| Suffix::Nano),
            tag_no_case("p").map(|_| Suffix::Pico),
            tag_no_case("A").map(|_| Suffix::None),
        )))(input)?;
    
        Ok((
            input,
            Number {
                value,
                suffix: suffix.unwrap_or(Suffix::None),
            }.into(),
        ))
    }
    wrap_parser!(_current_number(input), input, "expect current number")
}

pub fn resistance_number(input: &str) -> NomResult<Resistance> {
    pub fn _resistance_number(input: &str) -> NomResult<Resistance> {
        let (input, value) = float(input)?;
        let (input, suffix) = opt(alt((
            tag_no_case("gΩ").map(|_| Suffix::Mega),
            tag_no_case("megΩ").map(|_| Suffix::Mega),
            tag_no_case("kΩ").map(|_| Suffix::Kilo),
            tag_no_case("mΩ").map(|_| Suffix::Milli),
            tag_no_case("uΩ").map(|_| Suffix::Micro),
            tag_no_case("nΩ").map(|_| Suffix::Nano),
            tag_no_case("pΩ").map(|_| Suffix::Pico),
            tag_no_case("g").map(|_| Suffix::Mega),
            tag_no_case("meg").map(|_| Suffix::Mega),
            tag_no_case("k").map(|_| Suffix::Kilo),
            tag_no_case("m").map(|_| Suffix::Milli),
            tag_no_case("u").map(|_| Suffix::Micro),
            tag_no_case("n").map(|_| Suffix::Nano),
            tag_no_case("p").map(|_| Suffix::Pico),
            tag_no_case("Ω").map(|_| Suffix::None),
        )))(input)?;
    
        Ok((
            input,
            Number {
                value,
                suffix: suffix.unwrap_or(Suffix::None),
            }.into(),
        ))
    }
    wrap_parser!(_resistance_number(input), input, "expect resistance number")
}

pub fn capacitance_number(input: &str) -> NomResult<Capacitance> {
    pub fn _capacitance_number(input: &str) -> NomResult<Capacitance> {
        let (input, value) = float(input)?;
        let (input, suffix) = opt(alt((
            tag_no_case("gF").map(|_| Suffix::Mega),
            tag_no_case("megF").map(|_| Suffix::Mega),
            tag_no_case("kF").map(|_| Suffix::Kilo),
            tag_no_case("mF").map(|_| Suffix::Milli),
            tag_no_case("uF").map(|_| Suffix::Micro),
            tag_no_case("nF").map(|_| Suffix::Nano),
            tag_no_case("pF").map(|_| Suffix::Pico),
            tag_no_case("g").map(|_| Suffix::Mega),
            tag_no_case("meg").map(|_| Suffix::Mega),
            tag_no_case("k").map(|_| Suffix::Kilo),
            tag_no_case("m").map(|_| Suffix::Milli),
            tag_no_case("u").map(|_| Suffix::Micro),
            tag_no_case("n").map(|_| Suffix::Nano),
            tag_no_case("p").map(|_| Suffix::Pico),
            tag_no_case("F").map(|_| Suffix::None),
        )))(input)?;
    
        Ok((
            input,
            Number {
                value,
                suffix: suffix.unwrap_or(Suffix::None),
            }.into(),
        ))
    }
    wrap_parser!(_capacitance_number(input), input, "expect capacitance number")
}

pub fn inductance_number(input: &str) -> NomResult<Inductance> {
    pub fn _inductance_number(input: &str) -> NomResult<Inductance> {
        let (input, value) = float(input)?;
        let (input, suffix) = opt(alt((
            tag_no_case("gH").map(|_| Suffix::Mega),
            tag_no_case("megH").map(|_| Suffix::Mega),
            tag_no_case("kH").map(|_| Suffix::Kilo),
            tag_no_case("mH").map(|_| Suffix::Milli),
            tag_no_case("uH").map(|_| Suffix::Micro),
            tag_no_case("nH").map(|_| Suffix::Nano),
            tag_no_case("pH").map(|_| Suffix::Pico),
            tag_no_case("g").map(|_| Suffix::Mega),
            tag_no_case("meg").map(|_| Suffix::Mega),
            tag_no_case("k").map(|_| Suffix::Kilo),
            tag_no_case("m").map(|_| Suffix::Milli),
            tag_no_case("u").map(|_| Suffix::Micro),
            tag_no_case("n").map(|_| Suffix::Nano),
            tag_no_case("p").map(|_| Suffix::Pico),
            tag_no_case("H").map(|_| Suffix::Pico),
        )))(input)?;
    
        Ok((
            input,
            Number {
                value,
                suffix: suffix.unwrap_or(Suffix::None),
            }.into(),
        ))
    }
    wrap_parser!(_inductance_number(input), input, "expect inductance number")
}

pub fn frequency_number(input: &str) -> NomResult<Frequency> {
    let (input, n) = number(input)?;
    Ok((input, n.into()))
}

pub fn angle_number(input: &str) -> NomResult<Angle> {
    let (input, n) = number(input)?;
    Ok((input, n.into()))
}

pub fn comment(input: &str) -> NomResult<&str> {
    pub fn _comment(input: &str) -> NomResult<&str> {
        map(
        terminated(
                preceded(
                    alt((tag("*"), tag(";"))),
                    preceded(space0, not_line_ending),
                ),
                alt((line_ending, nom::combinator::eof)),
            ),
            |text: &str| text.trim_end(),
        )(input)
    }
    wrap_parser!(_comment(input), input, "comment")
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
        assert_eq!(res.0, " hhh");
        assert_eq!(res.1, 1.2323);
    }

    #[test]
    fn test_unsigned_int() {
        let res = unsigned_int("1231 hhh").unwrap();
        assert_eq!(res.0, " hhh");
        assert_eq!(res.1, 1231);
    }

    #[test]
    fn test_tstring() {
        let res = identifier("hello world!").unwrap();
        assert_eq!(res.0, " world!");
        assert_eq!(res.1, "hello");
    }

    #[test]
    fn test_comment() {
        let input = "* this is a comment\r\n.nextline";
        let (rest, c) = comment(input).unwrap();
        assert_eq!(c, "this is a comment");
        assert_eq!(rest, ".nextline");
    
        let input = "; another comment without newline";
        let (rest, c) = comment(input).unwrap();
        assert_eq!(c, "another comment without newline");
        assert_eq!(rest, "");
    }
}