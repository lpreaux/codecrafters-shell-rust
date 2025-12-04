use crate::parser::token::{FileDescriptor, RedirectMode, Token};
use anyhow::{bail, Result};

pub struct Lexer;

#[derive(PartialEq, Debug)]
enum LexerState {
    Default,
    Escaped,
    SingleQuoted,
    DoubleQuoted,
    EscapedInDoubleQuote,
}

impl Lexer {
    pub fn lex(input: &str) -> Result<Vec<Token>> {
        let mut tokens = Vec::with_capacity(input.len() / 4);
        let mut state = LexerState::Default;
        let mut curr = String::with_capacity(32);

        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            state = match (state, ch) {
                // État Escaped (hors quotes)
                (LexerState::Escaped, ch) => {
                    curr.push(ch);
                    LexerState::Default
                }

                // État EscapedInDoubleQuote
                (LexerState::EscapedInDoubleQuote, ch) => {
                    match ch {
                        '"' | '\\' => {
                            curr.push(ch);
                        }
                        _ => {
                            curr.push('\\');
                            curr.push(ch);
                        }
                    }
                    LexerState::DoubleQuoted
                }

                // Default - Whitespace
                (LexerState::Default, ' ') => {
                    Self::push_word_if_not_empty(&mut tokens, &mut curr);
                    tokens.push(Token::Whitespace);
                    LexerState::Default
                }

                // Default - Backslash
                (LexerState::Default, '\\') => LexerState::Escaped,

                // Default - Single Quote
                (LexerState::Default, '\'') => LexerState::SingleQuoted,

                // Default - Double Quote
                (LexerState::Default, '"') => LexerState::DoubleQuoted,

                // Default - Newline
                (LexerState::Default, '\n') => LexerState::Default,

                // Default - Redirect Operator
                (LexerState::Default, '>') => {
                    let fd = if curr.is_empty() {
                        FileDescriptor::Stdout
                    } else {
                        match FileDescriptor::from_str(&curr) {
                            Ok(fd) => {
                                curr.clear();
                                fd
                            }
                            Err(_) => {
                                // Si c'est UNIQUEMENT un nombre, c'est un FD invalide
                                if curr.chars().all(|c| c.is_ascii_digit()) {
                                    bail!("Bad file descriptor: {}", curr);
                                }
                                // Sinon, c'est un mot normal (ex: "hello3")
                                Self::push_word_if_not_empty(&mut tokens, &mut curr);
                                FileDescriptor::Stdout
                            }
                        }
                    };

                    let mode = if chars.peek() == Some(&'>') {
                        chars.next();
                        RedirectMode::Append
                    } else {
                        RedirectMode::Overwrite
                    };

                    tokens.push(Token::Redirect { mode, fd });
                    LexerState::Default
                }

                // Default - Autre caractère
                (LexerState::Default, ch) => {
                    curr.push(ch);
                    LexerState::Default
                }

                // Single Quoted - Fin de quote
                (LexerState::SingleQuoted, '\'') => {
                    tokens.push(Token::QuotedString(curr.clone(), '\''));
                    curr.clear();
                    LexerState::Default
                }

                // Single Quoted - Autre caractère
                (LexerState::SingleQuoted, ch) => {
                    curr.push(ch);
                    LexerState::SingleQuoted
                }

                // Double Quoted - Fin de quote
                (LexerState::DoubleQuoted, '"') => {
                    tokens.push(Token::QuotedString(curr.clone(), '"'));
                    curr.clear();
                    LexerState::Default
                }

                // Double Quoted - Backslash
                (LexerState::DoubleQuoted, '\\') => LexerState::EscapedInDoubleQuote,

                // Double Quoted - Autre caractère
                (LexerState::DoubleQuoted, ch) => {
                    curr.push(ch);
                    LexerState::DoubleQuoted
                }
            };
        }

        // Vérifications finales
        match state {
            LexerState::SingleQuoted => bail!("Unclosed single quote"),
            LexerState::DoubleQuoted => bail!("Unclosed double quote"),
            LexerState::EscapedInDoubleQuote => {
                bail!("Trailing backslash in double quote")
            }
            LexerState::Escaped => bail!("Trailing backslash"),
            LexerState::Default => {
                Self::push_word_if_not_empty(&mut tokens, &mut curr);
            }
        }

        Ok(tokens)
    }

    #[inline]
    fn push_word_if_not_empty(tokens: &mut Vec<Token>, curr: &mut String) {
        if !curr.is_empty() {
            tokens.push(Token::Word(curr.clone()));
            curr.clear();
        }
    }
}
