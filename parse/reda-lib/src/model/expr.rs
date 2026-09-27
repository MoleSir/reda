use std::str::FromStr;
use std::fmt;
use crate::error::{LibError, LibErrorContext};

#[derive(Debug, Clone, PartialEq)]
pub enum LibExpr {
    Const(bool),
    Var(String),
    Not(Box<LibExpr>),
    And(Vec<LibExpr>),
    Or(Vec<LibExpr>),
    Xor(Vec<LibExpr>),
}

impl FromStr for LibExpr {
    type Err = LibError;
    fn from_str(s: &str) -> Result<Self, LibError> {
        let tokens = tokenize(s).context(|| "tokenize expr")?;
        let mut parser = Parser { tokens, pos: 0 };
        parser.parse_expr().context(|| "parse expr")
    }
}

impl fmt::Display for LibExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LibExpr::Const(v) => {
                if *v { write!(f, "1") } else { write!(f, "0") }
            }
            LibExpr::Var(name) => write!(f, "{}", name),

            LibExpr::Not(expr) => {
                write!(f, "!{}", wrap_if_needed(expr))
            }
            LibExpr::And(exprs) => {
                write!(
                    f,
                    "{}",
                    exprs
                        .iter()
                        .map(|e| wrap_if_needed(e))
                        .collect::<Vec<_>>()
                        .join(" & ")
                )
            }
            LibExpr::Or(exprs) => {
                write!(
                    f,
                    "{}",
                    exprs
                        .iter()
                        .map(|e| wrap_if_needed(e))
                        .collect::<Vec<_>>()
                        .join(" | ")
                )
            }
            LibExpr::Xor(exprs) => {
                write!(
                    f,
                    "{}",
                    exprs
                        .iter()
                        .map(|e| wrap_if_needed(e))
                        .collect::<Vec<_>>()
                        .join(" ^ ")
                )
            }
        }
    }
}

fn wrap_if_needed(expr: &LibExpr) -> String {
    match expr {
        LibExpr::Const(_) | LibExpr::Var(_) => expr.to_string(),
        LibExpr::Not(inner) => format!("!{}", wrap_if_needed(inner)),
        _ => format!("({})", expr),
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Ident(String),
    Not,
    And,
    Or,
    Xor,
    LParen,
    RParen,
}

fn tokenize(s: &str) -> Result<Vec<Token>, LibError> {
    let mut tokens = vec![];
    let mut buf = String::new();
    let mut chars = s.chars().peekable();
    let mut pos = 0;

    while let Some(c) = chars.next() {
        pos += 1;
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                if !buf.is_empty() {
                    tokens.push(Token::Ident(buf.clone()));
                    buf.clear();
                }
            }
            '!' => {
                if !buf.is_empty() {
                    tokens.push(Token::Ident(buf.clone()));
                    buf.clear();
                }
                tokens.push(Token::Not);
            }
            '&' | '*' => {
                if !buf.is_empty() {
                    tokens.push(Token::Ident(buf.clone()));
                    buf.clear();
                }
                tokens.push(Token::And);
            }
            '|' | '+' => {
                if !buf.is_empty() {
                    tokens.push(Token::Ident(buf.clone()));
                    buf.clear();
                }
                tokens.push(Token::Or);
            }
            '^' => {
                if !buf.is_empty() {
                    tokens.push(Token::Ident(buf.clone()));
                    buf.clear();
                }
                tokens.push(Token::Xor);
            }
            '(' => {
                if !buf.is_empty() {
                    tokens.push(Token::Ident(buf.clone()));
                    buf.clear();
                }
                tokens.push(Token::LParen);
            }
            ')' => {
                if !buf.is_empty() {
                    tokens.push(Token::Ident(buf.clone()));
                    buf.clear();
                }
                tokens.push(Token::RParen);
            }
            _ if c.is_alphanumeric() || c == '_' => {
                buf.push(c);
            }
            _ => {
                // 出现非法字符，立即报错
                return Err(LibError::ParseExpr(format!(
                    "invalid character '{}' at position {} in expression '{}'",
                    c, pos, s
                )));
            }
        }
    }

    if !buf.is_empty() {
        tokens.push(Token::Ident(buf));
    }

    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}


impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
            Some(self.tokens[self.pos - 1].clone())
        } else {
            None
        }
    }

    fn parse_expr(&mut self) -> Result<LibExpr, LibError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<LibExpr, LibError> {
        let mut left = self.parse_xor()?;
        while let Some(Token::Or) = self.peek() {
            self.next();
            let right = self.parse_xor()?;
            left = LibExpr::Or(vec![left, right]);
        }
        Ok(left)
    }

    fn parse_xor(&mut self) -> Result<LibExpr, LibError> {
        let mut left = self.parse_and()?;
        while let Some(Token::Xor) = self.peek() {
            self.next();
            let right = self.parse_and()?;
            left = LibExpr::Xor(vec![left, right]);
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<LibExpr, LibError> {
        let mut left = self.parse_unary()?;
        while let Some(Token::And) = self.peek() {
            self.next();
            let right = self.parse_unary()?;
            left = LibExpr::And(vec![left, right]);
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<LibExpr, LibError> {
        if let Some(Token::Not) = self.peek() {
            self.next();
            let expr = self.parse_unary()?;
            Ok(LibExpr::Not(Box::new(expr)))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<LibExpr, LibError> {
        match self.next() {
            Some(Token::Ident(name)) => match name.as_str() {
                "0" | "FALSE" | "false" => Ok(LibExpr::Const(false)),
                "1" | "TRUE" | "true" => Ok(LibExpr::Const(true)),
                _ => Ok(LibExpr::Var(name)),
            },
            Some(Token::LParen) => {
                let expr = self.parse_expr()?;
                match self.next() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err(LibError::ParseExpr("missing ')'".into())),
                }
            }
            Some(tok) => Err(LibError::ParseExpr(format!("unexpected token: {:?}", tok))),
            None => Err(LibError::ParseExpr("unexpected end".into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_and() {
        let expr = "A & B";
        let tokens = tokenize(expr).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Ident("A".into()),
                Token::And,
                Token::Ident("B".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_not_and_or() {
        let expr = "!A & (B | C)";
        let tokens = tokenize(expr).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Not,
                Token::Ident("A".into()),
                Token::And,
                Token::LParen,
                Token::Ident("B".into()),
                Token::Or,
                Token::Ident("C".into()),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_tokenize_xor_expression() {
        let expr = "A ^ B";
        let tokens = tokenize(expr).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Ident("A".into()),
                Token::Xor,
                Token::Ident("B".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_with_constants() {
        let expr = "1";
        let tokens = tokenize(expr).unwrap();
        assert_eq!(tokens, vec![Token::Ident("1".into())]);

        let expr = "0";
        let tokens = tokenize(expr).unwrap();
        assert_eq!(tokens, vec![Token::Ident("0".into())]);
    }

    #[test]
    fn test_tokenize_with_underscore_names() {
        let expr = "A_1 & B_2";
        let tokens = tokenize(expr).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Ident("A_1".into()),
                Token::And,
                Token::Ident("B_2".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_invalid_character() {
        let expr = "A @ B";
        let err = tokenize(expr).unwrap_err();
        assert!(err.to_string().contains("@"));
    }

    #[test]
    fn test_tokenize_multiple_spaces() {
        let expr = "  A   &   B  ";
        let tokens = tokenize(expr).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Ident("A".into()),
                Token::And,
                Token::Ident("B".into()),
            ]
        );
    }


    fn parse_ok(s: &str) -> LibExpr {
        LibExpr::from_str(s).unwrap()
    }

    fn parse_err(s: &str) -> String {
        LibExpr::from_str(s)
            .unwrap_err()
            .to_string()
    }

    #[test]
    fn test_const_and_var() {
        assert_eq!(parse_ok("1"), LibExpr::Const(true));
        assert_eq!(parse_ok("true"), LibExpr::Const(true));
        assert_eq!(parse_ok("0"), LibExpr::Const(false));
        assert_eq!(parse_ok("FALSE"), LibExpr::Const(false));
        assert_eq!(parse_ok("A"), LibExpr::Var("A".into()));
        assert_eq!(parse_ok("foo_bar"), LibExpr::Var("foo_bar".into()));
    }

    #[test]
    fn test_not_expr() {
        assert_eq!(
            parse_ok("!A"),
            LibExpr::Not(Box::new(LibExpr::Var("A".into())))
        );
        assert_eq!(
            parse_ok("!1"),
            LibExpr::Not(Box::new(LibExpr::Const(true)))
        );
    }

    #[test]
    fn test_and_expr() {
        let expr = parse_ok("A & B");
        assert_eq!(
            expr,
            LibExpr::And(vec![
                LibExpr::Var("A".into()),
                LibExpr::Var("B".into())
            ])
        );
    }

    #[test]
    fn test_or_expr() {
        let expr = parse_ok("A | B");
        assert_eq!(
            expr,
            LibExpr::Or(vec![
                LibExpr::Var("A".into()),
                LibExpr::Var("B".into())
            ])
        );
    }

    #[test]
    fn test_xor_expr() {
        let expr = parse_ok("A ^ B");
        // 注意：你的代码中 XOR 实际上调用了 `Or`（应该修复为 Xor）
        // 这里只是保持一致
        assert_eq!(
            expr,
            LibExpr::Xor(vec![
                LibExpr::Var("A".into()),
                LibExpr::Var("B".into())
            ])
        );
    }

    #[test]
    fn test_nested_expr() {
        let expr = parse_ok("A & (B | !C)");
        assert_eq!(
            expr,
            LibExpr::And(vec![
                LibExpr::Var("A".into()),
                LibExpr::Or(vec![
                    LibExpr::Var("B".into()),
                    LibExpr::Not(Box::new(LibExpr::Var("C".into())))
                ])
            ])
        );
    }

    #[test]
    fn test_precedence() {
        let expr = parse_ok("A & B | C");
        assert_eq!(
            expr,
            LibExpr::Or(vec![
                LibExpr::And(vec![
                    LibExpr::Var("A".into()),
                    LibExpr::Var("B".into())
                ]),
                LibExpr::Var("C".into())
            ])
        );
    }

    #[test]
    fn test_invalid_char() {
        let err = parse_err("A $ B");
        assert!(
            err.contains("invalid character"),
            "unexpected error: {}",
            err
        );
    }

    #[test]
    fn test_missing_paren() {
        let err = parse_err("(A & B");
        assert!(err.contains("missing ')'"));
    }

    #[test]
    fn test_empty_input() {
        let err = parse_err("");
        assert!(err.contains("unexpected end"));
    }

    #[test]
    fn test_to_string() {
        let expr = LibExpr::And(vec![
            LibExpr::Var("A".into()),
            LibExpr::Or(vec![
                LibExpr::Var("B".into()),
                LibExpr::Not(Box::new(LibExpr::Var("C".into()))),
            ]),
        ]);
    
        assert_eq!(expr.to_string(), "A & (B | !C)");
    
        let expr2 = LibExpr::Or(vec![
            LibExpr::And(vec![
                LibExpr::Var("A".into()),
                LibExpr::Var("B".into()),
            ]),
            LibExpr::Var("C".into()),
        ]);
        assert_eq!(expr2.to_string(), "(A & B) | C");
    }
}
