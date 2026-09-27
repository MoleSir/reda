use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, line_ending, multispace0, multispace1, not_line_ending, one_of};
use nom::combinator::{map, cut, eof, map_res, opt, recognize, value};
use nom::error::{context, VerboseError, VerboseErrorKind};
use nom::multi::{many0, many1, separated_list0, separated_list1};
use nom::number::streaming::double;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;
use std::str;
use std::str::FromStr;
use crate::model::{LibAst, LibValue};
use crate::error::{LibError, LibResult};

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

pub fn parse_liberty(input: &str) -> LibResult<LibAst> {
    match wrap_unused(group)(input) {
        Ok((_, ast)) => Ok(ast),
        Err(e) => {
            match e {
                nom::Err::Error(err) | nom::Err::Failure(err) => {
                    let msg = pretty_print_error(input, err);
                    Err(LibError::Parse(msg))
                }
                nom::Err::Incomplete(_) => Err(LibError::Parse("incomplete input".into())),
            }
        }
    }
}

pub fn pretty_print_error(input: &str, e: VerboseError<&str>) -> String {
    use std::fmt::Write;
    let mut out = String::new();

    for (i, (substr, kind)) in e.errors.iter().enumerate() {
        // 计算出行号和列号
        let prefix = &input[..(input.len() - substr.len()).min(input.len())];
        let line_num = prefix.lines().count();
        let last_line = prefix.lines().last().unwrap_or("");
        let col_num = last_line.len() + 1;

        match kind {
            VerboseErrorKind::Context(ctx) => {
                writeln!(
                    &mut out,
                    "{i:02}: at line {line_num}, column {col_num}: in context `{ctx}`"
                ).unwrap();
            }
            VerboseErrorKind::Char(c) => {
                writeln!(
                    &mut out,
                    "{i:02}: at line {line_num}, column {col_num}: expected char `{c}`"
                ).unwrap();
            }
            VerboseErrorKind::Nom(kind) => {
                writeln!(
                    &mut out,
                    "{i:02}: at line {line_num}, column {col_num}: nom error: {:?}",
                    kind
                ).unwrap();
            }
        }
    }

    out
}

pub fn ast(input: &str) -> NomResult<'_, LibAst> {
    context(
        "ast",
        alt((
            attr,
            group,
        ))
    )(input)
}

/// 单行注释: // ...
fn line_comment(input: &str) -> NomResult<'_, ()> {
    value(
        (),
        preceded(tag("//"), alt((not_line_ending, eof))),
    )(input)
}

/// 多行注释: /* ... */
fn block_comment(input: &str) -> NomResult<'_, ()> {
    value((), delimited(tag("/*"), take_until("*/"), tag("*/")))(input)
}

#[allow(unused)]
fn comment(input: &str) -> NomResult<'_, &str> {
    context(
        "comment",
        recognize(delimited(tag("/*"), take_until("*/"), tag("*/"))),
    )(input)
}

/// 跳过空白
fn ws0(input: &str) -> NomResult<'_, ()> {
    value(
        (),
        many0(value((), multispace1)),
    )(input)
}

/// 跳过空白或注释
fn unused0(input: &str) -> NomResult<'_, ()> {
    value(
        (),
        many0(alt((
            value((), multispace1), // 至少一个空白，包括换行
            line_comment,
            block_comment,
        ))),
    )(input)
}

#[allow(unused)]
/// 包装 parser，自动跳过两边空白
fn wrap_ws<'a, F: 'a, O>(mut inner: F) -> impl FnMut(&'a str) -> NomResult<'a, O>
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

/// 包装 parser，自动跳过前方的空白
fn pre_ws<'a, F: 'a, O>(mut inner: F) -> impl FnMut(&'a str) -> NomResult<'a, O>
where
    F: FnMut(&'a str) -> NomResult<'a, O>,
{
    move |input: &'a str| {
        let (input, _) = ws0(input)?;
        let (input, res) = inner(input)?;
        Ok((input, res))
    }
}

/// 包装 parser，自动跳过两边空白或注释
fn wrap_unused<'a, F: 'a, O>(mut inner: F) -> impl FnMut(&'a str) -> NomResult<'a, O>
where
    F: FnMut(&'a str) -> NomResult<'a, O>,
{
    move |input: &'a str| {
        let (input, _) = unused0(input)?;
        let (input, res) = inner(input)?;
        let (input, _) = unused0(input)?;
        Ok((input, res))
    }
}

/// 包装 parser，自动跳过前方的空白或注释
fn pre_unused<'a, F: 'a, O>(mut inner: F) -> impl FnMut(&'a str) -> NomResult<'a, O>
where
    F: FnMut(&'a str) -> NomResult<'a, O>,
{
    move |input: &'a str| {
        let (input, _) = unused0(input)?;
        let (input, res) = inner(input)?;
        Ok((input, res))
    }
}

// typical string
// ie. abcdef, de234, jkl_mn, ...
pub fn identifier(input: &str) -> NomResult<'_, &str> {
    context(
        "identifier",
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"), tag(".")))),
        ))
    )(input)
}

/// integer parser, supports sign like +42 or -3
pub fn integer(input: &str) -> NomResult<'_, i32> {
    context(
        "integer",
        map_res(
            tuple((
                opt(one_of("+-")), // optional sign
                digit1,            // digits
            )),
            |(sign, digits): (Option<char>, &str)| {
                i32::from_str(digits).map(|n| match sign {
                    Some('-') => -n,
                    _ => n,
                })
            },
        ),
    )(input)
}

/// signed floating-point number parser
pub fn float(input: &str) -> NomResult<'_, f64> {
    context(
        "float",
        double
    )(input)
}

pub fn boolean(input: &str) -> NomResult<'_, bool> {
    map_res(alpha1, |s: &str| s.parse::<bool>())(input)
}

pub fn quoted_string(input: &str) -> NomResult<'_, &str> {
    context(
        "quoted string",
        preceded(char('\"'), cut(terminated(is_not("\""), char('\"')))),
    )(input)
}

pub fn quoted_floats(input: &str) -> NomResult<'_, Vec<f64>> {
    context(
        "quoted floats",
        delimited(
            pre_unused(char('"')), 
            separated_list1(
                pre_ws(char(',')),
                pre_ws(float)
            ),
            pre_ws(char('"')), 
        )
    )(input)
}

pub fn ast_value(input: &str) -> NomResult<'_, LibValue> {
    context(
        "ast value", 
        alt((
            map(boolean, LibValue::Bool),
            map(float, LibValue::Float),
            map(quoted_string, |s| LibValue::String(s.to_string())),
            map(identifier, |s| LibValue::Ident(s.to_string())),
        ))
    )(input)
}

pub fn ast_values(input: &str) -> NomResult<'_, Vec<LibValue>> {
    context(
        "ast values", 
        delimited(
            pre_unused(char('(')),
            delimited(
                opt(tuple((multispace0, char('\\'), line_ending))),
                separated_list1(
                    alt((
                        map(
                            tuple((multispace0, char(','), multispace0, char('\\'), line_ending)),
                            |_| Some(1),
                        ),
                        map(tuple((multispace0, char(','))), |_| Some(1)),
                        map(
                            tuple((multispace0, char('\\'), line_ending, multispace0, char(','))),
                            |_| Some(1),
                        ),
                    )),
                    preceded(multispace0, ast_value),
                ),
                opt(tuple((multispace0, tag("\\"), line_ending))),
            ),
            pre_unused(char(')')),
        )
    )(input)
}

pub fn simple_attr(input: &str) -> NomResult<'_, LibAst> {
    context(
        "simple attr",
        move |input| {
            let (input, key) = pre_unused(identifier)(input)?;
            let (input, _) = pre_unused(char(':'))(input)?;
            let (input, value) = pre_unused(ast_value)(input).to_failure()?;
            let (input, _) = pre_unused(char(';'))(input)?;

            Ok((input, LibAst::attr(key, value)))
        }
    )(input)
}

pub fn complex_attr(input: &str) -> NomResult<'_, LibAst> {
    context(
        "complex attr",
        map(
            tuple((
                pre_unused(identifier),
                pre_unused(ast_values),
                pre_unused(char(';')),
            )),

            |(name, values, _)| LibAst::attr(name, LibValue::Vec(values))
        )
    )(input)
}

pub fn attr(input: &str) -> NomResult<'_, LibAst> {
    context(
        "attr", 
        alt((simple_attr, complex_attr))
    )(input)
}

pub fn group(input: &str) -> NomResult<'_, LibAst> {
    context(
        "group",
        move |input| {
            let (input, kind) = pre_unused(identifier)(input)?;

            let (input, params) = pre_unused(delimited(
                pre_unused(char('(')),
                separated_list0(
                    pre_unused(tuple((multispace0, char(','), multispace0))),
                    pre_unused(map(alt((identifier, quoted_string)), String::from)),
                ),
                pre_unused(char(')')),
            ))(input)?;

            let (input, _) = pre_unused(char('{'))(input)?;
            let (input, items) = pre_unused(group_items0)(input).to_failure()?;
            let (input, _) = pre_unused(char('}'))(input).to_failure()?;

            Ok((input, LibAst::group(kind, params, items)))
        }
    )(input)
}

pub fn group_items1(input: &str) -> NomResult<'_, Vec<LibAst>> {
    context(
        "group items1",
        many1(pre_unused(ast))
    )(input)
}

pub fn group_items0(input: &str) -> NomResult<'_, Vec<LibAst>> {
    context(
        "group items0",
        many0(pre_unused(ast))
    )(input)
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;

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
        let res = unused0(input);
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

        let mut parser = pre_unused(identifier);
        let res = parser(input);
        assert!(res.is_ok());
        let (remaining, id) = res.unwrap();
        assert_eq!(id, "my_var");
        println!("identifier: {:?}", id);
        println!("left {:?}", remaining);
    }

    #[test]
    fn test_quoted_floats() {
        let input = r#"
            "1.0, 2.0,    3.0"
        "#;
        let mut parser = wrap_unused(quoted_floats);
        let res = parser(input);
        assert!(res.is_ok());
        let (remaining, values) = res.unwrap();
        assert_eq!(values, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_quoted_string() {
        let input = r#"
            "hhh???"
        "#;
        let mut parser = wrap_unused(quoted_string);
        let res = parser(input);
        assert!(res.is_ok());
        let (remaining, values) = res.unwrap();
        assert_eq!(values, "hhh???");
    }


    #[test]
    fn test_ast_values() {
        assert_eq!(
            ast_values(
                r#"( \
    "0, 0.18, 0.33", \
    "-0.555, -0.45, -0.225" \
    )"#
            ),
            Ok((
                "",
                vec![
                    LibValue::String("0, 0.18, 0.33".into()),
                    LibValue::String("-0.555, -0.45, -0.225".into()),
                ]
            ))
        );
        assert_eq!(
            ast_values(
                r#"( \
                 "a string(b)" \
                 )"#
            ),
            Ok(("", vec![LibValue::String("a string(b)".to_string())]))
        );
        assert_eq!(
            ast_values("(123,-456)"),
            Ok(("", vec![LibValue::Float(123.), LibValue::Float(-456.),]))
        );
    }


    #[test]
    fn test_attr() {
        assert_eq!(
            attr("capacitive_load_unit (1,pf);"),
            Ok((
                "",
                LibAst::Attr {
                    key: "capacitive_load_unit".to_string(),
                    value: LibValue::Vec(vec![LibValue::Float(1.), LibValue::Ident("pf".to_string()),]),
                }
            ))
        );
    }

    #[test]
    fn test_attr_multi_line() {
        assert_eq!(
            complex_attr(
                r#"values ( \
    "0, 0.18, 0.33", \
    "-0.555, -0.45, -0.225");"#
            ),
            Ok((
                "",
                LibAst::Attr {
                    key: "values".to_string(),
                    value: LibValue::Vec(vec![
                        LibValue::String("0, 0.18, 0.33".to_string()),
                        LibValue::String("-0.555, -0.45, -0.225".into()),
                    ]),
                }
            ))
        );
    }

    #[test]
    fn test_comment() {
        assert_eq!(
            comment("/*** abc **/def"),
            Ok(("def", "/*** abc **/"))
        );
        assert_eq!(
            comment(
                "/* multi
line
**
**/
**/rest"
            ),
            Ok((
                "
**/rest",
                "/* multi
line
**
**/"
            ))
        );
    }

    #[test]
    fn test_attribute_malformed() {
        assert!(attr("attr_name : a b ; ").is_err());
        println!("{:?}", attr("function		: \"0\";").unwrap());
    }

    #[test]
    fn test_attribute_bool() {
        assert_eq!(
            attr("attr_name : true ; "),
            Ok((
                " ",
                LibAst::Attr { key: String::from("attr_name"), value: LibValue::Bool(true), }
            ))
        );
        assert_eq!(
            attr("attr_name : false ; "),
            Ok((
                " ",
                LibAst::Attr { key: String::from("attr_name"), value: LibValue::Bool(false), }
            ))
        );
    }

    #[test]
    fn test_attribute_float() {
        assert_eq!(
            attr("attr_name : 345.123 ; "),
            Ok((
                " ",
                LibAst::Attr{ key: String::from("attr_name"), value: LibValue::Float(345.123), }
            ))
        );
        assert_eq!(
            attr("attr_name : -345.123 ; "),
            Ok((
                " ",
                LibAst::Attr{ key: String::from("attr_name"), value: LibValue::Float(-345.123),}
            ))
        );
    }


    #[test]
    fn test_parse_group() {
        let data = "library ( foo ) {
            abc ( 1, 2, 3 );
        }";
        match group(data) {
            Err(_) => {
                eprint!("e");
            }
            _ => {}
        };
        assert_eq!(
            group(data),
            Ok((
                "",
                LibAst::LibGroup {
                    kind: "library".to_string(),
                    params: vec!["foo".to_string()],
                    items: vec![LibAst::Attr {
                        key: "abc".to_string(),
                        value: LibValue::Vec(vec![
                            LibValue::Float(1.0), LibValue::Float(2.0), LibValue::Float(3.0),
                        ]),
                    },],
                },
            ))
        );
    }


    #[test]
    fn test_nested_group() {
        assert_eq!(
            group(
                r#"
            outer( outer ) {
                inner ( inner) {
                    abc ( 1, 2, 3 );
                }
                inner(inner2 ) {
                    abc ( 1, 2, 3 );
                }
            }"#
            ),
            Ok((
                "",
                LibAst::group(
                    "outer".to_string(),
                    vec!["outer".to_string()],
                    vec![
                        LibAst::group(
                            "inner".to_string(),
                            vec!["inner".to_string()],
                            vec![LibAst::attr(
                                "abc".to_string(),
                                LibValue::Vec(vec![LibValue::Float(1.0), LibValue::Float(2.0), LibValue::Float(3.0),]),
                            ),],
                        ),
                        LibAst::group(
                            "inner".to_string(),
                            vec!["inner2".to_string()],
                            vec![LibAst::attr(
                                "abc".to_string(),
                                LibValue::Vec(vec![LibValue::Float(1.0), LibValue::Float(2.0), LibValue::Float(3.0),]),
                            ),],
                        ),
                    ]
                )
            ))
        );
    }

    #[test]
    fn test_lib_simple() {
        let asts = parse_liberty(
                r#"
/*
 delay model :       typ
 check model :       typ
 power model :       typ
 capacitance model : typ
 other model :       typ
*/
library(foo) {

  delay_model : table_lookup;
  /* unit attributes */
  time_unit : "1ns";
  capacitive_load_unit (1, pf );
  function: "A & B";

  slew_upper_threshold_pct_rise : 80;
  nom_temperature : 25.0;
}
"#
        ).unwrap();
        assert_eq!(
            asts,
            LibAst::group(
                "library".to_string(),
                vec!["foo".to_string()],
                vec![
                    LibAst::attr(
                        "delay_model".to_string(),
                        LibValue::Ident("table_lookup".to_string())
                    ),
                    LibAst::attr(
                        "time_unit".to_string(),
                        LibValue::String("1ns".to_string())
                    ),
                    LibAst::attr(
                        "capacitive_load_unit".to_string(),
                        LibValue::Vec(vec![LibValue::Float(1.0), LibValue::Ident("pf".to_string()),]),
                    ),
                    LibAst::attr(
                        "function".to_string(),
                        LibValue::String("A & B".to_string()),
                    ),
                    LibAst::attr(
                        "slew_upper_threshold_pct_rise".to_string(),
                        LibValue::Float(80.0)
                    ),
                    LibAst::attr("nom_temperature".to_string(), LibValue::Float(25.0)),
                ],
            ),
        );
    }
}