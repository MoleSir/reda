use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, digit1, multispace1, not_line_ending, one_of};
use nom::combinator::{eof, map_res, opt, recognize, value};
use nom::error::VerboseError;
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::{Err, IResult};
use std::str;
use std::str::FromStr;

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

/// 单行注释: // ...
fn line_comment(input: &str) -> NomResult<()> {
    value(
        (),
        preceded(tag("//"), alt((not_line_ending, eof))),
    )(input)
}

/// 多行注释: /* ... */
fn block_comment(input: &str) -> NomResult<()> {
    value((), delimited(tag("/*"), take_until("*/"), tag("*/")))(input)
}

/// 跳过空白或注释
pub fn ws0(input: &str) -> NomResult<()> {
    value(
        (),
        many0(alt((
            value((), multispace1), // 至少一个空白，包括换行
            line_comment,
            block_comment,
        ))),
    )(input)
}

/// 包装 parser，自动跳过两边空白或注释
pub fn hws<'a, F: 'a, O>(mut inner: F) -> impl FnMut(&'a str) -> NomResult<'a, O>
where
    F: FnMut(&'a str) -> NomResult<'a, O>,
{
    move |input: &'a str| {
        let (input, _) = ws0(input)?;
        let (input, res) = inner(input)?;
        let (input, _) = ws0(input)?;
        Ok((input, res))
    }
}

// pub fn hws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> NomResult<'a, O>
// where
//     F: FnMut(&'a str) -> NomResult<'a, O>,
// {
//     delimited(space0, inner, space0)
// }

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

pub fn integer(input: &str) -> NomResult<i32> {
    pub fn _integer(input: &str) -> NomResult<i32> {
        let parser = tuple((
            opt(one_of("+-")),   // 符号
            digit1,              // 数字部分
        ));

        map_res(parser, |(sign, digits): (Option<char>, &str)| {
            i32::from_str(digits).map(|n| {
                let n = match sign {
                    Some('-') => -n,
                    _ => n, // None 或 '+'
                };
                n
            })
        })(input)
    }

    wrap_parser!(_integer(input), input, "integer")
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned_int() {
        let res = unsigned_int("1231 hhh").unwrap();
        assert_eq!(res.0, " hhh");
        assert_eq!(res.1, 1231);
    }


    #[test]
    fn test_int() {
        let res = integer("-1231 hhh").unwrap();
        assert_eq!(res.0, " hhh");
        assert_eq!(res.1, -1231);
    }

    #[test]
    fn test_tstring() {
        let res = identifier("hello world!").unwrap();
        assert_eq!(res.0, " world!");
        assert_eq!(res.1, "hello");
    }

    #[test]
    fn test_ws0() {
        let input = r#"
            // 这是单行注释
            /* 这是
               多行注释 */
            abc
        "#;

        // 我们用 ws0 跳过开头空白和注释，然后看看剩余输入
        let res = ws0(input);
        assert!(res.is_ok());
        let (remaining, _) = res.unwrap();

        // 剩下的应该是 "abc\n"
        assert!(remaining.trim_start().starts_with("abc"));
        println!("left: {:?}", remaining);
    }

    #[test]
    fn test_hws_with_identifier() {
        let input = r#"
            // 注释
            /* 多行
               注释 */
            my_var
        "#;

        let mut parser = hws(identifier);
        let res = parser(input);
        assert!(res.is_ok());
        let (remaining, id) = res.unwrap();
        assert_eq!(id, "my_var");
        println!("identifier: {:?}", id);
        println!("left {:?}", remaining);
    }
}