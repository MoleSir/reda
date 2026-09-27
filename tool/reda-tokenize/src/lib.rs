// SPDX-FileCopyrightText: 2023 Thomas Kramer
// SPDX-License-Identifier: GPL-3.0-or-later

//! A simple library for parsing data streams.
//!
//! Parsing is splitted into two tasks
//! * Splitting an iterator into tokens. This is done by a [`Lexer`].
//! * Processing the tokens: The 'Tokenized' struct provides helper functions for processing the stream of tokens.
//!
//! # Example
//! ```
//! use itertools::{Itertools, PeekingNext};
//! use libreda_stream_parser::*;
//!
//! struct ArrayLexer {}
//!
//! impl Lexer for ArrayLexer {
//!     type Char = char;
//!
//!     fn consume_next_token(
//!         &mut self,
//!         input: &mut (impl Iterator<Item = Self::Char> + PeekingNext),
//!         mut output: impl FnMut(Self::Char),
//!     ) -> Result<(), ParserError<char>> {
//!         // Skip whitespace.
//!         let _n = input.peeking_take_while(|c| c.is_whitespace()).count();
//!
//!         let is_terminal_char = |c: char| -> bool {
//!             let terminals = "[],";
//!             c.is_whitespace() || terminals.contains(c)
//!         };
//!
//!         if let Some(c) = input.next() {
//!             output(c);
//!             // Continue reading token if `c` was no terminal character.
//!             if !is_terminal_char(c) {
//!                 input
//!                     .peeking_take_while(|&c| !is_terminal_char(c))
//!                     .for_each(output);
//!             }
//!         }
//!
//!         Ok(())
//!     }
//! }
//!
//! /// Parse an array of the form `[1.0, 2, 3.1324]`.
//! fn parse_array(data: &str) -> Result<Vec<f64>, ParserError<char>> {
//!     let mut tk = tokenize(data.chars(), ArrayLexer {});
//!
//!     tk.advance()?;
//!
//!     let mut arr: Vec<f64> = vec![];
//!
//!     tk.expect_str("[")?;
//!
//!     loop {
//!         if tk.test_str("]")? {
//!             break;
//!         }
//!
//!         let num = tk.take_and_parse()?;
//!         arr.push(num);
//!
//!         tk.expect_str(",")?;
//!     }
//!
//!     Ok(arr)
//! }
//!
//! let data = r#"
//!     [
//!         1.23,
//!         2.34,
//!         3.456,
//!     ]
//! "#;
//!
//! let arr = parse_array(data).expect("parsing failed");
//!
//! assert_eq!(arr, vec![1.23, 2.34, 3.456]);
//! ```

#![deny(missing_docs)]

use std::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::num::ParseIntError;
use std::str::FromStr;

use itertools::PeekingNext;

/// Partition an input stream into tokens.
/// The lexer consumes one token from an input stream in each call of `consume_next_token`.
pub trait Lexer {
    /// Character datatype used by this lexer. Typically, this might be `char` or `u8`.
    type Char;

    /// Consume the next token from the iterator.
    fn consume_next_token(
        &mut self,
        input: &mut (impl Iterator<Item = Self::Char> + PeekingNext),
        output: impl FnMut(Self::Char),
    ) -> Result<(), ParserError<Self::Char>>;
}

/// Provide sequential access to tokens that are created on the fly by
/// splitting characters at whitespace.
pub struct Tokenized<I, L>
where
    I: Iterator,
{
    /// Underlying iterator over characters.
    iter: I,
    /// Tokenizer/lexer.
    lexer: L,
    has_current: bool,
    current_token: Option<Vec<I::Item>>,
}

// TODO: Implementing `StreamingIterator` from `streaming_iterator` could be a good fit.
impl<I, L> Iterator for Tokenized<I, L>
where
    I: Iterator + PeekingNext,
    L: Lexer<Char = I::Item>,
    I::Item: PartialEq + Eq + Clone + Copy + 'static,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_ref().map(|e| e.to_vec())
    }
}

impl<I, L> Tokenized<I, L>
where
    I: Iterator + PeekingNext,
    L: Lexer<Char = I::Item>,
    I::Item: PartialEq + Eq + Clone + Copy + 'static,
{
    /// Go to the next token and return a reference to it, if any.
    pub fn next_ref(&mut self) -> Option<&[I::Item]> {
        self.advance().ok().and_then(|_| self.current_token_ref())
    }

    /// Consume the current token and return it.
    /// Note that the current token is undefined before calling `advance` the first time.
    pub fn take(&mut self) -> Result<Vec<I::Item>, ParserError<I::Item>> {
        let s = self.current_token();
        self.advance()?;
        if let Some(s) = s {
            Ok(s)
        } else {
            Err(ParserError::UnexpectedEndOfFile)
        }
    }

    /// Advance to the next token.
    pub fn advance(&mut self) -> Result<(), ParserError<I::Item>> {
        let mut buffer = self.current_token.take().unwrap_or_default();

        buffer.clear();

        self.lexer
            .consume_next_token(&mut self.iter, |c| buffer.push(c))?;

        let has_next = !buffer.is_empty();

        if has_next {
            self.current_token = Some(buffer);
        }

        self.has_current = has_next;
        Ok(())
    }

    /// Access the current token by reference without consuming it.
    pub fn current_token_ref(&self) -> Option<&[I::Item]> {
        if self.has_current {
            self.current_token.as_deref()
        } else {
            None
        }
    }

    /// Get a clone of the current token without consuming it.
    pub fn current_token(&self) -> Option<Vec<I::Item>> {
        self.current_token_ref().map(|s| s.to_vec())
    }

    /// Test if the current token equals to the expected token.
    /// Returns `Ok(())` if the token matches and advances the iterator.
    /// Returns the actual token otherwise.
    /// Note that the current token is undefined before calling `advance` the first time.
    pub fn expect(
        &mut self,
        s: impl IntoIterator<Item = I::Item> + Clone,
    ) -> Result<(), ParserError<I::Item>> {
        match &self.current_token {
            None => Err(ParserError::UnexpectedEndOfFile)?,
            Some(token) => {
                if token.iter().copied().eq(s.clone()) {
                    self.advance()?;
                    Ok(())
                } else {
                    Err(ParserError::UnexpectedToken(
                        s.into_iter().collect(),
                        self.current_token().unwrap().to_vec(),
                    ))
                }
            }
        }
    }

    /// Test if the current token matches with the string.
    /// The token is consumed only if it matches.
    /// Note that the current token is undefined before calling `advance` the first time.
    pub fn test(&mut self, s: &[I::Item]) -> Result<bool, ParserError<I::Item>> {
        let result = self.peeking_test(s)?;
        if result {
            self.advance()?;
        }
        Ok(result)
    }

    /// Test if the current token matches with the string.
    /// The token is not consumed.
    /// Note that the current token is undefined before calling `advance` the first time.
    pub fn peeking_test(&mut self, s: &[I::Item]) -> Result<bool, ParserError<I::Item>> {
        if self.current_token.is_none() {
            Err(ParserError::UnexpectedEndOfFile)?;
        }

        if self.current_token_ref() == Some(s) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Consume all tokens until and including `s`.
    pub fn skip_until(&mut self, s: &[I::Item]) -> Result<(), ParserError<I::Item>> {
        while !self.test(s)? {
            self.advance()?;
        }
        Ok(())
    }
}
impl<I, L> Tokenized<I, L>
where
    I: Iterator<Item = char> + PeekingNext,
    L: Lexer<Char = I::Item>,
{
    /// Get a clone of the current token without consuming it.
    /// Note that the current token is undefined before calling `advance` the first time.
    pub fn current_token_str(&self) -> Option<String> {
        self.current_token_ref().map(|s| s.iter().collect())
    }

    /// Consume the current token, convert it to a string and return it.
    pub fn take_str(&mut self) -> Result<String, ParserError<I::Item>> {
        let s = self.current_token_str();
        self.advance()?;
        if let Some(s) = s {
            Ok(s)
        } else {
            Err(ParserError::UnexpectedEndOfFile)
        }
    }

    /// Consume a token and try to convert it to `F` it using `FromStr`.
    /// Note that the current token is undefined before calling `advance` the first time.
    pub fn take_and_parse<F: FromStr>(&mut self) -> Result<F, ParserError<I::Item>> {
        let result = if let Some(token) = self.current_token_ref() {
            let string: String = token.iter().collect();

            if let Ok(parsed) = string.parse::<F>() {
                Ok(parsed)
            } else {
                Err(ParserError::InvalidLiteral(token.to_vec()))
            }
        } else {
            Err(ParserError::UnexpectedEndOfFile)
        };

        self.advance()?;

        result
    }

    /// Test if the current token equals to the expected token.
    /// Returns `Ok(())` if the token matches and advances the iterator.
    /// Returns the actual token otherwise.
    /// Note that the current token is undefined before calling `advance` the first time.
    pub fn expect_str(&mut self, s: &str) -> Result<(), ParserError<I::Item>> {
        match &self.current_token {
            None => Err(ParserError::UnexpectedEndOfFile)?,
            Some(token) => {
                if token.iter().copied().eq(s.chars()) {
                    self.advance()?;
                    Ok(())
                } else {
                    Err(ParserError::UnexpectedToken(
                        s.chars().collect(),
                        self.current_token().unwrap().to_vec(),
                    ))
                }
            }
        }
    }

    /// Test if the current token matches with the string.
    /// The token is consumed only if it matches.
    pub fn test_str(&mut self, s: &str) -> Result<bool, ParserError<I::Item>> {
        let result = self.peeking_test_str(s)?;
        if result {
            self.advance()?;
        }
        Ok(result)
    }

    /// Test if the current token matches with the string.
    /// The token is not consumed.
    pub fn peeking_test_str(&mut self, s: &str) -> Result<bool, ParserError<I::Item>> {
        match &self.current_token {
            None => Err(ParserError::UnexpectedEndOfFile)?,
            Some(token) => Ok(token.iter().copied().eq(s.chars())),
        }
    }

    /// Consume all tokens until and including `s`.
    pub fn skip_until_str(&mut self, s: &str) -> Result<(), ParserError<I::Item>> {
        while !self.test_str(s)? {
            self.advance()?;
        }
        Ok(())
    }
}

/// Split a stream of characters into tokens separated by whitespace.
/// Comments are ignored.
pub fn tokenize<I, L>(iter: I, lexer: L) -> Tokenized<Peekable<I>, L>
where
    I: Iterator<Item = char>,
{
    Tokenized {
        iter: iter.peekable(),
        lexer,
        has_current: false,
        current_token: None,
    }
}

/// Error type issued from lexer and parser.
#[derive(Clone, Debug)]
pub enum ParserError<C: 'static> {
    /// Reached end of file before end of library arrived.
    UnexpectedEndOfFile,
    /// Expected and actual token.
    UnexpectedToken(Vec<C>, Vec<C>),
    /// Unknown literal. The literal is given as a string.
    InvalidLiteral(Vec<C>),
    /// Failed to parse an integer.
    ParseIntError(ParseIntError),
}

impl<C: 'static + fmt::Display + fmt::Debug> Error for ParserError<C> {}

impl<C: fmt::Display + fmt::Debug> fmt::Display for ParserError<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedEndOfFile => write!(f, "Unexpected end of file."),
            ParserError::UnexpectedToken(actual, exp) => {
                write!(f, "Unexpected token. '{actual:?}' instead of '{exp:?}'")
            }
            ParserError::InvalidLiteral(n) => write!(f, "Invalid literal: '{n:?}'."),
            ParserError::ParseIntError(e) => write!(f, "Illegal integer: '{e:?}'"),
        }
    }
}

impl<C> From<ParseIntError> for ParserError<C> {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

#[test]
fn test_tokenize_simple() {
    use itertools::Itertools;

    struct MyLexer {}

    impl Lexer for MyLexer {
        type Char = char;

        fn consume_next_token(
            &mut self,
            input: &mut (impl Iterator<Item = Self::Char> + PeekingNext),
            mut output: impl FnMut(Self::Char),
        ) -> Result<(), ParserError<char>> {
            if let Some(c) = input.next() {
                output(c);
                let take_whitespace = c.is_whitespace();

                input
                    .peeking_take_while(|c| c.is_whitespace() == take_whitespace)
                    .for_each(output);
            }

            Ok(())
        }
    }

    let data = "here \n are \t some words  ";

    let mut tk = tokenize(data.chars(), MyLexer {});

    tk.advance().unwrap();
    tk.expect_str("here").unwrap();
    tk.next();
    tk.expect_str("are").unwrap();
    tk.next();
    tk.expect_str("some").unwrap();
    tk.next();
    tk.expect_str("words").unwrap();
}
