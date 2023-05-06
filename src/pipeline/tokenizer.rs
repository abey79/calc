//! The tokenizer stage.
//!
//! Transforms a `RawInput` into a `TokenizedInput`.

use crate::data::span::{Loc, Span};
use crate::data::token::{Token, TokenKind};
use crate::errors::{SyntaxError, TokenizerError};
use crate::states::{RawInput, TokenContext, TokenizedInput};

type Result<T> = std::result::Result<T, TokenizerError>;

pub(crate) fn tokenize(input: RawInput) -> Result<TokenizedInput> {
    let mut tokenizer = Tokenizer::new(input);
    tokenizer.run()?;
    Ok(TokenizedInput {
        text_ctx: tokenizer.input.text_ctx,
        token_ctx: tokenizer.token_ctx,
    })
}

/// Implement the tokenizer stage.
struct Tokenizer {
    input: RawInput,

    /// current position in the input, updated by [`next()`]
    pos: usize,

    /// current location in the input, updated by [`next()`]
    loc: Loc,

    /// starting location of the current token, updated by the lexer loop
    start_loc: Loc,

    /// current stream of token
    token_ctx: TokenContext,
}

impl Tokenizer {
    fn new(input: RawInput) -> Self {
        Self {
            input,
            pos: 0,
            loc: Loc::default(),
            start_loc: Loc::default(),
            token_ctx: TokenContext::default(),
        }
    }

    fn source(&self) -> &str {
        &self.input.as_ref()
    }

    /// Push a token into the token context.
    fn push(&mut self, kind: TokenKind) {
        let span = Span::new(self.start_loc, self.loc);
        let token = Token::new(kind);
        self.token_ctx.push_token(token, span);
    }

    /// Return the next character in the input stream and update the current location.
    ///
    /// Returns `None` if the end of the input is reached.
    fn next(&mut self) -> Option<char> {
        let c = self.source().chars().nth(self.pos);
        if let Some(c) = c {
            self.pos += 1;
            if c == '\n' {
                self.loc.line += 1;
                self.loc.col = 0;
            } else {
                self.loc.col += 1;
            }
        }

        c
    }

    /// Return the next character in the input stream without updating the current location.
    ///
    /// Returns `None` if the end of the input is reached.
    fn peek(&self) -> Option<char> {
        self.source().chars().nth(self.pos)
    }

    /// Return the next character in the input stream if it matches `c` and update the current
    /// location.
    fn accept(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.next();
            true
        } else {
            false
        }
    }

    fn err<T>(&self, err: SyntaxError) -> Result<T> {
        let span = Span::new(self.loc, self.loc);
        let new_err = TokenizerError::SyntaxError(err, self.input.error_context(span));

        Err(new_err)
    }

    fn run(&mut self) -> Result<()> {
        while let Some(c) = self.next() {
            self.start_loc = self.loc;

            match c {
                // whitespace
                c if c.is_whitespace() => continue,
                // integer/float
                c if c.is_ascii_digit() => {
                    let mut num = c.to_string();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_digit() {
                            num.push(c);
                            self.next();
                        } else {
                            break;
                        }
                    }
                    if let Some(c) = self.peek() {
                        if c == '.' {
                            num.push(c);
                            self.next();
                            while let Some(c) = self.peek() {
                                if c.is_ascii_digit() {
                                    num.push(c);
                                    self.next();
                                } else {
                                    break;
                                }
                            }
                            self.push(TokenKind::Float(num.parse().unwrap()));
                        } else {
                            self.push(TokenKind::Int(num.parse().unwrap()));
                        }
                    } else {
                        self.push(TokenKind::Int(num.parse().unwrap()));
                    }
                }
                // names/keywords
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let mut name = c.to_string();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_alphanumeric() || c == '_' {
                            name.push(c);
                            self.next();
                        } else {
                            break;
                        }
                    }
                    match name.as_str() {
                        // keywords
                        "print" => self.push(TokenKind::Print),
                        _ => self.push(TokenKind::Name(name)),
                    }
                }
                // misc
                ';' => self.push(TokenKind::Semi),
                '(' => self.push(TokenKind::LParen),
                ')' => self.push(TokenKind::RParen),
                '=' => self.push(TokenKind::Assign),
                '+' => self.push(TokenKind::Plus),
                '-' => self.push(TokenKind::Minus),
                '*' => self.push(TokenKind::Star),
                '/' => {
                    if self.accept('/') {
                        while let Some(c) = self.next() {
                            if c == '\n' {
                                break;
                            }
                        }
                    } else if self.accept('*') {
                        while let Some(c) = self.next() {
                            if c == '*' && self.peek() == Some('/') {
                                self.next();
                                break;
                            }
                        }
                    } else {
                        self.push(TokenKind::Slash);
                    }
                }

                c => return self.err(SyntaxError::UnexpectedCharacter(c)),
            }
        }

        Ok(())
    }
}
